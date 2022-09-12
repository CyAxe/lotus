mod core;
use futures::{stream, StreamExt};
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, BufRead};
use std::path::Path;

pub struct Lotus {
    script: String,
}

impl Lotus {
    pub fn init(script: String) -> Self {
        Lotus { script }
    }

    pub async fn start(&self, threads: usize, output_path: &str) {
        let stdin = io::stdin();
        let urls = stdin
            .lock()
            .lines()
            .map(|x| x.unwrap().to_string())
            .collect::<Vec<String>>();

        let urls = urls.iter().map(|url| url.as_str()).collect::<Vec<&str>>();

        let active = self.get_scripts("active");
        let passive = self.get_scripts("passive");

        // ProgressBar Settings
        let bar = ProgressBar::new(urls.len() as u64 * active.len() as u64 * passive.len() as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}").expect("ProgressBar Error")
            .tick_chars(format!("{}", "⣾⣽⣻⢿⡿⣟⣯⣷").as_str())
            .progress_chars("#>-"));

        let lualoader = core::LuaLoader::new(output_path.to_string());
        let scan = stream::iter(urls.into_iter()).for_each_concurrent(threads, |url| {
            let active = active.clone();
            let bar = bar.clone();
            let lualoader = lualoader.clone();
            async move {
                stream::iter(active.into_iter())
                    .for_each_concurrent(15, |(script_out, script_name)| {
                        let bar = bar.clone();
                        let lualoader = lualoader.clone();
                        log::debug!("RUNNING {} on {}", script_name,url);
                        async move {
                            lualoader.run_scan(&bar, &script_out, &url).await.unwrap();
                            log::debug!("FINISHED {} on {}", script_name,url);
                        }
                    })
                    .await;
            }
        });
        scan.await;
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
