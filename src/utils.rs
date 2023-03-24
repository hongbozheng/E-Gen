use std::process::Command;
use std::sync::{Arc, Mutex};
use crate::*;

/// ## function to set global max # of threads for extraction
/// ## MAX_NUM_THREADS = floor(MAX # of THREADS of the OS x MAX_NUM_THREADS_PCT)
/// ## Argument
/// * `None`
/// ## Return
/// * `None`
pub fn set_max_num_threads(max_num_threads: &mut Option<Arc<Mutex<u32>>>) {
    let output = Command::new("cat").arg("/proc/sys/kernel/threads-max")
        .output().expect("Failed to get MAX OS Threads!");
    let mut max_os_threads_str = String::from_utf8_lossy(&output.stdout).to_string();
    max_os_threads_str.pop();
    let max_os_threads = match max_os_threads_str.parse::<u32>() {
        Ok(max_os_threads) => max_os_threads,
        Err(e) => {
            log_error(&format!("Failed to parse {}: {}", max_os_threads_str, e));
            return;
        }
    };
    unsafe {
        match max_num_threads {
            None => {
                let global_max_num_threads = max_num_threads
                    .get_or_insert_with(|| Arc::new(Mutex::new(0)));
                let mut mutex = global_max_num_threads.lock().unwrap();
                *mutex = (max_os_threads as f32 * THREAD_PCT).floor() as u32;
                drop(mutex);
            },
            Some(_) => {
                log_error("[utils.rs] MAX_NUM_THREADS HAS ALREADY BEEN SET !\n");
            }
        }
    }
}

/// ## function to set global max str len of rewrite
/// ## Argument
/// * `max_rw_len` - maximum rewrite length limit
/// ## Return
/// * `None`
pub unsafe fn set_max_rw_len(max_rw_len: u8) { MAX_RW_LEN = max_rw_len; }

/// ## function to set global csg (context-sensitive grammar flag)
/// ## Argument
/// * `max_rw_len` - maximum rewrite length limit
/// ## Return
/// * `None`
pub unsafe fn set_csg(csg: bool) { CSG = csg; }

/// ## function to print the type of a variable
/// ## Argument
/// * `_` - reference of any variable
/// ## Return
/// * `None`
pub fn pt_type_of<T>(_: &T) {
    log_debug(&format!("Var Type {}", std::any::type_name::<T>()));
}

/// ## function to print egraph information
/// ## Argument
/// * `egraph` - egraph
/// ## Return
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
}

/// ## function to print root eclasses
/// ## Argument
/// * `root_eclasses` - root eclass vec<Id>
/// ## Return
/// * `None`
pub fn pt_root_ecls_info(root_ecls: &Vec<Id>) {
    log_debug_raw("\n");
    log_debug("---------- Root EClasses ----------\n");
    log_debug(&format!("{:?}\n", root_ecls));
    log_debug("-----------------------------------\n");
}

/// ## function to print eclass(es) to skip during extraction
/// ## Argument
/// * `skip_ecls` - vec<String> to skip during extraction
/// ## Return
/// * `None`
pub fn pt_skip_ecls(skip_ecls: &HashMap<String, f64>) {
    log_debug_raw("\n");
    log_debug("---------- Skip EClasses ----------\n");
    log_debug(&format!("{:?}\n", skip_ecls));
    log_debug("-----------------------------------\n");
}

/// ## function to print grammar
/// ## Argument
/// * `grammar` - grammar HashMap
/// ## Return
/// * `None`
pub fn pt_grammar(grammar: &HashMap<String, Vec<String>>){
    log_debug_raw("\n");
    log_debug("------------- Grammar -------------\n");
    for (eclass, rewrite) in grammar {
        log_debug(&format!("{} -> {:?}\n", eclass, rewrite));
    }
    log_debug("-----------------------------------\n");
}

/// ## function to print initial rewrites
/// ## Argument
/// * `init_rw` - init_rw Vec<String>
/// ## Return
/// * `None`
pub fn pt_init_rw(init_rw: &Vec<String>) {
    log_debug_raw("\n");
    log_debug("--------- Initial Rewrite ---------\n");
    log_debug(&format!("{:?}\n", init_rw));
    log_debug("-----------------------------------\n");
}

/// ## function to print final rw
/// ## Argument
/// * `mutex` - mutex of global variable rw_vec
/// ## Return
/// * `None`
pub fn pt_rw(mutex: &Arc<Mutex<Vec<String>>>) {
    let mut rw_vec = mutex.lock().unwrap();
    rw_vec.sort_unstable();
    rw_vec.dedup();
    for rw in rw_vec.iter() {
        log_info(&format!("{}\n", rw));
    }
}