use std::process::exit;
use crate::*;

/// ## private function to print command line input help information
/// ## Argument
/// * `None`
/// ## Return
/// * `None`
fn help() {
    println!("[USAGE]: cargo run -thd <thread pct> -len <max rw len> -csg <csg flag>");
    println!("[USAGE]:   <thread pct> -> OS thread percentage");
    println!("[USAGE]:   <thread pct> -> default 1.0 [100%]");
    println!("[USAGE]:   <thread pct> -> required = false");
    println!("[USAGE]:   <max rw len> -> maximum rw length");
    println!("[USAGE]:   <max rw len> -> default 25");
    println!("[USAGE]:   <max rw len> -> required = false");
    println!("[USAGE]:     <csg flag> -> context-sensitive grammar flag");
    println!("[USAGE]:     <csg flag> -> default 0");
    println!("[USAGE]:     <csg flag> -> required = false");
    println!("[USAGE]:              0 -> false, run context-free grammar");
    println!("[USAGE]:              1 -> true,  run context-sensitive grammar");
    exit(1);
}

/// ## private function to set global variable os thread percentage
/// ## Argument
/// * `usr_input` - user input
/// ## Return
/// * `None`
fn set_thread_pct(usr_input: &String) {
    let thread_pct = match usr_input.parse::<f32>() {
        Ok(thread_pct) => thread_pct,
        Err(_) => {
            log_error("Invalid input value for OS threads percentage, expect f32.\n");
            exit(1);
        }
    };
    if 0.0 < thread_pct && thread_pct <= 1.0 {
        unsafe { THREAD_PCT = thread_pct; }
        set_max_num_threads();
    } else {
        log_error("Invalid input value for OS threads percentage, needs to be in (0.0, 1.0]\n");
        exit(1);
    }
}

/// ## private function to set global variable max rewrite length
/// ## Argument
/// * `usr_input` - user input
/// ## Return
/// * `None`
fn set_max_rw_len(usr_input: &String) {
    let max_rw_len = match usr_input.parse::<u8>(){
        Ok(max_rw_len) => max_rw_len,
        Err(_) => {
            log_error("Invalid input value for max rw length, expect u8.\n");
            exit(1);
        }
    };
    if max_rw_len > 0 {
        unsafe { MAX_RW_LEN = max_rw_len; }
    } else {
        log_error("Invalid input value for max rw length, expect to u8 in (0, 2^8].\n");
        exit(1);
    }
}

/// ## private function to set variable csg flag
/// ## Argument
/// * `usr_input` - user input
/// * `csg` - context-sensitive grammar flag
/// ## Return
/// * `None`
fn set_csg(usr_input: &String) {
    let csg = match usr_input.parse::<u8>(){
        Ok(csg) => csg,
        Err(_) => {
            log_error("Invalid input value for csg flag, expect u8.\n");
            exit(1);
        }
    };
    match csg {
        0 => unsafe { CSG = false; },
        1 => {},
        _ => {
            log_error("Invalid input value for csg flag, expect either 0 || 1.\n");
            exit(1);
        },
    }
}

/// ## private function to set command line input 1
/// ## Argument
/// * `args` - command line arguments
/// ## Return
/// * `None`
fn set_cli_1(args: &Vec<String>) {
    let usr_input = &args[2];
    if args[1] == "-thd" {
        set_thread_pct(usr_input);
    } else if args[1] == "-len" {
        set_max_rw_len(usr_input);
    } else if args[1] == "-csg" {
        set_csg(usr_input);
    } else { help(); }
}

/// ## private function to set command line input 2
/// ## Argument
/// * `args` - command line arguments
/// ## Return
/// * `None`
fn set_cli_2(args: &Vec<String>) {
    set_cli_1(args);
    let usr_input = &args[4];
    if args[3] == "-thd" {
        set_thread_pct(usr_input);
    } else if args[3] == "-len" {
        set_max_rw_len(usr_input);
    } else if args[3] == "-csg" {
        set_csg(usr_input);
    } else { help(); }
}

/// ## private function to set command line input 3
/// ## Argument
/// * `args` - command line arguments
/// ## Return
/// * `None`
fn set_cli_3(args: &Vec<String>) {
    set_cli_2(args);
    let usr_input = &args[6];
    if args[5] == "-thd" {
        set_thread_pct(usr_input);
    } else if args[5] == "-len" {
        set_max_rw_len(usr_input);
    } else if args[5] == "-csg" {
        set_csg(usr_input);
    } else { help(); }
}

/// ## public function to set hyper-parameters
/// ## Argument
/// * `args` - command line arguments
/// ## Return
/// * `None`
pub fn set_hyperparam(args: &Vec<String>) {
    match args.len() {
        1 => {
            log_info("Executing program with the following default values...\n");
            log_info("os threads = 1.00 [100%]\n");
            log_info("max rw len = 25\n");
            log_info("csg flag   = false\n");
            log_info_raw("\n");
            set_max_num_threads();
        },
        2 => { help(); },
        3 => {
            set_cli_1(args);
        },
        4 => { help(); },
        5 => {
            set_cli_2(args);
        },
        6 => { help(); },
        7 => {
            set_cli_3(args);
        },
        _ => {
            log_error("INVALID COMMAND LINE ARGUMENT(S)\n");
            log_error("Run `cargo run -h` or `cargo run --help` to check CLI\n");
        },
    }
}