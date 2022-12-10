extern crate core;

use std::{env::args, process::exit};
use egg::{ContextGrammar, Language, Math, math_rule, MathEGraph, RecExpr, Runner};

fn help() {
    println!("[USAGE]: cargo run -csg <csg flag> -de <debug flag>");
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

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut csg = false;
    let mut DEBUG = false;

    match args.len() {
        1 => {
            println!("[INFO]: Executing program with the following default flags...");
            println!("[INFO]: csg flag   = false");
            println!("[INFO]: debug flag = false\n");
        },
        2 => { help(); },
        3 => {
            if args[1] == "-csg" {
                set_csg_flag(&mut csg, args[2].parse::<u8>().unwrap());
            } else if args[1] == "-de" {
                set_debug_flag(&mut DEBUG, args[2].parse::<u8>().unwrap());
            } else { help(); }
        },
        4 => { help(); },
        5 => {
            if args[3] == "-csg" {
                set_csg_flag(&mut csg, args[4].parse::<u8>().unwrap());
            } else if args[3] == "-de" {
                set_debug_flag(&mut DEBUG, args[4].parse::<u8>().unwrap());
            } else { help(); }
        },
        _ => {
            eprintln!("[ERROR]: INVALID COMMAND LINE ARGUMENT(S)");
            eprintln!("[ERROR]: Run `cargo run -h` or `cargo run --help` to check CLI");
        },
    }

    let expr: &str = "(* x y)";
    println!("[INFO]: Initial expression {}", expr);
    let recexpr: RecExpr<Math> = expr.parse().unwrap();
    let runner = Runner::default().with_expr(&recexpr).run(&math_rule());
    let egraph: MathEGraph = runner.egraph;
    let iters = runner.iterations;
    let roots = runner.roots;
    let stop_reason = runner.stop_reason;
    println!("[INFO]: EGraph total size {}", egraph.total_size());
    println!("[INFO]: EGraph contains {} node(s)", egraph.total_number_of_nodes());
    println!("[INFO]: EGraph contains {} eclass(es)\n", egraph.number_of_classes());
    let eclasses = egraph.classes();
    println!("[INFO]: EClass Information");
    for eclass in eclasses {
        println!("[INFO]: {:?}",eclass);
        // let id = &eclass.id;
        // let enodes = &eclass.nodes;
        // println!("enodes in eclass id: {}",id);
        // for enode in enodes {
        //     println!("{}",enode);
        //     let children = enode.children();
        //     if children.is_empty() {println!("children node(s): None");}
        //     else {println!("children node(s): {:?}",children);}
        // }
        // println!("\n");
    }
    // println!("Iterations");
    // for iter in &iters {
    //     println!("{:?}",iter);
    // }

    print!("\n[INFO]: Runner Root(s)");
    for root in &roots {
        print!(" {:?}",root);
    }
    println!("\n[INFO]: Root EClass ID {}\n", roots[0]);
    // println!("\n[INFO]: Stop Reason {:?}",stop_reason.unwrap());
    // let extractor = Extractor::new(&egraph,AstSize);
    // //let find_cost = extractor.find_costs();
    // let (best_cost,simplified_expr) = extractor.find_best(roots[0]);
    // println!("Simplified Expression {} to {} with Cost {}",expr,simplified_expr,best_cost);
    //
    // println!("--------------------------------------------------\n");
    // let csg = std::env::args().nth(1);
    // match csg.is_some() {
    //     Ok(true) => { println!("[INFO]: Context-Sensitive Grammar Flag = {:?}", csg) }
    //     Err(false) => {}
    // }
    // println!("{:?}",csg);
    // let DEBUG = std::env::args().nth(2).expect("[CLI]: DEBUG Message Flag");

    let mut ctx_g = ContextGrammar::new(egraph, expr, roots, DEBUG);
    println!("[INFO]: Creating grammar...");
    ctx_g.set_grammar();
    println!("[INFO]: Finish creating grammar");

    println!("[INFO]: Setting initial expression...");
    ctx_g.set_init_rw();
    println!("[INFO]: Finish setting inital expression\n");

    println!("[INFO]: ---------- Grammar ----------");
    let grammar = ctx_g.get_grammar();
    for (eclass, rewrite) in grammar {
        println!("[INFO]: {} -> {:?}", eclass, rewrite);
    }
    println!("[INFO]: -----------------------------\n");

    println!("[INFO]: ----- Initial Expression ----");
    let init_rw = ctx_g.get_init_rw();
    println!("[INFO]: {}", init_rw);
    println!("[INFO]: -----------------------------\n");

    if csg {
        
    } else {
        println!("[INFO]: Start context-free grammar extraction...");
        ctx_g.cfg_extract(init_rw, 0);
    }
}