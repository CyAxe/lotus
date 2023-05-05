use crate::CliErrors;
use log::{debug, error};
use std::fs::File;
use std::io::prelude::Write;
use std::path::PathBuf;

pub const CVE_EXAMPLE: &str = include_str!("lua_examples/cve.lua");
pub const FUZZ_EXAMPLE: &str = include_str!("lua_examples/fuzz.lua");
pub const SERVICE_EXAMPLE: &str = include_str!("lua_examples/service.lua");

pub fn write_file(file_name: PathBuf, content_str: &'static str) -> Result<(), CliErrors> {
    debug!("Trying to check the {} path", file_name.to_str().unwrap());
    if file_name.exists() {
        error!("File exists, ignore ...");
        Err(CliErrors::FileExists)
    } else {
        debug!("Trying to opening the file");
        match File::create(file_name) {
            Ok(mut created_file) => {
                debug!("File opened, trying to write content");
                let write_file = created_file.write_all(content_str.as_bytes());
                if write_file.is_ok() {
                    debug!("Success");
                    Ok(())
                } else {
                    error!("Cannot write in this file");
                    Err(CliErrors::WritingError)
                }
            }
            Err(err) => {
                error!("Opening File Error {}", err);
                Err(CliErrors::WritingError)
            }
        }
    }
}
