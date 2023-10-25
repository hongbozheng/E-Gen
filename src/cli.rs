use crate::*;
use std::process::exit;

#[derive(Clone, Debug, PartialEq)]
/// Command line arguments struct
pub enum CmdLineArg {
    /// exhaustive extraction flag
    Bool(bool),
    /// maximum expression rewrite length
    UInt(u8),
    /// percentage of max # of OS threads
    Float(f64),
    /// expressions || input filename & output filename
    String(String),
}

impl CmdLineArg {
    /// public function to convert member variables
    /// in struct CmdLineArg to type String
    /// ## Argument
    /// * `self`
    /// ## Return
    /// * `String`
    pub fn to_string(&self) -> String {
        match self {
            CmdLineArg::Bool(value) => value.to_string(),
            CmdLineArg::UInt(value) => value.to_string(),
            CmdLineArg::Float(value) => value.to_string(),
            CmdLineArg::String(value) => value.clone(),
        }
    }

    /// ### public function to convert String variables to type CmdLineArg
    /// #### Argument
    /// * `s` - &str type variable
    /// #### Return
    /// * `Result` - whether conversion is successfully or not
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if let Ok(value) = s.parse::<bool>() {
            return Ok(CmdLineArg::Bool(value));
        }
        if let Ok(value) = s.parse::<u8>() {
            return Ok(CmdLineArg::UInt(value));
        }
        if let Ok(value) = s.parse::<f64>() {
            return Ok(CmdLineArg::Float(value));
        }
        Ok(CmdLineArg::String(s.to_string()))
    }
}

/// ### private function to print command line input help information
/// #### Argument
/// * `None`
/// #### Return
/// * `None`
fn help() {
    log_info_raw("[USAGE]: cargo run [-t] <thd pct> [-n] <num tokens>     [-f] <exhaustive flag>\n");
    log_info_raw("[USAGE]:           [-e] <expr>    [-i] <input filepath> [-o] <output filepath>\n");
    log_info_raw("[USAGE]: <thd pct>         -> OS thread percentage\n");
    log_info_raw("[USAGE]:  type             -> float64\n");
    log_info_raw("[USAGE]:  default           = 1.0 [100%]\n");
    log_info_raw("[USAGE]:  required         -> false\n");
    log_info_raw("[USAGE]: <num tokens>      -> number of tokens limit\n");
    log_info_raw("[USAGE]:  type             -> uint8\n");
    log_info_raw("[USAGE]:  default           = 8\n");
    log_info_raw("[USAGE]:  required         -> false\n");
    log_info_raw("[USAGE]: <exhaustive flag> -> exhaustive extraction flag\n");
    log_info_raw("[USAGE]:  type             -> uint8\n");
    log_info_raw("[USAGE]:  default           = 0\n");
    log_info_raw("[USAGE]:  required         -> false\n");
    log_info_raw("[USAGE]:  0 -> false, run optimized extraction\n");
    log_info_raw("[USAGE]:  1 -> true,  run exhaustive extraction\n");
    log_info_raw("[USAGE]: <expr>            -> initial expression\n");
    log_info_raw("[USAGE]:  type             -> &str\n");
    log_info_raw("[USAGE]:  default           = None\n");
    log_info_raw("[USAGE]:  required         -> True if [-i] & [-o] not provided\n");
    log_info_raw("[USAGE]: <input filepath>  -> input expressions filepath\n");
    log_info_raw("[USAGE]:  type             -> &str\n");
    log_info_raw("[USAGE]:  default           = None\n");
    log_info_raw("[USAGE]:  required         -> True if [-e] not provided\n");
    log_info_raw("[USAGE]: <output filepath> -> output expressions filepath\n");
    log_info_raw("[USAGE]:  type             -> &str\n");
    log_info_raw("[USAGE]:  default           = None\n");
    log_info_raw("[USAGE]:  required         -> True if [-e] not provided\n");
    exit(1);
}

/// ### private function to set OS thread percentage
/// #### Argument
/// * `usr_input` - user input
/// #### Return
/// * `f64` OS thread percentage
fn set_thd_pct(cli: &mut Vec<CmdLineArg>, usr_input: &str) {
    let thd_pct = match usr_input.parse::<f64>() {
        Ok(thd_pct) => { thd_pct },
        Err(_) => {
            log_error(&format!("Invalid input value \"{}\" for OS threads percentage, expect f64.\n", usr_input));
            exit(1);
        }
    };
    if 0.0 < thd_pct && thd_pct <= 1.0 {
        cli[0] = CmdLineArg::Float(thd_pct);
    } else {
        log_error(&format!("Invalid input value \"{}\" for OS threads percentage, needs to be in (0.0, 1.0]\n", thd_pct));
        exit(1);
    }
}

/// ### private function to set maximum expression rewrite length
/// #### Argument
/// * `usr_input` - user input
/// #### Return
/// * `u8` - maximum expression rewrite length
fn set_num_tokens(cli: &mut Vec<CmdLineArg>, usr_input: &str) {
    let num_tokens = match usr_input.parse::<u8>(){
        Ok(num_tokens) => { num_tokens },
        Err(_) => {
            log_error(&format!("Invalid input value \"{}\" for number of tokens limit, expect u8.\n", usr_input));
            exit(1);
        }
    };
    if num_tokens > 0 {
        cli[1] = CmdLineArg::UInt(num_tokens);
    } else {
        log_error(&format!("Invalid input value \"{}\" for number of tokens limit, expect to u8 in (0, 2^8].\n", usr_input));
        exit(1);
    }
}

/// ### private function to set exhaustive extraction flag
/// #### Argument
/// * `usr_input` - user input
/// #### Return
/// * `bool` - exhaustive extraction flag
fn set_exhaustive_flag(cli: &mut Vec<CmdLineArg>, usr_input: &str) {
    let exhaustive = match usr_input.parse::<u8>(){
        Ok(exhaustive) => { exhaustive },
        Err(_) => {
            log_error(&format!("Invalid input value \"{}\" for exhaustive extraction flag, expect u8.\n", usr_input));
            exit(1);
        }
    };
    match exhaustive {
        0u8 => { cli[2] = CmdLineArg::Bool(false); },
        1u8 => { cli[2] = CmdLineArg::Bool(true); },
        _ => {
            log_error(&format!("Invalid input value \"{}\" for exhaustive extraction flag, expect either 0 || 1.\n", usr_input));
            exit(1);
        },
    }
}

/// ### public function to set hyper-parameters
/// #### Argument
/// * `args` - command line argument(s)
/// #### Return
/// * `None`
pub fn parse_args(args: &Vec<String>) -> Vec<CmdLineArg> {
    let args: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();

    if (!args.contains(&"-e") && !(args.contains(&"-i") && args.contains(&"-o"))) ||
       (args.contains(&"-e") && args.contains(&"-i")) ||
       (args.contains(&"-e") && args.contains(&"-o")) ||
       (args.contains(&"-i") ^ args.contains(&"-o")) {
        log_error("Either an initial expression or input & output filepaths is accepted.\n");
        help();
    }

    let mut cli: Vec<CmdLineArg> = vec![CmdLineArg::Float(0.8f64),
                                        CmdLineArg::UInt(25),
                                        CmdLineArg::Bool(false),];

    match args.len() {
        2 | 4 | 6 | 8 | 10 | 12 => { help(); },
        3 => {
            cli.push(CmdLineArg::String(args[2].to_string()));
        },
        5 => {
            match args[1] {
                "-t" => { set_thd_pct(&mut cli, args[2]); },
                "-n" => { set_num_tokens(&mut cli, args[2]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                "-e" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[1]));
                    help();
                },
            }
            match args[3] {
                "-t" => { set_thd_pct(&mut cli, args[4]); },
                "-n" => { set_num_tokens(&mut cli, args[4]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                "-e" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                    help();
                },
            }
        },
        7 => {
            if args.contains(&"-e") {
                match args[1] {
                    "-t" => { set_thd_pct(&mut cli, args[2]); },
                    "-n" => { set_num_tokens(&mut cli, args[2]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                    "-e" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[3] {
                    "-t" => { set_thd_pct(&mut cli, args[4]); },
                    "-n" => { set_num_tokens(&mut cli, args[4]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                    "-e" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[5] {
                    "-t" => { set_thd_pct(&mut cli, args[6]) },
                    "-n" => { set_num_tokens(&mut cli, args[6]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[6]); },
                    "-e" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[6]));
                        help() },
                }
            } else {
                match args[1] {
                    "-t" => { set_thd_pct(&mut cli, args[2]); },
                    "-n" => { set_num_tokens(&mut cli, args[2]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                    "-i" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[3] {
                    "-t" => { set_thd_pct(&mut cli, args[4]); },
                    "-n" => { set_num_tokens(&mut cli, args[4]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                    "-i" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[5] {
                    "-t" => { set_thd_pct(&mut cli, args[6]); },
                    "-n" => { set_num_tokens(&mut cli, args[6]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[6]); },
                    "-i" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
            }
        },
        9 => {
            if args.contains(&"-e") {
                match args[1] {
                    "-t" => { set_thd_pct(&mut cli, args[2]); },
                    "-n" => { set_num_tokens(&mut cli, args[2]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                    "-e" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[1]));
                        help() },
                }
                match args[3] {
                    "-t" => { set_thd_pct(&mut cli, args[4]); },
                    "-n" => { set_num_tokens(&mut cli, args[4]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                    "-e" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[5] {
                    "-t" => { set_thd_pct(&mut cli, args[6]); },
                    "-n" => { set_num_tokens(&mut cli, args[6]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[6]); },
                    "-e" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[7] {
                    "-t" => { set_thd_pct(&mut cli, args[8]); },
                    "-n" => { set_num_tokens(&mut cli, args[8]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[8]); },
                    "-e" => { cli.push(CmdLineArg::String(args[8].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
            } else {
                match args[1] {
                    "-t" => { set_thd_pct(&mut cli, args[2]); },
                    "-n" => { set_num_tokens(&mut cli, args[2]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                    "-i" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[1]));
                        help() },
                }
                match args[3] {
                    "-t" => { set_thd_pct(&mut cli, args[4]); },
                    "-n" => { set_num_tokens(&mut cli, args[4]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                    "-i" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                        help() },
                }
                match args[5] {
                    "-t" => { set_thd_pct(&mut cli, args[6]); },
                    "-n" => { set_num_tokens(&mut cli, args[6]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[6]); },
                    "-i" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[5]));
                        help() },
                }
                match args[7] {
                    "-t" => { set_thd_pct(&mut cli, args[8]); },
                    "-n" => { set_num_tokens(&mut cli, args[8]); },
                    "-f" => { set_exhaustive_flag(&mut cli, args[8]); },
                    "-i" => { cli.push(CmdLineArg::String(args[8].to_string())); },
                    "-o" => { cli.push(CmdLineArg::String(args[8].to_string())); },
                    _ => {
                        log_error(&format!("Invalid command line argument \"{}\".\n", &args[7]));
                        help() },
                }
            }
        },
        11 => {
            match args[1] {
                "-t" => { set_thd_pct(&mut cli, args[2]); },
                "-n" => { set_num_tokens(&mut cli, args[2]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[2]); },
                "-i" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                "-o" => { cli.push(CmdLineArg::String(args[2].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[1]));
                    help();
                },
            }
            match args[3] {
                "-t" => { set_thd_pct(&mut cli, args[4]); },
                "-n" => { set_num_tokens(&mut cli, args[4]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[4]); },
                "-i" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                "-o" => { cli.push(CmdLineArg::String(args[4].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[3]));
                    help();
                },
            }
            match args[5] {
                "-t" => { set_thd_pct(&mut cli, args[6]); },
                "-n" => { set_num_tokens(&mut cli, args[6]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[6]); },
                "-i" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                "-o" => { cli.push(CmdLineArg::String(args[6].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[5]));
                    help();
                },
            }
            match args[7] {
                "-t" => { set_thd_pct(&mut cli, args[8]); },
                "-n" => { set_num_tokens(&mut cli, args[8]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[8]); },
                "-i" => { cli.push(CmdLineArg::String(args[8].to_string())); },
                "-o" => { cli.push(CmdLineArg::String(args[8].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[7]));
                    help();
                },
            }
            match args[9] {
                "-t" => { set_thd_pct(&mut cli, args[10]); },
                "-n" => { set_num_tokens(&mut cli, args[10]); },
                "-f" => { set_exhaustive_flag(&mut cli, args[10]);  },
                "-i" => { cli.push(CmdLineArg::String(args[10].to_string())); },
                "-o" => { cli.push(CmdLineArg::String(args[10].to_string())); },
                _ => {
                    log_error(&format!("Invalid command line argument \"{}\".\n", &args[9]));
                    help();
                },
            }
        },
        _ => {
            log_error("Invalid command line arguments.\n");
            help();
        },
    }

    return cli;
}