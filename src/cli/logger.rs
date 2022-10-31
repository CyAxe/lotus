// Lotus init logger
pub fn init_log(log_path: &str) -> Result<(), std::io::Error> {
    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("isahc", log::LevelFilter::Warn);
        // Disalbe unwanted loggers
    logger
        .chain(fern::log_file(log_path).unwrap())
        .apply()
        .unwrap();
    Ok(())
}
