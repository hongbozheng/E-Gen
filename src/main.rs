use std::process::exit;

use egg::{ContextGrammar, Language, Extractor, AstSize};
/* import hyperparameter set up */
use egg::set_hyperparam;
/* import extraction functions */
use egg::{get_global_skip_ecls, get_global_grammar, get_global_equiv_exprs, setup_extract, extract};
/* import log level & logger functions */
use egg::{log_info, log_info_raw};
/* import refactor function */
use egg::refactor;
/* import utils functions */
use egg::{generate_dataset, pt_egraph_info, pt_root_ecls_info, pt_grammar, pt_init_rw, pt_skip_ecls, pt_rw};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    set_hyperparam(&args);

    // let _res = refactor("../data/equivexp_5_ops.test", "../data/refactor.test");

    let _res = generate_dataset("../data/equivexp_5_ops_ref.test", "../data/generate.test");
    exit(0);

    /* working */
    // let init_expr: &str = "(+ (d x (* 2 x)) y)";
    // let init_expr: &str = "(+ x (+ x (+ x x)))";
    // let init_expr: &str = "(* (cos x) y)";
    // let init_expr: &str = "(sin (* -1 x))";
    // let init_expr: &str = "(+ (pow (sin x) 2) (pow (cos x) 2))";
    // let init_expr: &str = "(/ (d x (sin x)) (* -1 (d x (cos x))))";
    // let init_expr: &str = "(/ (sec x) (sin x))";
    // let init_expr: &str = "(acos (+ 5 (* x (exp pi))))";
    let init_expr: &str = "(d x (pow (sin (* 2 x)) 2))";
    // let init_expr: &str = "(d x (pow x 2))";
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
    // let init_expr: &str = "(d x (pow (sin x) 2))";
    // let init_expr: &str = "(- (cos x) (cos y))";
    // let init_expr: &str = "(tan (- x y))";
    // let init_expr: &str = "(sin (- x y))";
    // let init_expr: &str = "(* (cos x) (sin y))";
    // let init_expr: &str = "(- (cos x) (cos y))";
    // let init_expr: &str = "(d x (tan x))"; // wow...

    // let init_expr: &str = "(* (sin x) y)";
    // let init_expr: &str = "(/ (d x (* x x)) 2)";
    // let init_expr: &str = "(/ (d x (* x x)) y)";
    // let init_expr: &str = "(/ (* 1 x) 1)";
    // let init_expr: &str = "(/ (* 2 x) (* 2 x))";
    // let init_expr: &str = "(/ (* 2 x) (* 2 1))";
    // let init_expr: &str = "(/ (/ (* 2 x) 2) x)";
    // let init_expr: &str = "(/ x x)";
    // let init_expr: &str = "(+ (sinh x) (cosh x))";
    // let init_expr: &str = "(sinh (+ x y))";
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
        log_info_raw("\n");
        log_info("Creating grammar & setting initial rewrite...\n");
        setup_extract(&mut ctx_gr);

        let skip_ecls = get_global_skip_ecls();
        pt_skip_ecls(skip_ecls);

        let grammar = get_global_grammar();
        pt_grammar(grammar);

        log_info_raw("\n");
        log_info(format!("Total # of grammar {}\n", grammar.len()).as_str());
    }

    let init_rw = ctx_gr.get_init_rw();
    pt_init_rw(init_rw);
    log_info_raw("\n");
    log_info(format!("Total # of initial rw {}\n", init_rw.len()).as_str());
    unsafe { extract(init_rw.clone());}

    unsafe {
        let mutex = get_global_equiv_exprs();
        pt_rw(mutex);
    }
}