use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliErrors {
    #[error("There is no URLS in the Stdin ")]
    EmptyStdin,
    #[error("File not found")]
    ReadingError,
    #[error("RegexError")]
    RegexError,
    #[error("RegexPatternError")]
    RegexPatternError,
}
