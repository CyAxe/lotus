mod core;
use crate::core::utils::bar::create_progress;
use futures::{stream, StreamExt};
use glob::glob;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;

pub struct Lotus {
    script: String,
}

impl Lotus {
    pub fn init(script: String) -> Self {
        Lotus { script }
    }

    pub async fn start(&self, threads: usize, script_threads: usize, output_path: &str) {
        let stdin = io::stdin();
        let urls = stdin
            .lock()
            .lines()
            .map(|x| x.unwrap().to_string())
            .collect::<Vec<String>>();

        let urls = urls.iter().map(|url| url.as_str()).collect::<Vec<&str>>();
        let active = self.get_scripts();

        // ProgressBar Settings
        let bar = create_progress(urls.len() as u64 * active.len() as u64);


        let lualoader = Arc::new(core::LuaLoader::new(&bar, output_path.to_string()));
        stream::iter(urls.into_iter())
            .map(move |url| {
                let active = active.clone();
                let lualoader = Arc::clone(&lualoader);
                let script_path = &self.script;
                stream::iter(active.into_iter())
                    .map(move |(_script_out, script_name)| {
                        log::debug!("RUNNING {} on {}", script_name, url);
                        let lualoader = Arc::clone(&lualoader);
                        async move { 
                            lualoader.run_scan(None, &_script_out,&script_path, url).await.unwrap() 
                        }
                    })
                    .buffer_unordered(script_threads)
                    .collect::<Vec<_>>()
            })
            .buffer_unordered(threads)
            .collect::<Vec<_>>()
            .await;
    }

    fn get_scripts(&self) -> Vec<(String, String)> {
        let mut scripts = Vec::new();
        for entry in glob(
            format!(
                "{}{}",
                Path::new(&self.script).to_str().unwrap(),
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
