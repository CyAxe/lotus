use lotus::Lotus;
mod args;
mod logger;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let cmd_opts = args::cmd_args();
    if cmd_opts.is_present("log") == true {
        logger::init_log(cmd_opts.value_of("log").unwrap()).unwrap();
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


