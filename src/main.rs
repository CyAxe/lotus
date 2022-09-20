use clap::{App, Arg, ArgMatches};
use lotus::Lotus;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cmd_opts = cmd_args();
    if cmd_opts.is_present("log") == true {
        init_log(cmd_opts.value_of("log").unwrap()).unwrap();
    }
    let lua_code = cmd_opts.value_of("scripts").unwrap();
    let lottas = Lotus::init(lua_code.to_string());
    lottas
        .start(
            cmd_opts
                .value_of("workers")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            cmd_opts
                .value_of("script_threads")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            &cmd_opts.value_of("output").unwrap(),
        )
        .await;
    Ok(())
}

pub fn cmd_args() -> ArgMatches {
    App::new("Lotus")
        .version("0.2-beta")
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
            Arg::with_name("log")
                .help("Save all lots to custom file")
                .takes_value(true)
                .short('l')
                .long("log")
            )

        .arg(
            Arg::with_name("script_threads")
                .help("Workers for lua scripts")
                .short('t')
                .long("script-threads")
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("scripts")
                .help("Path of scripts dir")
                .takes_value(true)
                .short('s')
                .long("scripts")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Path of the JSON output fiel")
                .required(true)
                .takes_value(true)
                .long("output")
                .short('o'),
        )
        .arg(Arg::with_name("nolog").help("no logging"))
        .get_matches()
}

fn init_log(log_path: &str) -> Result<(), std::io::Error> {
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
    logger.chain(fern::log_file(log_path).unwrap()).apply().unwrap();
    Ok(())
}
