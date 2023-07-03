pub mod selenium_lua;
use lazy_static::lazy_static;
use std::sync::Arc;
use thirtyfour::WebDriver;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref SELENIUM_DRIVERS_LIMIT: Arc<Mutex<i32>> = Arc::new(Mutex::new(5));
    pub static ref LIVE_SELENIUM_DRIVERS: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    pub static ref LIVE_DRIVERS: Arc<Mutex<Vec<WebDriver>>> = Arc::new(Mutex::new(Vec::new()));
}
