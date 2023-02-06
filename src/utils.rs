use std::fmt::format;
use crate::*;

/// ## function to print the type of a variable
/// ## Argument
/// * `_` - reference of any variable
pub fn pt_type_of<T>(_: &T) { println!("[DEBUG]: Var Type {}", std::any::type_name::<T>()); }

/// ## function to print egraph information
/// ## Argument
/// * `egraph` - egraph
pub fn pt_egraph_info(egraph: &MathEGraph) {
    log_debug_raw("\n");
    log_debug("------- EGraph Information -------\n");
    log_debug("------------- EClass -------------\n");
    for eclass in egraph.classes() {
        log_debug(format!("------------ EClass {} ------------\n", eclass.id).as_str());
        for i in 0..eclass.nodes.len() {
            if eclass.nodes[i].to_string().parse::<f64>().is_ok() {
                log_debug(format!("<f64> {}", eclass.nodes[i]).as_str());
            } else {
                log_debug(format!("enode {}", eclass.nodes[i]).as_str());
                for k in 0..eclass.nodes[i].children().len() {
                    log_debug_raw(format!(" {}", eclass.nodes[i].children()[k]).as_str());
                }
            }
            log_debug_raw("\n");
        }
        log_debug("parents");
        for k in 0..eclass.parents().len() {
            log_debug_raw(format!(" {:?}", eclass.parents().nth(k).unwrap()).as_str());
        }
        log_debug_raw("\n");
        log_debug(format!("data {:?}\n", eclass.data).as_str());
    }
    log_debug("----------------------------------\n");
}

/// ## function to print root eclasses
/// ## Argument
/// * `root_eclasses` - root eclass vec<Id>
pub fn pt_root_ecls_info(root_ecls: &Vec<Id>) {
    log_debug_raw("\n");
    log_debug("---------- Root EClasses ----------\n");
    log_debug(format!("{:?}\n", root_ecls).as_str());
    log_debug("-----------------------------------\n");
}

/// ## function to print eclass(es) to skip during extraction
/// ## Argument
/// * `skip_ecls` - vec<String> to skip during extraction
pub fn pt_skip_ecls(skip_ecls: &Vec<String>) {
    log_debug_raw("\n");
    log_debug("---------- Skip EClasses ----------\n");
    log_debug(format!("{:?}\n", skip_ecls).as_str());
    log_debug("-----------------------------------\n");
}

/// ## function to print grammar
/// ## Argument
/// * `grammar` - grammar HashMap
pub fn pt_grammar(grammar: &HashMap<String, Vec<String>>){
    log_debug_raw("\n");
    log_debug("------------- Grammar -------------\n");
    for (eclass, rewrite) in grammar {
        log_debug(format!("{} -> {:?}\n", eclass, rewrite).as_str());
    }
    log_debug("-----------------------------------\n");
}

/// ## function to print initial rewrites
/// ## Argument
/// * `init_rw` - init_rw Vec<String>
pub fn pt_init_rw(init_rw: &Vec<String>) {
    log_debug_raw("\n");
    log_debug("--------- Initial Rewrite ---------\n");
    log_debug(format!("{:?}\n", init_rw).as_str());
    log_debug("-----------------------------------\n");
}