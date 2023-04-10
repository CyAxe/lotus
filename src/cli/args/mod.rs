pub mod scan;
pub mod new;
use structopt::StructOpt;
use scan::UrlsOpts;
use new::NewOpts;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Lotus",
    about = "Fast Web Security Scanner written in Rust based on Lua Scripts"
)]
pub enum Opts {
    #[structopt(about = "Create a lua example code based on the type of scan")]
    NEW(NewOpts),
    #[structopt(about = "Use CVE, VULN scripts to scan the given URLs")]
    URLS(UrlsOpts),
}
