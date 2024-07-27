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
        default_value_t = false,
        action = ArgAction::SetTrue
    )]
    /// optimized extraction flag
    pub flag: bool,

    #[arg(
        short = 'n',
        long = "n_equiv_exprs",
        required = false,
        default_value_t = 10,
        value_parser = check_n_equiv_exprs
    )]
    /// number of equivalent expressions
    pub n_equiv_exprs: u8,

    #[arg(
        short = 'l',
        long = "init_token_limit",
        required = false,
        default_value_t = 8,
        value_parser = check_token_limit
    )]
    /// initial token limit
    pub init_token_limit: u8,

    #[arg(
        short = 'm',
        long = "max_token_limit",
        required = false,
        default_value_t = 12,
        value_parser = check_token_limit
    )]
    /// maximum token limit
    pub max_token_limit: u8,

    #[arg(
        short = 't',
        long = "time_limit",
        required = false,
        default_value_t = 350,
        value_parser = check_time_limit,
    )]
    /// initial time limit
    pub init_time_limit: u16,

    #[arg(
        short = 'e',
        long = "input_expr",
        required = false,
        required_unless_present_all = &["input_filepath", "output_filepath"],
        conflicts_with_all = &["input_filepath", "output_filepath"]
    )]
    /// input expression
    pub input_expr: Option<String>,

    #[arg(
        short = 'i',
        long = "input_filepath",
        required = false,
        required_unless_present = "input_expr",
        requires = "output_filepath",
        conflicts_with = "input_expr"
    )]
    /// input filepath
    pub input_filepath: Option<String>,

    #[arg(
        short = 'o',
        long = "output_filepath",
        required = false,
        required_unless_present = "input_expr",
        requires = "input_filepath",
        conflicts_with = "input_expr",
    )]
    /// output filepath
    pub output_filepath: Option<String>,
}

#[derive(Clone, Debug)]
/// Argument datatype struct
pub enum CliDtype {
    /// boolean
    Bool(bool),
    /// unsigned integer 8-bit
    UInt8(u8),
    /// unsigned integer 16-bit
    UInt16(u16),
    /// floating point 64-bit
    Float(f64),
    /// string
    String(String),
}

impl CliDtype {
    /// public function to convert member variables
    /// in struct ArgDtype to type String
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `String`
    pub fn to_string(&self) -> String {
        match self {
            CliDtype::Bool(value) => value.to_string(),
            CliDtype::UInt8(value) => value.to_string(),
            CliDtype::UInt16(value) => value.to_string(),
            CliDtype::Float(value) => value.to_string(),
            CliDtype::String(value) => value.clone(),
        }
    }

    /// ### public function to convert String variables to type ArgDtype
    /// #### Argument
    /// * `s` - &str type variable
    /// #### Return
    /// * `Result` - whether conversion is successfully or not
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if let Ok(value) = s.parse::<bool>() {
            return Ok(CliDtype::Bool(value));
        }
        if let Ok(value) = s.parse::<u8>() {
            return Ok(CliDtype::UInt8(value));
        }
        if let Ok(value) = s.parse::<u16>() {
            return Ok(CliDtype::UInt16(value));
        }
        if let Ok(value) = s.parse::<f64>() {
            return Ok(CliDtype::Float(value));
        }
        Ok(CliDtype::String(s.to_string()))
    }
}

/// ### private function to check if user's input for number of equivalent expressions variable
/// ### n_equiv_exprs is valid
/// #### Argument
/// * `s` - user's input
/// #### Return
/// * `Result` valid u8 input, or error message
fn check_n_equiv_exprs(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(n_equiv_exprs) => { return Ok(n_equiv_exprs); },
        Err(_) => {
            return Err(format!("\n[ERROR]: Invalid value '{}' for number of equivalent expressions, expect u8.", s));
        },
    };
}

/// ### private function to check if user's input for token limit variable
/// ### token_limit is valid
/// #### Argument
/// * `s` - user's input
/// #### Return
/// * `Result` valid u8 input, or error message
fn check_token_limit(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(init_token_limit) => {
            if init_token_limit > 0 && init_token_limit <= u8::MAX {
                return Ok(init_token_limit);
            } else {
                return Err(format!("\n[ERROR]: Invalid input value '{}' for token limit, expect u8 in range (0, 2^8].", s));
            }
        },
        Err(_) => {
            return Err(format!("\n[ERROR]: Invalid value '{}' for token limit, expect u8.", s));
        },
    };
}

/// ### private function to check if user's input for initial time limit variable
/// ### init_time_limit is valid
/// #### Argument
/// * `s` - user's input
/// #### Return
/// * `Result` valid u16 input, or error message
fn check_time_limit(s: &str) -> Result<u16, String> {
    match s.parse::<u16>() {
        Ok(time_limit) => {
            if time_limit > 0 && time_limit <= u16::MAX {
                return Ok(time_limit);
            } else {
                return Err(format!("\n[ERROR]: Invalid input value '{}' for time limit, expect to u16 in range (0, 2^16].", s));
            }
        },
        Err(_) => {
            return Err(format!("\n[ERROR]: Invalid value '{}' for time limit, expect u16.", s));
        },
    };
}

/// ### private function to print command line input help information
/// #### Argument
/// * `None`
/// #### Return
/// * `None`
pub fn help() {
    log_info_raw("[USAGE]: cargo run [-f] <optim ext flag>   [-n] <n equiv exprs>\n");
    log_info_raw("[USAGE]:           [-l] <init token limit> [-m] <max token limit>\n");
    log_info_raw("[USAGE]:           [-t] <init time limit>  [-e] <expr>\n");
    log_info_raw("[USAGE]:           [-i] <input filepath> & [-o] <output filepath>\n");
    log_info_raw("[USAGE]:\n");
    log_info_raw("[USAGE]: <optimized flag>   -> optimized extraction flag\n");
    log_info_raw("[USAGE]:  true              -> run optimized extraction\n");
    log_info_raw("[USAGE]:  false             -> run exhaustive extraction\n");
    log_info_raw("[USAGE]:  datatype          -> bool\n");
    log_info_raw("[USAGE]:  default            = false\n");
    log_info_raw("[USAGE]:  required          -> false\n");
    log_info_raw("[USAGE]: <n equiv exprs>    -> number of equivalent expressions\n");
    log_info_raw("[USAGE]:  datatype          -> uint8\n");
    log_info_raw("[USAGE]:  default            = 10\n");
    log_info_raw("[USAGE]:  required          -> false\n");
    log_info_raw("[USAGE]: <init token limit> -> initial tokens limit\n");
    log_info_raw("[USAGE]:  datatype          -> uint8\n");
    log_info_raw("[USAGE]:  default            = 8\n");
    log_info_raw("[USAGE]:  required          -> false\n");
    log_info_raw("[USAGE]: <max token limit>  -> maximum tokens limit\n");
    log_info_raw("[USAGE]:  datatype          -> uint8\n");
    log_info_raw("[USAGE]:  default            = 12\n");
    log_info_raw("[USAGE]:  required          -> false\n");
    log_info_raw("[USAGE]: <init time limit>  -> initial time limit in sec\n");
    log_info_raw("[USAGE]:  datatype          -> uint16\n");
    log_info_raw("[USAGE]:  default            = 350\n");
    log_info_raw("[USAGE]:  required          -> false\n");
    log_info_raw("[USAGE]: <expr>             -> initial expression\n");
    log_info_raw("[USAGE]:  datatype          -> String\n");
    log_info_raw("[USAGE]:  default            = None\n");
    log_info_raw("[USAGE]:  required          -> True if [-i] & [-o] not provided\n");
    log_info_raw("[USAGE]: <input filepath>   -> input expressions filepath\n");
    log_info_raw("[USAGE]:  type              -> String\n");
    log_info_raw("[USAGE]:  default            = None\n");
    log_info_raw("[USAGE]:  required          -> True if [-e] not provided\n");
    log_info_raw("[USAGE]: <output filepath>  -> output expressions filepath\n");
    log_info_raw("[USAGE]:  type              -> String\n");
    log_info_raw("[USAGE]:  default            = None\n");
    log_info_raw("[USAGE]:  required          -> True if [-e] not provided\n");
}

/// ### public function to parse command line input(s)
/// #### Argument
/// * `None`
/// #### Return
/// * `None` - command line input(s)
pub fn parse_args() -> Vec<CliDtype> {
    let cli: Cli = match Cli::try_parse() {
        Ok(cli) => { cli },
        Err(e) => {
            log_error(&format!("Error encountered while trying to parse command line input(s).\n"));
            log_error(&format!("{}\n", e));
            help();
            exit(1);
        },
    };

    if cli.max_token_limit < cli.init_token_limit {
        log_error(&format!("Maximum token limit {} needs to be ≥ initial token limit {}.\n", cli.max_token_limit, cli.init_token_limit));
        exit(1);
    }

    let mut cli_dtype: Vec<CliDtype> = vec![CliDtype::Bool(cli.flag),
                                            CliDtype::UInt8(cli.n_equiv_exprs),
                                            CliDtype::UInt8(cli.init_token_limit),
                                            CliDtype::UInt8(cli.max_token_limit),
                                            CliDtype::UInt16(cli.init_time_limit),];

    match cli.input_expr {
        Some(input_expr) => {
            cli_dtype.push(CliDtype::String(input_expr));
            return cli_dtype;
        },
        None => { },
    };
    match cli.input_filepath {
        Some(input_fpath) => { cli_dtype.push(CliDtype::String(input_fpath)); },
        None => { },
    };
    match cli.output_filepath {
        Some(output_fpath) => { cli_dtype.push(CliDtype::String(output_fpath)); },
        None => { },
    };

    return cli_dtype;
}
