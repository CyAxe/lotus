mod core;
use glob::glob;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
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

    pub fn start(&self,threads: usize) {
        let active = self.get_scripts("active");
        let lualoader = core::LuaLoader::new();
        let threader = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        threader.install(|| {
            self.urls.par_iter().for_each(|url| {
                // PARSED CUSTTOM PARAMETER
                active.iter().for_each(|(script_out, _script_name)| {
                    lualoader.run_scan(
                        &script_out,
                        url,
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
                    core::utils::files::filename_to_string(path.to_str().unwrap()).unwrap(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                )),
                Err(e) => tracing::error!("{:?}", e),
            }
        }
        scripts
    }
}
