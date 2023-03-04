use std::process::exit;
use egg::{ContextGrammar, Language, Extractor, AstSize};
/* import extraction functions */
use egg::{get_global_skip_ecls, get_global_grammar, get_global_rw_vec, setup_extract, extract};
/* import log level & logger functions */
use egg::{log_info, log_debug, log_info_raw, log_debug_raw};
/* import utils functions */
use egg::{pt_egraph_info, pt_root_ecls_info, pt_grammar, pt_init_rw, pt_skip_ecls};

fn help() {
    println!("[USAGE]: cargo run -csg <csg flag> -de <debug flag> -len <max rw len>");
    println!("[USAGE]:   <csg flag> -> context-sensitive grammar flag");
    println!("[USAGE]:   <csg flag> -> default 0");
    println!("[USAGE]:   <csg flag> -> required = false");
    println!("[USAGE]:            0 -> false, run context-free grammar");
    println!("[USAGE]:            1 -> true,  run context-sensitive grammar");
    println!("[USAGE]: <debug flag> -> context-sensitive grammar flag");
    println!("[USAGE]: <debug flag> -> default 0");
    println!("[USAGE]: <debug flag> -> required = false");
    println!("[USAGE]:            0 -> false, disable debug messages printing");
    println!("[USAGE]:            1 -> true,  enable debug messages printing");
    println!("[USAGE]: <max rw len> -> maximum rw length");
    println!("[USAGE]: <max rw len> -> default 25");
    println!("[USAGE]: <max rw len> -> required = false");
    exit(1);
}

fn set_csg_flag(csg: &mut bool, csg_flag: u8) {
    match csg_flag {
        0 => {},
        1 => { *csg = true; },
        _ => {
            eprintln!("[ERROR]: Invalid csg flag");
            help();
        },
    }
}

fn set_debug_flag(DEBUG: &mut bool, debug_flag: u8) {
    match debug_flag {
        0 => {},
        1 => { *DEBUG = true; },
        _ => {
            eprintln!("[ERROR]: Invalid debug flag");
            help();
        },
    }
}

fn set_max_rw_len(max_rw_len: &mut u8, max_rw_len_usr: u8) {
    *max_rw_len = max_rw_len_usr;
}

fn set_cli_1(args: &Vec<String>, csg: &mut bool, DEBUG: &mut bool, max_rw_len: &mut u8) {
    if args[1] == "-csg" {
        set_csg_flag(csg, args[2].parse::<u8>().unwrap());
    } else if args[1] == "-de" {
        set_debug_flag(DEBUG, args[2].parse::<u8>().unwrap());
    } else if args[1] == "-len" {
        set_max_rw_len(max_rw_len, args[2].parse::<u8>().unwrap());
    } else { help(); }
}

fn set_cli_2(args: &Vec<String>, csg: &mut bool, DEBUG: &mut bool, max_rw_len: &mut u8) {
    set_cli_1(args, csg, DEBUG, max_rw_len);
    if args[3] == "-csg" {
        set_csg_flag(csg, args[4].parse::<u8>().unwrap());
    } else if args[3] == "-de" {
        set_debug_flag(DEBUG, args[4].parse::<u8>().unwrap());
    } else if args[3] == "-len" {
        set_max_rw_len(max_rw_len, args[4].parse::<u8>().unwrap());
    } else { help(); }
}

fn set_cli_3(args: &Vec<String>, csg: &mut bool, DEBUG: &mut bool, max_rw_len: &mut u8) {
    set_cli_2(args, csg, DEBUG, max_rw_len);
    if args[5] == "-csg" {
        set_csg_flag(csg, args[6].parse::<u8>().unwrap());
    } else if args[5] == "-de" {
        set_debug_flag(DEBUG, args[6].parse::<u8>().unwrap());
    } else if args[5] == "-len" {
        set_max_rw_len(max_rw_len, args[6].parse::<u8>().unwrap());
    } else { help(); }
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut csg = false;
    let mut DEBUG = false;
    let mut max_rw_len = 25;

    match args.len() {
        1 => {
            log_info("Executing program with the following default flags...\n");
            log_info("csg flag   = false\n");
            log_info("debug flag = false\n");
            log_info("max rw len = 25\n");
            log_info_raw("\n");
        },
        2 => { help(); },
        3 => {
            set_cli_1(&args, &mut csg, &mut DEBUG, &mut max_rw_len);
        },
        4 => { help(); },
        5 => {
           set_cli_2(&args, &mut csg, &mut DEBUG, &mut max_rw_len);
        },
        6 => { help(); },
        7 => {
            set_cli_3(&args, &mut csg, &mut DEBUG, &mut max_rw_len);
        },
        _ => {
            eprintln!("[ERROR]: INVALID COMMAND LINE ARGUMENT(S)");
            eprintln!("[ERROR]: Run `cargo run -h` or `cargo run --help` to check CLI");
        },
    }

    /* working */
    // let init_expr: &str = "(+ (d x (* 2 x)) y)";
    // let init_expr: &str = "(+ x (+ x (+ x x)))";
    // let init_expr: &str = "(* (cos x) y)";
    // let init_expr: &str = "(sin (* -1 x))";
    // let init_expr: &str = "(+ (pow (sin x) 2) (pow (cos x) 2))";
    // let init_expr: &str = "(/ (d x (sin x)) (* -1 (d x (cos x))))";
    // let init_expr: &str = "(/ (sec x) (sin x))";
    // let init_expr: &str = "(+ (* (cos (/ x 2)) 1) 0)";
    // let init_expr: &str = "(sqrt (/ x 2))";
    // let init_expr: &str = "(* (* x 2) 2)";
    // let init_expr: &str = "(/ 1 (sec x))";
    // let init_expr: &str = "(/ 1 (csc x))";
    // let init_expr: &str = "(+ (pow (tan x) 2) 1)";
    // let init_expr: &str = "(cos (* 2 x))";
    // let init_expr: &str = "(- (pow (cos x) 2) (pow (sin x) 2))";
    // let init_expr: &str = "(/ (sin x) (cos x))";
    // let init_expr: &str = "(/ (cos x) (sec x))";
    // let init_expr: &str = "(d x (pow (cos x) 2))";
    let init_expr: &str = "(d x (pow (sin x) 2))";
    // let init_expr: &str = "(- (cos x) (cos y))";
    // let init_expr: &str = "(tan (- x y))";
    // let init_expr: &str = "(sin (- x y))";
    // let init_expr: &str = "(* (cos x) (sin y))";
    // let init_expr: &str = "(- (cos x) (cos y))";
    // let init_expr: &str = "(d x (tan x))"; // wow...

    // let init_expr: &str = "(* (sin x) y)";
    let init_expr: &str = "(/ (d x (* x x)) 2)";
    // let init_expr: &str = "(/ (d x (* x x)) y)";
    // let init_expr: &str = "(/ (* 1 x) 1)";
    // let init_expr: &str = "(/ (* 2 x) (* 2 x))";
    // let init_expr: &str = "(/ (* 2 x) (* 2 1))";
    // let init_expr: &str = "(/ (/ (* 2 x) 2) x)";
    // let init_expr: &str = "(/ x x)";
    // let init_expr: &str = "(+ (csch x) (cosh x))";
    // let init_expr: &str = "(d x (+ (pow x 2) (pow (sin x) 2)))";

    /* commutative rule break extraction */
    // let init_expr: &str = "(/ (* (* (d x (sin x)) (/ 1 (cos x))) (sin x)) (* -1 (d x (cos x))))";
    log_info(format!("Initial expression {}\n", init_expr).as_str());

    let mut ctx_gr = ContextGrammar::new(init_expr);
    log_info("Creating egraph with initial expression & rewrite rules...\n");
    ctx_gr.set_egraph();

    let egraph = ctx_gr.get_egraph();
    log_info_raw("\n");
    log_info(format!("EGraph total size {}\n", egraph.total_size()).as_str());
    log_info(format!("EGraph contains {} node(s)\n", egraph.total_number_of_nodes()).as_str());
    log_info(format!("EGraph contains {} eclass(es)\n", egraph.number_of_classes()).as_str());

    /* TODO: DEBUG */
    // pt_egraph_info(&egraph);

    let root_ecls = ctx_gr.get_root_ecls();
    pt_root_ecls_info(&root_ecls);

    /* TODO: DEBUG */
    // log_debug_raw("\n");
    // log_debug("------------ Extractor -----------\n");
    // let extractor = Extractor::new(&egraph, AstSize);
    // let (best_cost, simpl_expr) = extractor.find_best(root_ecls[0]);
    // log_debug(format!("Simplified Expression to {} with Cost {}\n",simpl_expr, best_cost).as_str());
    // log_debug("----------------------------------\n");

    unsafe {
        setup_extract(&mut ctx_gr);

        let skip_ecls = get_global_skip_ecls();
        pt_skip_ecls(skip_ecls);

        let grammar = get_global_grammar();
        // pt_grammar(grammar);
        log_info_raw("\n");
        log_info(format!("Total # of grammar {}\n", grammar.len()).as_str());
    }

    let init_rw = ctx_gr.get_init_rw();
    pt_init_rw(init_rw);
    log_info_raw("\n");
    log_info(format!("Total # of initial rw {}\n", init_rw.len()).as_str());
    extract(csg, init_rw.clone());

    unsafe {
        let results = get_global_rw_vec();
        log_info(format!("Total # of RW {}\n", results.lock().unwrap().len()).as_str());
        for rw in results.lock().unwrap().iter() {
            log_info(format!("{}\n", rw).as_str());
        }
    }
}