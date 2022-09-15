use lotus::Lotus;
use clap::{App, Arg, ArgMatches};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    init_log().unwrap();
    let cmd_opts = cmd_args();
    let lua_code = cmd_opts.value_of("scripts").unwrap();
    let lottas = Lotus::init(lua_code.to_string());
    lottas
        .start(
            cmd_opts.value_of("workers").unwrap().trim().parse::<usize>().unwrap(),
            cmd_opts.value_of("script_threads").unwrap().trim().parse::<usize>().unwrap(),
            &cmd_opts.value_of("output").unwrap(),
        )
        .await;
    Ok(())
}


pub fn cmd_args() -> ArgMatches {
    App::new("Lotus")
        .version("0.1-beta")
        .author("Khaled Nassar <knassar702@gmail.com>")
        .about("Fast Web Security Scanner written in Rust based on Lua Scripts ")
        .arg(
            Arg::with_name("workers")
                .help("Number of works of urls")
                .required(true)
                .short('w')
                .takes_value(true)
                .default_value("10")
                .long("workers"),
        )
        .arg(
            Arg::with_name("script_threads")
            .help("Workers for lua scripts")
            .short('t')
            .long("script-threads")
            .takes_value(true)
            .default_value("5")
            )

        .arg(
            Arg::with_name("scripts")
            .help("Path of scripts dir")
            .takes_value(true)
            .short('s')
            .long("scripts")
            .required(true)
            )

        .arg(
            Arg::with_name("output")
            .help("Path of the JSON output fiel")
            .required(true)
            .takes_value(true)
            .long("output")
            .short('o')
            )
        .arg(
            Arg::with_name("nolog")
                .help("no logging")
            )
        .get_matches()
}

fn init_log() -> Result<(), std::io::Error> {
    let no_log = true;
    let log_file = match home::home_dir() {
        Some(path) => fern::log_file(path.join("lotus.log").to_str().unwrap()).unwrap(),
        None => {
            eprintln!("Impossible to get your home dir!");
            fern::log_file("lotus.log").unwrap()
        }
    };
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
    if no_log == true {
        logger.apply().unwrap();
    } else {
        logger.chain(log_file).apply().unwrap();
    }
    Ok(())
}
