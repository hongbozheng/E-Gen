use std::{process::exit};
use egg::{ContextGrammar, Language, Extractor, AstSize};

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
            println!("[INFO]: Executing program with the following default flags...");
            println!("[INFO]: csg flag   = false");
            println!("[INFO]: debug flag = false");
            println!("[INFO]: max rw len = 25\n");
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
    let init_expr: &str = "(* (cos x) y)";
    // let init_expr: &str = "(/ (d x (sin x)) (* -1 (d x (cos x))))";
    /* not working */
    /* haven't hard code the init_rw yet */
    // let init_expr: &str = "(* (sin x) y)";
    /* commutative rule break extraction */
    // let init_expr: &str = "(/ (* (* (d x (sin x)) (/ 1 (cos x))) (sin x)) (* -1 (d x (cos x))))";
    println!("[INFO]: Initial expression {}", init_expr);

    let mut ctx_g = ContextGrammar::new(csg, DEBUG, max_rw_len, init_expr);
    println!("\n[INFO]: Creating egraph with initial expression & rewrite rules...");
    ctx_g.set_egraph();
    println!("[INFO]: Creating grammar...");
    ctx_g.set_grammar();
    println!("[INFO]: Setting initial rewrite...");
    ctx_g.set_init_rw();

    let egraph = ctx_g.get_egraph();
    println!("[INFO]: EGraph total size {}", egraph.total_size());
    println!("[INFO]: EGraph contains {} node(s)", egraph.total_number_of_nodes());
    println!("[INFO]: EGraph contains {} eclass(es)", egraph.number_of_classes());

    /* TODO: DEBUG */
    println!("\n[DEBUG]: ------- EGraph Information -------");
    println!("[DEBUG]: ------------- EClass -------------");
    for eclass in egraph.classes() {
        println!("[DEBUG]: ------------ EClass {} ------------", eclass.id);
        for i in 0..eclass.nodes.len() {
            print!("[DEBUG]: enode {}", eclass.nodes[i]);
            for k in 0..eclass.nodes[i].children().len() {
                print!(" {}", eclass.nodes[i].children()[k]);
            }
            println!();
        }
        print!("[DEBUG]: parents");
        for k in 0..eclass.parents().len() {
            print!(" {:?}", eclass.parents().nth(k).unwrap());
        }
        println!("\n[DEBUG]: data  {:?}", eclass.data);
    }
    println!("[DEBUG]: ----------------------------------\n");

    println!("\n[INFO]: ---------- Root EClasses ----------");
    let root_eclasses = ctx_g.get_root_eclasses();
    print!("[INFO]:");
    for i in 0..root_eclasses.len() {
        print!(" {}", root_eclasses[i]);
    }
    println!("\n[INFO]: -----------------------------------");

    /* TODO: DEBUG */
    println!("\n[DEBUG]: ------------ Extractor -----------");
    let extractor = Extractor::new(&egraph, AstSize);
    let (best_cost, simpl_expr) = extractor.find_best(root_eclasses[0]);
    println!("[DEBUG]: Simplified Expression to {} with Cost {}",simpl_expr, best_cost);
    println!("[DEBUG]: ----------------------------------");

    println!("\n[INFO]: ------------- Grammar -------------");
    let grammar = ctx_g.get_grammar();
    for (eclass, rewrite) in grammar {
        println!("[INFO]: {} -> {:?}", eclass, rewrite);
    }
    println!("[INFO]: -----------------------------------");

    println!("\n[INFO]: --------- Initial Rewrite ---------");
    let init_rw = ctx_g.get_init_rw();
    println!("[INFO]: {:?}", init_rw);
    println!("[INFO]: -----------------------------------");

    ctx_g.extract();
    let mut rw_list = ctx_g.get_rw();
    let orig_rw_num = rw_list.len();
    /* rw_list.sort_unstable(); */
    rw_list.dedup();
    if orig_rw_num == rw_list.len() {
        println!("[INFO]: RW are all unique");
    } else {
        println!("[INFO]: RW have duplicates");
    }
    rw_list.sort_by(|rw1, rw2| rw1.len().cmp(&rw2.len()));
    println!("[INFO]: Total # of RW {}", rw_list.len());
    for rw in rw_list {
        println!("[INFO]: {}", rw);
    }
}