mod core;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn start() {
    let tester_idk = core::LuaLoader::new();
    let urls = vec![
        "http://testphp.vulnweb.com/listproducts.php?cat=1'"
    ];
    let threader = rayon::ThreadPoolBuilder::new()
        .num_threads(20)
        .build()
        .unwrap();
    urls.par_iter().for_each(|url| println!("URL: {:?}", url));
    threader.install(|| {
        urls.par_iter().for_each(|url| {
            tester_idk.load_auth(url.to_string());
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
