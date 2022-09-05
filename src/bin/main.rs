use lotus::Lotus;
use std::io::{self, BufRead};
use structopt::StructOpt;

fn main() -> Result<(), std::io::Error> {
    init_log().unwrap();
    let cmd_opts = Opt::from_args();
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lua_code = cmd_opts.scripts;
    let lottas = Lotus::init(lua_code.to_string());
    lottas.start(
        cmd_opts.threads,
        lines.map(|x| x.unwrap().to_string()).collect(),
        cmd_opts.json_output,
    );
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "lotus",
    about = "Fast Web Security Scanner written in Rust based on Lua Scripts "
)]

struct Opt {
    #[structopt(
        short = "w",
        long = "workers",
        default_value = "30",
        help = "number of workers"
    )]
    threads: usize,
    #[structopt(short = "s", long = "scripts", help = "Path of Scripts dir")]
    scripts: String,
    #[structopt(short = "o", long = "output", help = "Path of output JSON file")]
    json_output: String,
}

fn init_log() -> Result<(), std::io::Error> {
    let log_file = match home::home_dir() {
                Some(path) => fern::log_file(path.join("lotus.log").to_str().unwrap()).unwrap(),
                None => { 
                    eprintln!("Impossible to get your home dir!");
                    fern::log_file("lotus.log").unwrap()
                },
            };
    fern::Dispatch::new()
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
        .chain(log_file)
        .apply()
        .unwrap();
    Ok(())
}
