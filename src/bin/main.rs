use lotus::Lottas;
use std::io::{self, BufRead};
use structopt::StructOpt;

fn main() -> Result<(), std::io::Error> {
    let cmd_opts = Opt::from_args();
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
        .chain(fern::log_file("output.log")?)
        .apply()
        .unwrap();
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lua_code = cmd_opts.scripts;
    let lottas = Lottas::init(lines.map(|x| x.unwrap()).collect(), lua_code.to_string());
    lottas.start(cmd_opts.threads, &cmd_opts.json_output);
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "lotus",
    about = "Fast Web Security Scanner written in Rust with Lua Support to make DAST process Faster "
)]

pub struct Opt {
    #[structopt(
        short = "w",
        long = "workers",
        default_value = "30",
        help = "number of workers"
    )]
    pub threads: usize,
    #[structopt(
        short = "t",
        long = "timeout",
        default_value = "10",
        help = "connection timeout"
    )]
    pub timeout: usize,
    #[structopt(short = "s", long = "scripts", help = "Path of Scripts dir")]
    pub scripts: String,
    #[structopt(short = "o", long = "output", help = "Path of output JSON file")]
    pub json_output: String,
}
