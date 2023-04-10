use crate::cli::errors::CliErrors;
use std::path::PathBuf;
use structopt::StructOpt;


fn get_script_type(script_type: &str) -> Result<ScriptType, CliErrors> {
    let script_type = match script_type {
        "fuzz" => ScriptType::Fuzz,
        "cve" => ScriptType::CVE,
        "passive" => ScriptType::PASSIVE,
        "service" => ScriptType::SERVICE,
        _ => ScriptType::NotSupported,
    };
    if script_type == ScriptType::NotSupported {
        Err(CliErrors::UnsupportedScript)
    } else {
        Ok(script_type)
    }
}




#[derive(Debug, PartialEq)]
pub enum ScriptType {
    Fuzz,
    CVE,
    PASSIVE,
    SERVICE,
    NotSupported,
}



#[derive(Debug, StructOpt)]
pub struct NewOpts {
    #[structopt(short = "-s", long, parse(try_from_str = get_script_type))]
    pub scan_type: ScriptType,
    #[structopt(short = "f", long)]
    pub file_name: PathBuf,
}

