use lottas::Lottas;
use std::io::{self, BufRead};
use structopt::StructOpt;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), std::io::Error> {
    let cmd_opts = Opt::from_args();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .pretty()
        .with_thread_ids(true)
        .with_ansi(true)
        .with_thread_names(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let lua_code = cmd_opts.scripts;
    let lottas = Lottas::init(lines.map(|x| x.unwrap()).collect(), lua_code.to_string());
    lottas.start();
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "lottas",
    about = "Fast Web Security Scanner written in Rust with Lua Support to make DAST process Faster "
)]

pub struct Opt {
    #[structopt(
        short = "w",
        long = "workers",
        default_value = "30",
        help = "number of workers"
    )]
    pub scripts_threads: usize,
    #[structopt(
        long = "scripts-workers",
        default_value = "2",
        help = "How many script to run in the same time"
    )]
    pub scripts_worker: usize,
    #[structopt(
        short = "t",
        long = "timeout",
        default_value = "10",
        help = "connection timeout"
    )]
    pub timeout: usize,
    #[structopt(short = "s", long = "scripts", help = "Path of Scripts dir")]
    pub scripts: String,
}
