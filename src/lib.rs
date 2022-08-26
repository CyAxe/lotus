mod core;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn start() {
    let tester_idk = core::LuaLoader::new();
    let urls = vec!["http://testphp.vulnweb.com/listproducts.php?cat=1'"];
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
