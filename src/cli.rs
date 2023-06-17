use crate::*;
use std::fmt;
use std::process::exit;

pub enum Value<'a> {
    Bool(bool),
    Float64(f64),
    Str(&'a str),
    Uint8(u8),
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::Float64(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Uint8(value) => write!(f, "{}", value),
        }
    }
}

/// ## private function to print command line input help information
/// ## Argument
/// * `None`
/// ## Return
/// * `None`
fn help() {
    log_info_raw("[USAGE]: cargo run [-t] <thd pct> [-l] <max rw len>     [-f] <csg flag>\n");
    log_info_raw("[USAGE]:           [-e] <expr>    [-i] <input filepath> [-o] <output filepath>\n");
    log_info_raw("[USAGE]: <thd pct>    -> OS thread percentage\n");
    log_info_raw("[USAGE]:  type        -> float64\n");
    log_info_raw("[USAGE]:  default      = 1.0 [100%]\n");
    log_info_raw("[USAGE]:  required    -> false\n");
    log_info_raw("[USAGE]: <max rw len> -> maximum rw length\n");
    log_info_raw("[USAGE]:  type        -> uint8\n");
    log_info_raw("[USAGE]:  default      = 25\n");
    log_info_raw("[USAGE]:  required    -> false\n");
    log_info_raw("[USAGE]: <csg flag>   -> context-sensitive grammar flag\n");
    log_info_raw("[USAGE]:  type        -> uint8\n");
    log_info_raw("[USAGE]:  default      = 0\n");
    log_info_raw("[USAGE]:  required    -> false\n");
    log_info_raw("[USAGE]:  0 -> false, run context-free grammar\n");
    log_info_raw("[USAGE]:  1 -> true,  run context-sensitive grammar\n");
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
    exit(0);
}

/// ## private function to set OS thread percentage
/// ## Argument
/// * `usr_input` - user input
/// ## Return
/// * `f64` OS thread percentage
fn set_thd_pct(usr_input: &str) -> f64{
    let thd_pct = match usr_input.parse::<f64>() {
        Ok(thd_pct) => thd_pct,
        Err(_) => {
            log_error(format!("Invalid input value \"{}\" for OS threads percentage, expect f64.\n", usr_input).as_str());
            exit(0);
        }
    };
    if 0.0 < thd_pct && thd_pct <= 1.0 {
        return thd_pct;
    } else {
        log_error(format!("Invalid input value \"{}\" for OS threads percentage, needs to be in (0.0, 1.0]\n", thd_pct).as_str());
        exit(0);
    }
}

/// ## private function to set maximum expression rewrite length
/// ## Argument
/// * `usr_input` - user input
/// ## Return
/// * `u8` - maximum expression rewrite length
fn set_max_rw_len(usr_input: &str) -> u8 {
    let max_rw_len = match usr_input.parse::<u8>(){
        Ok(max_rw_len) => max_rw_len,
        Err(_) => {
            log_error(format!("Invalid input value \"{}\" for max rw length, expect u8.\n", usr_input).as_str());
            exit(0);
        }
    };
    if max_rw_len > 0 {
        return max_rw_len;
    } else {
        log_error(format!("Invalid input value \"{}\" for max rw length, expect to u8 in (0, 2^8].\n", usr_input).as_str());
        exit(0);
    }
}

/// ## private function to set exhaustive extraction flag
/// ## Argument
/// * `usr_input` - user input
/// ## Return
/// * `bool` - exhaustive extraction flag
fn set_flag(usr_input: &str) -> bool {
    let csg = match usr_input.parse::<u8>(){
        Ok(csg) => csg,
        Err(_) => {
            log_error(format!("Invalid input value \"{}\" for exhaustive extraction flag, expect u8.\n", usr_input).as_str());
            exit(0);
        }
    };
    match csg {
        0 => {return false;},
        1 => {return true;},
        _ => {
            log_error(format!("Invalid input value \"{}\" for exhaustive extraction flag, expect either 0 || 1.\n", usr_input).as_str());
            exit(0);
        },
    }
}

/// ## public function to set hyper-parameters
/// ## Argument
/// * `args` - command line arguments
/// ## Return
/// * `None`
pub fn parse_args(args: Vec<&str>) -> HashMap<&str, Value> {
    println!("{:?}", args);

    if (!args.contains(&"-e") && !(args.contains(&"-i") && args.contains(&"-o"))) ||
       (args.contains(&"-e") && args.contains(&"-i")) ||
       (args.contains(&"-e") && args.contains(&"-o")) ||
       (args.contains(&"-i") ^ args.contains(&"-o")) {
        log_error("Either an initial expression or input & output filepaths is accepted.\n");
        help();
    }

    let mut cli: HashMap<&str, Value> = Default::default();

    match args.len() {
        2 | 4 | 6 | 8 | 10 | 12 => { help(); },
        3 => {
            cli.insert("expr", Value::Str(args[2]));
        },
        5 => {
            match args[1] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[2]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[2]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[2]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-e" => { cli.insert("expr", Value::Str(args[2])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[1]).as_str());
                    help();
                },
            }
            match args[3] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[4]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[4]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[4]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-e" => { cli.insert("expr", Value::Str(args[4])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                    help();
                },
            }
        },
        7 => {
            if args.contains(&"-e") {
                match args[1] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[2]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[2]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[2]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[2])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[3] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[4]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[4]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[4]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[4])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[5] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[6]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[6]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[6]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[6])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[6]).as_str());
                        help();
                    },
                }
            } else {
                match args[1] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[2]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[2]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[2]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[2])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[2])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[3] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[4]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[4]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[4]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[4])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[4])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[5] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[6]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[6]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[6]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[6])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[6])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
            }
        },
        9 => {
            if args.contains(&"-e") {
                match args[1] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[2]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[2]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[2]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[2])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[1]).as_str());
                        help();
                    },
                }
                match args[3] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[4]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[4]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[4]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[4])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[5] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[6]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[6]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[6]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[6])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[7] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[8]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[8]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[8]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-e" => { cli.insert("expr", Value::Str(args[8])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
            } else {
                match args[1] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[2]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[2]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[2]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[2])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[2])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[1]).as_str());
                        help();
                    },
                }
                match args[3] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[4]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[4]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[4]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[4])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[4])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                        help();
                    },
                }
                match args[5] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[6]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[6]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[6]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[6])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[6])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[5]).as_str());
                        help();
                    },
                }
                match args[7] {
                    "-t" => {
                        let thd_pct = set_thd_pct(args[8]);
                        cli.insert("thd pct", Value::Float64(thd_pct));
                    },
                    "-l" => {
                        let max_rw_len = set_max_rw_len(args[8]);
                        cli.insert("max rw len", Value::Uint8(max_rw_len));
                    },
                    "-f" => {
                        let flag = set_flag(args[8]);
                        cli.insert("csg", Value::Bool(flag));
                    },
                    "-i" => { cli.insert("input filepath", Value::Str(args[8])); },
                    "-o" => { cli.insert("output filepath", Value::Str(args[8])); },
                    _ => {
                        log_error(format!("Invalid command line argument \"{}\"\n", &args[7]).as_str());
                        help();
                    },
                }
            }
        },
        11 => {
            match args[1] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[2]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[2]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[2]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-i" => { cli.insert("input filepath", Value::Str(args[2])); },
                "-o" => { cli.insert("output filepath", Value::Str(args[2])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[1]).as_str());
                    help();
                },
            }
            match args[3] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[4]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[4]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[4]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-i" => { cli.insert("input filepath", Value::Str(args[4])); },
                "-o" => { cli.insert("output filepath", Value::Str(args[4])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[3]).as_str());
                    help();
                },
            }
            match args[5] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[6]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[6]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[6]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-i" => { cli.insert("input filepath", Value::Str(args[6])); },
                "-o" => { cli.insert("output filepath", Value::Str(args[6])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[5]).as_str());
                    help();
                },
            }
            match args[7] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[8]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[8]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[8]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-i" => { cli.insert("input filepath", Value::Str(args[8])); },
                "-o" => { cli.insert("output filepath", Value::Str(args[8])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\"\n", &args[7]).as_str());
                    help();
                },
            }
            match args[9] {
                "-t" => {
                    let thd_pct = set_thd_pct(args[10]);
                    cli.insert("thd pct", Value::Float64(thd_pct));
                },
                "-l" => {
                    let max_rw_len = set_max_rw_len(args[10]);
                    cli.insert("max rw len", Value::Uint8(max_rw_len));
                },
                "-f" => {
                    let flag = set_flag(args[10]);
                    cli.insert("csg", Value::Bool(flag));
                },
                "-i" => { cli.insert("input filepath", Value::Str(args[10])); },
                "-o" => { cli.insert("output filepath", Value::Str(args[10])); },
                _ => {
                    log_error(format!("Invalid command line argument \"{}\".\n", &args[9]).as_str());
                    help();
                },
            }
        },
        _ => {
            log_error("Invalid command line arguments.\n");
            help();
        },
    }

    println!("{:?}", cli);

    return cli;
}