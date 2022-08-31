mod core;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use log::debug;

pub struct Lottas {
    urls: Vec<String>,
    script: String,
}

impl Lottas {
    pub fn init(urls: Vec<String>, script: String) -> Self {
        debug!("INIT THE Lottas Config");
        Lottas { urls, script }
    }

    pub fn start(&self, threads: usize) {
        let active = self.get_scripts("active");
        let bar = ProgressBar::new(self.urls.len() as u64 * active.len() as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}").expect("ProgressBar Error")
            .tick_chars(format!("{}", "⣾⣽⣻⢿⡿⣟⣯⣷").as_str())
            .progress_chars("#>-"));
        let lualoader = core::LuaLoader::new(bar);
        let threader = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        threader.install(|| {
            self.urls.par_iter().for_each(|url| {
                // PARSED CUSTTOM PARAMETER
                active.iter().for_each(|(script_out, _script_name)| {
                    lualoader.run_scan(&script_out, url);
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
                Err(e) => log::error!("{:?}", e),
            }
        }
        scripts
    }
}
