use crate::lua::network::selenium_driver::{LIVE_SELENIUM_DRIVERS, SELENIUM_DRIVERS_LIMIT};
use mlua::ExternalError;
use mlua::UserData;
use std::sync::Arc;
use tealr::TypeName;
use thirtyfour::prelude::*;
use thirtyfour::WebDriver;
use tokio::sync::Mutex;

#[derive(TypeName)]
pub struct Selenium {
    pub driver: SeleniumStatus,
}

#[derive(Clone)]
pub enum SeleniumStatus {
    Ready(Arc<Mutex<WebDriver>>),
    NotReady,
}
impl Selenium {
    pub async fn default() -> Self {
        let limit_lock = SELENIUM_DRIVERS_LIMIT.lock().await;
        let mut live_lock = LIVE_SELENIUM_DRIVERS.lock().await;
        if *limit_lock == *live_lock {
            log::debug!("Not ready yet");
            Selenium {
                driver: SeleniumStatus::NotReady,
            }
        } else {
            *live_lock += 1;
            log::debug!("Create a new client");
            Selenium {
                driver: SeleniumStatus::Ready(Arc::new(Mutex::new(
                    WebDriver::new("http://localhost:9515", {
                        let mut caps = DesiredCapabilities::chrome();
                        caps.set_binary("/usr/bin/brave-browser-stable").unwrap();
                        caps
                    })
                    .await
                    .unwrap(),
                ))),
            }
        }
    }
    pub async fn get_ready(&mut self) -> bool {
        match self.driver {
            SeleniumStatus::Ready(..) => return true,
            SeleniumStatus::NotReady => {
                *self = Selenium::default().await;
                return false
            }
        }
    }

}


impl std::clone::Clone for Selenium {
    fn clone(&self) -> Self {
        Selenium {
            driver: self.driver.clone(),
        }
    }
}

impl UserData for Selenium {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("many_live", |_, _ , ()| async move {
            Ok(*LIVE_SELENIUM_DRIVERS.lock().await)
        });
        methods.add_async_method("get_ready", |_, mut this, ()| async move {
            Ok(match this.get_ready().await {
                true => true,
                false => this.get_ready().await
            })
        });
        // Update the `go` method
        methods.add_async_method("go", |_, this, url: String| async move {
            match &this.driver {
                SeleniumStatus::Ready(the_driver) => {
                    println!("GOING TO {}", url);
                    let driver_lock = the_driver.lock().await;
                    let driver = &*driver_lock;
                    driver.goto(url).await.unwrap();
                    Ok(true)
                }
                SeleniumStatus::NotReady => Ok(false),
            }
        });

        // Update the `source` method
        methods.add_async_method("source", |_, this, ()| async move {
            match &this.driver {
                SeleniumStatus::Ready(the_driver) => {
                    let driver_lock = the_driver.lock().await;
                    let driver = &*driver_lock;
                    Ok(driver.source().await.unwrap())
                }
                SeleniumStatus::NotReady => Err("Selenium isn't ready yet".to_lua_err()),
            }
        });

        // Update the `exit` method
        methods.add_async_method("exit", |_, this, ()| async move {
            match &this.driver {
                SeleniumStatus::Ready(the_driver) => {
                    let driver_lock = the_driver.lock().await;
                    let driver = &*driver_lock;
                    driver.close_window().await.unwrap();
                    log::debug!("Window has been closed");

                    let mut live_selenium = LIVE_SELENIUM_DRIVERS.lock().await;
                    *live_selenium -= 1;
                    println!("NUMBERE {}",live_selenium);

                    Ok(true)
                }
                SeleniumStatus::NotReady => Err("Selenium isn't ready yet".to_lua_err()),
            }
        });
    }
}
