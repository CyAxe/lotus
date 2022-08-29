mod core;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn start(urls: Vec<String>) {
    let tester_idk = core::LuaLoader::new();
    let threader = rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build()
        .unwrap();
    threader.install(|| {
        urls.par_iter().for_each(|url| {
            tester_idk.load_auth(url.to_string());
        });
    });
}
