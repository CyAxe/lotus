use lotus::Lotus;
mod args;
mod logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let current_subcommand = args::cmd_args()
        .subcommand_name()
        .unwrap_or("urls")
        .to_string();
    if args::cmd_args()
        .subcommand_matches(&current_subcommand)
        .unwrap()
        .is_present("log")
        == true
    {
        logger::init_log(
            args::cmd_args()
                .subcommand_matches(&current_subcommand)
                .unwrap()
                .value_of("log")
                .unwrap(),
        )
        .unwrap();
    }
    let lottas = Lotus::init(
        args::cmd_args()
            .subcommand_matches(&current_subcommand)
            .unwrap()
            .value_of("scripts")
            .unwrap()
            .to_string(),
    );
    lottas
        .start(
            args::cmd_args()
                .subcommand_matches(&current_subcommand)
                .unwrap()
                .value_of("workers")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            args::cmd_args()
                .subcommand_matches(&current_subcommand)
                .unwrap()
                .value_of("script_threads")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap(),
            &args::cmd_args()
                .subcommand_matches(&current_subcommand)
                .unwrap()
                .value_of("output")
                .unwrap(),
        )
        .await;
    Ok(())
}
