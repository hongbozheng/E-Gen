use crate::*;
use std::process::exit;
use std::sync::{Arc, Mutex};

/// ### public function to set global max str len of rewrite
/// #### Argument
/// * `max_rw_len` - maximum rewrite length limit
/// #### Return
/// * `None`
pub unsafe fn set_max_rw_len(max_rw_len: u8) {
    MAX_RW_LEN = max_rw_len;
    return;
}

/// ### public function to set global exhaustive (exhaustive extraction flag)
/// #### Argument
/// * `exhaustive` - exhaustive extraction flag
/// #### Return
/// * `None`
pub unsafe fn set_exhaustive_flag(exhaustive: bool) {
    EXHAUSTIVE = exhaustive;
    return;
}

pub fn rm_permutation(equiv_exprs: &mut Vec<String>) {
    let mut equiv_exprs_distinct = HashSet::default();
    let mut i = 0;

    while i < equiv_exprs.len() {
        let mut tokens: Vec<String> = equiv_exprs[i].split_whitespace().collect();
        tokens.sort();
        let expr: String = tokens.iter().collect();

        if !equiv_exprs_distinct.contains(&expr) {
            equiv_exprs_distinct.insert(expr);
            i += 1;
        } else {
            equiv_exprs.remove(i);
        }
    }
}

/// ### public function to print the type of a variable
/// #### Argument
/// * `_` - reference of any variable
/// #### Return
/// * `None`
pub fn pt_type_of<T>(_: &T) {
    log_debug(&format!("Variable Type {}", std::any::type_name::<T>()));
    return;
}

/// ### public function to print egraph information
/// #### Argument
/// * `egraph` - egraph
/// #### Return
/// * `None`
pub fn pt_egraph_info(egraph: &MathEGraph) {
    log_debug_raw("\n");
    log_debug("------- EGraph Information -------\n");
    log_debug("------------- EClass -------------\n");
    for eclass in egraph.classes() {
        log_debug(&format!("------------ EClass {} ------------\n", eclass.id));
        for i in 0..eclass.nodes.len() {
            if eclass.nodes[i].to_string().parse::<f64>().is_ok() {
                log_debug(&format!("<f64> {}", eclass.nodes[i]));
            } else {
                log_debug(&format!("enode {}", eclass.nodes[i]));
                for k in 0..eclass.nodes[i].children().len() {
                    log_debug_raw(&format!(" {}", eclass.nodes[i].children()[k]));
                }
            }
            log_debug_raw("\n");
        }
        log_debug("parents");
        for k in 0..eclass.parents().len() {
            log_debug_raw(&format!(" {:?}", eclass.parents().nth(k).unwrap()));
        }
        log_debug_raw("\n");
        log_debug(&format!("data {:?}\n", eclass.data));
    }
    log_debug("----------------------------------\n");
    return;
}

/// ### public function to print root eclasses
/// #### Argument
/// * `root_eclasses` - root eclass vec<Id>
/// #### Return
/// * `None`
pub fn pt_root_ecls_info(root_ecls: &Vec<Id>) {
    log_debug_raw("\n");
    log_debug("---------- Root EClasses ----------\n");
    log_debug(&format!("{:?}\n", root_ecls));
    log_debug("-----------------------------------\n");
    return;
}

/// ### public function to print eclass(es) to skip during extraction
/// #### Argument
/// * `skip_ecls` - vec<String> to skip during extraction
/// #### Return
/// * `None`
pub fn pt_skip_ecls(skip_ecls: &HashMap<String, f64>) {
    log_debug_raw("\n");
    log_debug("---------- Skip EClasses ----------\n");
    log_debug(&format!("{:?}\n", skip_ecls));
    log_debug("-----------------------------------\n");
    return;
}

/// ### public function to print grammar
/// #### Argument
/// * `grammar` - grammar HashMap
/// #### Return
/// * `None`
pub fn pt_grammar(grammar: &HashMap<String, Vec<String>>){
    log_debug_raw("\n");
    log_debug("------------- Grammar -------------\n");
    for (eclass, rewrite) in grammar {
        log_debug(&format!("{} -> {:?}\n", eclass, rewrite));
    }
    log_debug("-----------------------------------\n");
    return;
}

/// ### public function to print initial rewrites
/// #### Argument
/// * `init_rw` - init_rw Vec<String>
/// #### Return
/// * `None`
pub fn pt_init_rw(init_rw: &Vec<String>) {
    log_debug_raw("\n");
    log_debug("--------- Initial Rewrite ---------\n");
    log_debug(&format!("{:?}\n", init_rw));
    log_debug("-----------------------------------\n");
    return;
}

/// ### public function to print equivalent expressions
/// #### Argument
/// * `mutex` - mutex of global variable rw_vec
/// #### Return
/// * `None`
pub fn pt_equiv_exprs(equiv_exprs_option: Option<Arc<Mutex<Vec<String>>>>) {
    let equiv_exprs_arc = match equiv_exprs_option {
        Some(equiv_exprs_arc) => { equiv_exprs_arc },
        None => {
            log_error("The variable equiv_exprs is None.\n");
            exit(1);
        },
    };

    let mut equiv_exprs = equiv_exprs_arc.lock().unwrap().clone();
    let mut set = HashSet::default();
    equiv_exprs.retain(|e| set.insert(e.clone()));
    for expr in equiv_exprs {
        log_info(&format!("{}\n", expr));
    }

    return;
}