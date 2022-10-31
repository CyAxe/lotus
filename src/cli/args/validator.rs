use std::path::Path;

pub(crate) fn file_exists(file_path: &str) -> Result<(), String> {
    match Path::new(file_path).exists() {
        true => Ok(()),
        false => Err(format!("the lua Report File doesnt exists: {}", file_path)),
    }
}
