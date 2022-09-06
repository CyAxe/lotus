use glob::glob;
use std::path::Path;
use super::files::filename_to_string;

pub fn get_scripts(scripts_path: &str ,script_type: &str) -> Vec<(String, String)> {
    let mut scripts = Vec::new();
    for entry in glob(
        format!(
            "{}{}",
            Path::new(scripts_path).join(script_type).to_str().unwrap(),
            "/*.lua"
        )
        .as_str(),
    )
    .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => scripts.push((
                filename_to_string(path.to_str().unwrap()).unwrap(),
                path.file_name().unwrap().to_str().unwrap().to_string(),
            )),
            Err(e) => log::error!("{:?}", e),
        }
    }
    scripts
}

#[macro_export]
macro_rules! is_file {
    ($file_path:expr) => {{
        use std::path;
        println!("aboba {}",$file_path);
        true
    }};
}
