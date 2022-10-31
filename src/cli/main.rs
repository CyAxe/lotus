use lotus::Lotus;
use lotus::RequestOpts;
mod args;
mod logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if args::cmd_args().is_present("log") == true {
        logger::init_log(args::cmd_args().value_of("log").unwrap()).unwrap();
    }
    let lottas = Lotus::init(args::cmd_args().value_of("scripts").unwrap().to_string());
    let request_opts = RequestOpts {
        proxy: match args::cmd_args().value_of("proxy") {
            Some(proxy) => Some(proxy.to_string()),
            None => None,
        },
        timeout: args::cmd_args()
            .value_of("timeout")
            .unwrap()
            .to_string()
            .parse::<u64>()
            .unwrap(),
        redirects: args::cmd_args()
            .value_of("redirects")
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap(),
    };
    lottas
        .start(
            args::cmd_args()
                .value_of("workers")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            request_opts,
            args::cmd_args()
                .value_of("script_threads")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            &args::cmd_args().value_of("output").unwrap_or(""),
            &args::cmd_args().value_of("out_script").unwrap_or(""),
        )
        .await;
    Ok(())
}
