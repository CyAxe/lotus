use log::debug;
use std::fs::File;
use std::io::{self, Read};

pub fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    debug!("READING {:?}", s);
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
