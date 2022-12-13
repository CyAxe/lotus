mod threads;

use crate::RequestOpts;
use mlua::Lua;
use crate::lua_api::{get_utilsfunc, get_matching_func, http_func, payloads_func, encoding_func};
use crate::network::http::Sender;
use std::sync::{Arc, Mutex};
use thirtyfour::prelude::*;

#[derive(Clone)]
pub struct LuaLoader<'a> {
    output_dir: String,
    request: RequestOpts,
    bar: &'a indicatif::ProgressBar,
}

/// Start Lotus by adding the ProgressBar and http request options
impl<'a> LuaLoader<'a> {
    pub fn new(
        bar: &'a indicatif::ProgressBar,
        request: RequestOpts,
        output_dir: String,
    ) -> LuaLoader {
        LuaLoader {
            output_dir,
            request,
            bar,
        }
    }

    fn set_lua(&self, target_url: &str, lua: &Lua,  driver: Option<Arc<Mutex<WebDriver>>> ) {
        // Adding Lotus Lua Function
        get_utilsfunc(self.bar, &lua);
        get_matching_func(&lua);
        http_func(target_url, &lua);
        encoding_func(&lua);
        payloads_func(&lua);
        // HTTP Sender
        lua.globals()
            .set(
                "http",
                Sender::init(
                    self.request.headers.clone(),
                    self.request.proxy.clone(),
                    self.request.timeout,
                    self.request.redirects,
                ),
            )
            .unwrap();
        if !driver.is_none() {
            lua.globals()
                .set(
                    "openbrowser",
                    lua.create_function(move |_, url: String| {
                        futures::executor::block_on({
                            let driver = Arc::clone(driver.as_ref().unwrap());
                            async move {
                                driver.lock().unwrap().goto(url).await.unwrap();
                            }
                        });
                        Ok(())
                    })
                    .unwrap(),
                )
                .unwrap();
        }
    }
    pub async fn run_scan(&self, target_url: &str, driver: Option<Arc<Mutex<WebDriver>>>, script_code: &str,script_dir: &str) {
        let lua = Lua::new();
        // settings lua api
        self.set_lua(target_url,&lua, driver);
        lua.globals().set("SCRIPT_PATH", script_dir).unwrap();


        // Handle this error please
       lua.load(script_code).exec_async().await.unwrap();
       let main_func = lua.globals().get::<_, mlua::Function>("main").unwrap();
       main_func.call_async::<_, mlua::Value>(target_url)
                .await
                .unwrap();

    }
}
