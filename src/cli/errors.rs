use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliErrors {
    #[error("There is no URLS in the Stdin ")]
    EmptyStdin,
    #[error("File not found")]
    ReadingError,
    #[error("RegexError")]
    RegexError,
    #[error("Cannot change the file content")]
    WritingError,
    #[error("File Exists already")]
    FileExists,
    #[error("RegexPatternError")]
    RegexPatternError,
    #[error("Unsupported script type")]
    UnsupportedScript,
}


#[derive(Error, Debug)]
pub enum Network {
    #[error("Connection Timeout")]
    ConnectionTimeout,
}