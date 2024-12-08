pub mod scan;
use scan::UrlsOpts;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Lotus",
    about = "Fast Web Security Scanner written in Rust based on Lua Scripts"
)]
pub enum Opts {
    #[structopt(about = "Use CVE, VULN scripts to scan the given URLs")]
    SCAN(UrlsOpts),
}
