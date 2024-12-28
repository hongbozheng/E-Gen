use crate::*;
use clap::{ArgAction, Parser};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(
    about = "Equivalent Expressions Generation",
    long_about = None,
    version = "0.0.1",
    author = "Hongbo Zheng",
)]
/// Command line inputs
pub struct Cli {
    #[arg(
        short = 'f',
        long = "flag",
        required = false,
        requires = "op",
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    /// extra operator flag
    pub flag: bool,

    #[arg(
        short = 'o',
        long = "operator",
        required = false,
        requires = "flag"
    )]
    /// operator
    pub op: Option<String>,

    #[arg(
        short = 'i',
        long = "input_filepath",
        required = true,
        requires = "ref_filepath"
    )]
    /// input filepath
    pub input_filepath: String,

    #[arg(
        short = 'r',
        long = "ref_filepath",
        required = true,
        requires = "input_filepath"
    )]
    /// refactor file
    pub ref_filepath: String,
}

/// ### private function to print command line input help information
/// #### Argument
/// * `None`
/// #### Return
/// * `None`
pub fn help() {
    log_info_raw("[USAGE]: cargo run [-f] <operator flag>   [-o] <operator>\n");
    log_info_raw("[USAGE]:           [-i] <input filepath>  [-r] <refactor filepath>\n");
    log_info_raw("[USAGE]:\n");
    log_info_raw("[USAGE]: <operator flag>     -> operator flag\n");
    log_info_raw("[USAGE]:  false              -> no  extra operator\n");
    log_info_raw("[USAGE]:  true               -> add extra operator\n");
    log_info_raw("[USAGE]:  datatype           -> bool\n");
    log_info_raw("[USAGE]:  default             = false\n");
    log_info_raw("[USAGE]:  required           -> true if [-o] provided\n");
    log_info_raw("[USAGE]: <operator>          -> operator\n");
    log_info_raw("[USAGE]:  datatype           -> String\n");
    log_info_raw("[USAGE]:  required           -> True if [-f] provided\n");
    log_info_raw("[USAGE]: <input filepath>    -> input filepath\n");
    log_info_raw("[USAGE]:  datatype           -> String\n");
    log_info_raw("[USAGE]:  required           -> true\n");
    log_info_raw("[USAGE]: <refactor filepath> -> refactor filepath\n");
    log_info_raw("[USAGE]:  datatype           -> String\n");
    log_info_raw("[USAGE]:  required           -> true\n");
}

/// ### public function to parse command line input(s)
/// #### Argument
/// * `cli` - command line input(s)
/// #### Return
/// * `None`
pub fn parse_args() -> Cli {
    let cli: Cli = match Cli::try_parse() {
        Ok(cli) => { cli },
        Err(e) => {
            log_error(&format!("Error encountered while trying to parse command line input(s).\n"));
            log_error_raw(&format!("{}\n", e));
            help();
            exit(1);
        },
    };

    return cli;
}
