mod core;
use glob::glob;
use reqwest::Url;
use std::path::Path;
use tracing::debug;

pub struct Lottas {
    urls: Vec<String>,
    script: String,
}

impl Lottas {
    pub fn init(urls: Vec<String>, script: String) -> Self {
        debug!("INIT THE Lottas Config");
        Lottas { urls, script }
    }

    pub fn start(&self) {
        let active = self.get_scripts("active");
        let lualoader = core::LuaLoader::new();
        self.urls.iter().for_each(|url| {
            let parsed_url = Url::parse(url).unwrap();
            // PARSED CUSTTOM PARAMETER
            parsed_url.query_pairs().into_iter().for_each(|url_param| {
                active.iter().for_each(|(script_path, script_name)| {
                    let script_out = core::utils::files::filename_to_string(&script_path);
                    lualoader.run_scan(
                        &script_out.unwrap(),
                        (url, url_param.0.to_string().as_str()),
                    );
                });
            });
        });
    }

    fn get_scripts(&self, script_type: &str) -> Vec<(String, String)> {
        let mut scripts = Vec::new();
        for entry in glob(
            format!(
                "{}{}",
                Path::new(&self.script).join(script_type).to_str().unwrap(),
                "/*.lua"
            )
            .as_str(),
        )
        .expect("Failed to read glob pattern")
        {
            match entry {
                Ok(path) => scripts.push((
                    path.to_str().unwrap().to_string(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                )),
                Err(e) => tracing::error!("{:?}", e),
            }
        }
        scripts
    }
}
