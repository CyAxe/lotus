use std::collections::HashMap;
use std::path::Path;

pub(crate) fn file_exists(file_path: &str) -> Result<(), String> {
    match Path::new(file_path).exists() {
        true => Ok(()),
        false => Err(format!("the lua Report File doesnt exists: {}", file_path)),
    }
}

pub(crate) fn valid_json(json_value: &str) -> Result<(), String> {
    match serde_json::from_str::<HashMap<String, String>>(json_value) {
        Ok(_json_data) => Ok(()),
        Err(_err) => Err("Headers Value is not a Valid Json data".to_string()),
    }
}
