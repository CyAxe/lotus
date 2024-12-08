use crate::cli::{
    args::new::ScriptType,
    default_scripts::{write_file, CVE_EXAMPLE, FUZZ_EXAMPLE, SERVICE_EXAMPLE},
    errors::CliErrors,
};
use std::path::PathBuf;

pub fn new_args(scan_type: ScriptType, file_name: PathBuf) {
    let script_code = match scan_type {
        ScriptType::Fuzz => FUZZ_EXAMPLE,
        ScriptType::CVE => CVE_EXAMPLE,
        ScriptType::SERVICE => SERVICE_EXAMPLE,
        ScriptType::NotSupported => "",
    };
    let write_script_file = write_file(file_name, script_code);
    if let Err(CliErrors::FileExists) = write_script_file {
        log::error!(
            "File Exists, cannot overwrite it, please rename/remove it or try another name"
        );
    } else if let Err(CliErrors::WritingError) = write_script_file {
        log::error!("{}", CliErrors::WritingError.to_string().as_str(),);
    } else {
        log::error!("A copy of the Example file has been created",);
    }
    log::info!("Exit ..");
}
