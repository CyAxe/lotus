
#[cfg(test)]
mod tests {
    pub use lotus::utils::args::{parse_timeout, parse_proxy, Cli};
    use structopt::StructOpt;
    use std::path::PathBuf;

    #[test]
    fn test_parse_timeout_valid() {
        assert_eq!(parse_timeout("10").unwrap(), 10);
        assert_eq!(parse_timeout("0").unwrap(), 0);
        assert!(parse_timeout("999999").is_ok());
    }

    #[test]
    fn test_parse_timeout_invalid() {
        assert!(parse_timeout("-10").is_err());
        assert!(parse_timeout("abc").is_err());
        assert!(parse_timeout("").is_err());
    }

    #[test]
    fn test_parse_proxy_valid() {
        assert_eq!(
            parse_proxy("http://example.com").unwrap(),
            "http://example.com"
        );
        assert_eq!(
            parse_proxy("https://example.com").unwrap(),
            "https://example.com"
        );
    }

    #[test]
    fn test_parse_proxy_invalid() {
        assert!(parse_proxy("ftp://example.com").is_err());
        assert!(parse_proxy("example.com").is_err());
        assert!(parse_proxy("").is_err());
    }

    #[test]
    fn test_cli_struct() {
        let args = vec![
            "lotus",                  // program name
            "--timeout", "15",        // timeout
            "--proxy", "http://proxy.test", // proxy
            "--output", "output.log", // output file
        ];
        let cli = Cli::from_iter_safe(&args).unwrap();

        assert_eq!(cli.timeout, 15);
        assert_eq!(cli.proxy.unwrap(), "http://proxy.test");
        assert_eq!(cli.output, PathBuf::from("output.log"));
    }

    #[test]
    fn test_cli_struct_defaults() {
        let args = vec![
            "lotus",                  // program name
            "--output", "output.log", // output file
        ];
        let cli = Cli::from_iter_safe(&args).unwrap();

        assert_eq!(cli.timeout, 30); // default timeout
        assert!(cli.proxy.is_none());
        assert_eq!(cli.output, PathBuf::from("output.log"));
    }
}
