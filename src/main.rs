mod utils;
use utils::term::{
    logger::init_logger,
    bar::{ProgressManager, GLOBAL_PROGRESS_BAR}
};
use utils::net::requester;
use std::thread;
use std::time::Duration;
use lotus::lua::engine;


#[tokio::main]
async fn main() {
    let progress_manager = ProgressManager::new(100, "Initializing...");
    let req = requester::Requester::new(requester::RequestOptions::default()).unwrap();
    init_logger(progress_manager.progress_bar);

    log_info!("Starting the process...");
    log_warn!("This is a warning message.");
    log_error!("An error occurred during execution.");

    if let Some(ref pb) = *GLOBAL_PROGRESS_BAR.lock().unwrap() {
        for i in 0..=100 {
            pb.set_message(format!("Processing item {}", i));
            engine::tester();
            pb.inc(1);
            //thread::sleep(Duration::from_millis(50));
        }
        pb.finish_with_message("Process completed successfully.");
    }

    log_info!("Process completed.");
}
