mod core;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::Path;
use std::sync::Arc;

pub struct Lotus {
    script: String,
}

impl Lotus {
    pub fn init(script: String) -> Self {
        debug!("INIT THE Lottas Config");
        Lotus { script }
    }

    pub fn start(&self, threads: usize, urls: Vec<String>, output_path: String) {
        let urls = Arc::new(urls);
        let active = self.get_scripts("active");
        let passive = self.get_scripts("passive");
        let bar = ProgressBar::new(urls.len() as u64 * active.len() as u64 * passive.len() as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}").expect("ProgressBar Error")
            .tick_chars(format!("{}", "⣾⣽⣻⢿⡿⣟⣯⣷").as_str())
            .progress_chars("#>-"));
        let lualoader = core::LuaLoader::new();
        let threader = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        threader.install(move || {
            urls.par_iter().for_each(|url| {
                active.iter().for_each(|(script_out, _script_name)| {
                    let _ = lualoader
                        .run_scan(&bar, &output_path, &script_out, url)
                        .unwrap();
                    bar.inc(1);
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
