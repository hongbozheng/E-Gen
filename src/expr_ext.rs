use std::sync::{Arc, Mutex};
use std::thread;
use crate::*;
// use regex::Regex;

/// max # of threads can be used (not max # of OS threads)
pub static mut MAX_NUM_THREADS: Option<Arc<Mutex<u32>>> = None;
/// private global variable to store eclass(es) to skip during extraction
static mut SKIP_ECLS: Option<HashMap<String, f64>> = None;
/// private global variable to store grammar from MathEGraph
static mut GRAMMAR: Option<HashMap<String, Vec<String>>> = None;
/// global variable to store equivalent expression results
pub static mut EQUIV_EXPRS: Option<Arc<Mutex<Vec<String>>>> = None;

/// ## private function to set global variable GRAMMAR & SKIP_ECLS from MathEGraph
/// ## make GRAMMAR & SKIP_ECLS visible by all threads
/// ## Argument
/// * `math_egraph` - math_egraph
/// ## Return
/// * `None`
fn set_global_grammar(math_egraph: &MathEGraph) {
    let mut global_grammar = HashMap::default();
    let mut global_skip_ecls = HashMap::default();
    let eclasses = math_egraph.classes();

    for eclass in eclasses {
        let mut rewrite_rules: Vec<String> = vec![];
        let ecls: String = format!("{}{}", "e", eclass.id);
        let enodes = &eclass.nodes;

        if enodes.len() == 1 {
            match enodes[0].to_string().parse::<f64>() {
                Ok(float64) => {
                    if float64 == 1.0 || float64 == 0.0 {
                        global_skip_ecls.insert(ecls.clone(), float64);
                    }
                },
                Err(_) => {},
            }
        }

        for enode in enodes {
            let mut rewrite = enode.to_string();
            let children = enode.children();
            for child in children {
                rewrite = format!("{} {}{}", rewrite, "e", child);
            }
            rewrite_rules.push(rewrite);
        }
        global_grammar.insert(ecls, rewrite_rules);
    }

    unsafe {
        SKIP_ECLS = Some(global_skip_ecls);
        GRAMMAR = Some(global_grammar);
    }
}

/// ## private function to set the initial rewrite from self
/// ## Argument
/// * `ctx_gr` - context grammar struct
/// ## Return
/// * `None`
fn set_init_rw(ctx_gr: &mut ContextGrammar) {
    for rc in &ctx_gr.root_ecls {
        let mut root_ecls = format!("{}{}", "e", rc);
        unsafe {
            if GRAMMAR.as_ref().unwrap().contains_key(&*root_ecls) {
                ctx_gr.init_rw = GRAMMAR.as_ref().unwrap().get(&*root_ecls).unwrap().clone();
            } else {
                root_ecls = format!("{}{}", "e", ctx_gr.egraph.find(*rc));
                ctx_gr.init_rw = GRAMMAR.as_ref().unwrap().get(&*root_ecls).unwrap().clone();
            }
        }
    }
    /* TODO: May still have to fix simplified to const issue here !!!!! */
    // let mut root_eclass = format!("{}{}", "e", "8");
    // self.init_rw = self.grammar.get(&*root_eclass).unwrap().clone();
}

/// ## private function to initialize global variable EQUIV_EXPRS
/// ## to store rewrite results from all threads
/// ## Argument
/// * `None`
/// ## Return
/// * `None`
fn set_equiv_exprs() {
    let equiv_exprs = Arc::new(Mutex::new(vec![]));
    unsafe { EQUIV_EXPRS = Some(equiv_exprs); }
}

/// ## public function to get private global variable SKIP_ECLS
/// ## Argument
/// * `None`
/// ## Return
/// * `SKIP_ECLS` - immutable reference of global variable SKIP_ECLS
pub unsafe fn get_global_skip_ecls() -> &'static HashMap<String, f64> {
    return SKIP_ECLS.as_ref().unwrap();
}

/// ## public function to get private global variable GRAMMAR
/// ## Argument
/// * `None`
/// ## Return
/// * `GRAMMAR` - immutable reference of global variable GRAMMAR
pub unsafe fn get_global_grammar() -> &'static HashMap<String, Vec<String>> {
    return GRAMMAR.as_ref().unwrap();
}

/// ## public function to get private global variable EQUIV_EXPRS
/// ## Argument
/// * `None`
/// ## Return
/// * `EQUIV_EXPRS` - immutable reference of global variable EQUIV_EXPRS
pub unsafe fn get_global_equiv_exprs() -> &'static Arc<Mutex<Vec<String>>> {
    return EQUIV_EXPRS.as_ref().unwrap();
}

/// ## public function to setup for extraction
/// ## SKIP_ECLS, GRAMMAR, EQUIV_EXPRS
/// ## Argument
/// * `ctx_gr` context grammar struct
/// ## Return
/// * `None`
pub fn setup_extract(ctx_gr: &mut ContextGrammar) {
    let math_egraph = ctx_gr.get_egraph();
    set_global_grammar(math_egraph);
    set_init_rw(ctx_gr);
    set_equiv_exprs();
}

/// ## private member function to check if an eclass appears in str
/// ## Argument
/// * `eclass` - eclass index to search for
/// * `str`    - str to search
/// ## Return
/// * `bool` - whether distinct eclass exits in str or not
fn contain_distinct_ecls(eclass: &String, str: &String) -> bool {
    let matches: Vec<_> = str.match_indices(eclass).collect();
    for mat in matches {
        let start_idx = &mat.0;
        let end_idx = &(start_idx + eclass.len());
        if (*end_idx != str.len() && str.chars().nth(*end_idx).unwrap() == ' ') ||
            *end_idx == str.len() {
            return true;
        }
    }
    return false;
}

/// ## private member function to skip meaningless rewrite rule(s)
/// ## Argument
/// * `rw` - rewrite rule
unsafe fn skip_rw(rw: &String) -> bool {
    for (eclass, constant) in SKIP_ECLS.as_ref().unwrap() {
        if contain_distinct_ecls(eclass, rw) {
            if constant == &1.0f64 {
                if rw.contains('*') { return true; }
                else if rw.contains("pow") { return true; }
            } else if constant == &0.0f64 {
                if rw.contains('+') { return true; }
            } else {
                log_fatal("Invalid Pattern in fn skip_rw !\n");
            }
        }
    }
    return false;
}

/// ## private member function to update the frequency of rewrite rules
/// ## and check if it needs to skip the rewrite rule
/// ## Argument
/// * `self`
// unsafe fn update_freq(rw: &String, inc: bool) -> bool {
//     if inc {
//         if freq.contains_key(rw) && freq.get(rw).unwrap() < &FREQ_MAX {
//             *freq.get_mut(rw).unwrap() += 1;
//         } else if freq.contains_key(rw) && freq.get(rw).unwrap() == &FREQ_MAX {
//             return true;
//         } else {
//             freq.insert(rw.clone(), 1);
//         }
//     } else {
//         *freq.get_mut(rw).unwrap() -= 1;
//     }
//     return false;
// }

// pub fn update_freq(&mut self, rw: &String, inc: bool) -> bool {
//     if self.freq.contains_key(rw) {
//         *self.freq.get_mut(rw).unwrap() += 1;
//     } else {
//         self.freq.insert(rw.clone(), 1);
//     }
//     println!("{:?}", self.freq);
//     return false;
// }

/// ## private function to replace distinct eclass with rewrite rule
/// ## Argument
/// * `op`  - operand that needs to be replaced
/// * `rw`  - rewrite rule that is going to be replaced with
/// * `str` - original expression
fn replace_distinct_ecls(op: &str, rw: &String, str: &mut String) {
    let matches: Vec<_> = str.match_indices(op).collect();
    for mat in matches {
        let start_idx = &mat.0;
        let end_idx = &(start_idx + op.len());
        if (end_idx != &str.len() && str.chars().nth(*end_idx).unwrap() == ' ') ||
            end_idx == &str.len() {
            str.replace_range(start_idx..end_idx,rw);
            break;
        }
    }
}

/// ## private function to check if any eclass is in str
/// ## Argument
/// * `str` - current equation str
fn contain_ecls(str: &String) -> bool {
    let matches: Vec<_> = str.match_indices('e').collect();
    for mat in matches {
        let start_idx = &mat.0;
        if str.chars().nth(start_idx-1).unwrap() == ' ' &&
            str.chars().nth(start_idx+1).unwrap().is_ascii_digit() {
            return true;
        }
    }
    return false;
}

/// ## private function to extract all equivalent mathematical expressions
/// ## Context-Sensitive Grammar
/// ## Argument
/// * `str` - rewrite expression
/// * `idx` - fn call idx for debugging purpose
unsafe fn csg_extract(mut str: String, idx: u8) {
    log_trace("-----------------------------------\n");
    log_trace(format!("Function Call {}\n", idx).as_str());
    let prev_str = str.clone();
    let expr: Vec<&str> = prev_str.split_whitespace().collect();

    let mut term: bool = false;

    for i in 0..expr.len() {
        if expr.len() == 1 {
            let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
            let mut mutex = global_equiv_exprs.lock().unwrap();
            mutex.push(str.clone());
            drop(mutex);

            log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
            return;
        }

        let op = expr[i];
        let grammar = GRAMMAR.as_ref().unwrap();
        // TODO: THIS LINE IS EXTREMELY SLOW, WHEN !KEY, IT'S O(N)
        // probably it's still good idea to check
        if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") ||
            !grammar.contains_key(op) { continue; }
        log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
        let rw_list = grammar.get(op).unwrap();

        for k in 0..rw_list.len() {
            let rw = &rw_list[k];
            log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
            log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());

            if SUPPRESS { if skip_rw(rw) { continue; } }

            // if rw.contains('e') {
            //     if self.update_freq(rw, true) {
            //         // println!("[INFO]:  Freq exceeds limit, Switching RW...");
            //         continue;
            //     }
            // }

            #[allow(unused_doc_comments)]
            /// ```rust
            /// /* Regex will solve indistinct eclass match in str.replacen() */
            /// /* Original Code */
            /// str = str.replacen(op, &*rw, 1);
            /// /* Using Regex (has performance issue since it's slow) */
            /// use regex::Regex;
            /// let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();                ///
            /// str.replace_range(mat.start()..mat.end(), &rw);
            /// ```
            replace_distinct_ecls(op, rw, &mut str);
            log_trace_raw(format!("[AFTER]: {}\n", str).as_str());

            if str.len() >= MAX_RW_LEN as usize {
                log_trace("STR exceeds length limit, Try another RW...\n");
                // if rw.contains('e') {
                //     // println!("[INFO]:  Freq exceeds limit, try another rw...");
                //     self.update_freq(rw, false);
                // }
                str = prev_str.clone();
                continue;
            }
            if !contain_ecls(&str) && k == rw_list.len()-1 {
                // let mut global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap().lock().unwrap();
                // global_equiv_exprs.push(str.clone());

                let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
                let mut mutex = global_equiv_exprs.lock().unwrap();
                mutex.push(str.clone());
                drop(mutex);

                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                term = true;
                break;
            } else if !contain_ecls(&str) {
                let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
                let mut mutex = global_equiv_exprs.lock().unwrap();
                mutex.push(str.clone());
                drop(mutex);

                str = prev_str.clone();
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
            } else {
                // let grammar = Arc::clone(&grammar).deref();

                // println!("New Thread Spawn {}", 50-num_threads);
                // let thread = thread::spawn(move || {
                //     csg_extract(str.clone(),  idx+1);
                // });
                // thread.join().unwrap();

                // if num_threads > 0 {
                //     num_threads -= 1;
                //     println!("New Thread Spawn {}", 50-num_threads);
                //     let thread = thread::spawn(move || {
                //         csg_extract(str.clone(),  idx+1);
                //     });
                //     thread.join().unwrap();
                // } else { csg_extract(str.clone(), idx+1); }

                let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
                let mut mutex = global_max_num_threads.lock().unwrap();
                if *mutex > 0 {
                    *mutex -= 1;
                    drop(mutex);
                    let handle = thread::Builder::new().name(rw.clone()).spawn(move || {
                        csg_extract(str.clone(), idx+1);
                    }).unwrap();
                    handle.join().unwrap();
                } else {
                    drop(mutex);
                    csg_extract(str.clone(), idx+1);
                }

                log_trace(format!("Back to Function Call {}\n", idx).as_str());
                // if rw.contains('e') {
                //     // println!("[INFO]:  Freq exceeds limit, try another rw...");
                //     self.update_freq(rw, false);
                // }
                str = prev_str.clone();
            }
        }
        if term { break; }
    }
    log_trace(format!("Finish Function Call {}\n", idx).as_str());
    log_trace("-----------------------------------\n");
}

/// ## private function to extract all equivalent mathematical expressions
/// ## Context-Free Grammar
/// ## Argument
/// * `str` - rewrite expression
/// * `idx` - fn call idx for debugging purpose
unsafe fn cfg_extract(mut str: String, idx: u8) {
    log_trace("-----------------------------------\n");
    log_trace(format!("Function Call {}\n", idx).as_str());
    let prev_str = str.clone();
    let expr: Vec<&str> = prev_str.split(" ").collect();

    let mut term: bool = false;

    for i in 0..expr.len() {
        if expr.len() == 1 {
            let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
            let mut mutex = global_equiv_exprs.lock().unwrap();
            mutex.push(str.clone());
            drop(mutex);
            log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
            return;
        }

        let op = expr[i];
        let grammar = GRAMMAR.as_ref().unwrap();
        // TODO: THIS LINE IS EXTREMELY SLOW, WHENEVER !KEY, IT'S O(N)
        // probably it's still good idea to check
        if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") ||
            !grammar.contains_key(op) { continue; }
        log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
        let rw_list = grammar.get(op).unwrap();

        for k in 0..rw_list.len() {
            let rw = &rw_list[k];
            log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
            log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());

            #[allow(unused_doc_comments)]
            /// ```rust
            /// /* Regex will solve indistinct eclass match in str.replacen() */
            /// /* Original Code */
            /// str = str.replacen(op, &*rw, 1);
            /// /* Using Regex (has performance issue since it's slow) */
            /// use regex::Regex;
            /// let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();                ///
            /// str.replace_range(mat.start()..mat.end(), &rw);
            /// ```
            replace_distinct_ecls(op, rw, &mut str);
            log_trace_raw(format!("[AFTER]: {}\n", str).as_str());

            if str.len() >= MAX_RW_LEN as usize {
                log_trace("STR exceeds length limit, Try another RW...\n");
                str = prev_str.clone();
                continue;
            }
            if !contain_ecls(&str) && k == rw_list.len()-1 {
                let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
                let mut mutex = global_equiv_exprs.lock().unwrap();
                mutex.push(str.clone());
                drop(mutex);
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                term = true;
                break;
            } else if !str.contains('e') {
                let global_equiv_exprs = EQUIV_EXPRS.as_ref().unwrap();
                let mut mutex = global_equiv_exprs.lock().unwrap();
                mutex.push(str.clone());
                drop(mutex);
                str = prev_str.clone();
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
            } else {
                let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
                let mut mutex = global_max_num_threads.lock().unwrap();
                if *mutex > 0 {
                    *mutex -= 1;
                    drop(mutex);
                    let handle = thread::Builder::new().name(rw.clone()).spawn(move || {
                        csg_extract(str.clone(), idx+1);
                    }).unwrap();
                    handle.join().unwrap();
                } else {
                    drop(mutex);
                    csg_extract(str.clone(), idx+1);
                }

                log_trace(format!("Back to Function Call {}\n", idx).as_str());
                str = prev_str.clone();
                if k == rw_list.len()-1 {
                    term = true;
                    break;
                }
            }
        }
        if term { break; }
    }
    log_trace(format!("Finish Function Call {}\n", idx).as_str());
    log_trace("-----------------------------------\n");
}

/// ## function to perform rewrite extraction from egraph
/// ## Argument
/// * `csg` - context-sentitive grammar flag
/// ## Return
/// * `None`
pub unsafe fn extract(init_rw: Vec<String>) {
    log_info_raw("\n");
    match CSG {
        true => {
            let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
            let mutex = global_max_num_threads.lock().unwrap();
            log_info(&format!("MAX NUM THREADS {}\n", mutex));
            drop(mutex);
            log_info("Start multithreaded context-sensitive grammar extraction...\n");

            let handles: Vec<_> = init_rw.into_iter().map(|rw| {
                thread::Builder::new().name(rw.clone()).spawn(move || {
                    log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
                    csg_extract(rw, 0);
                }).unwrap()
            }).collect();

            log_info("Waiting for all threads to finish execution...\n");
            for handle in handles {
                handle.join().unwrap();
            }

            log_info_raw("\n");
            log_info("Finish context-sensitive grammar extraction\n");
        },
        false => {
            let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
            let mutex = global_max_num_threads.lock().unwrap();
            log_info(&format!("MAX NUM THREADS {}\n", mutex));
            drop(mutex);
            log_info("Start multithreaded context-free grammar extraction...\n");

            let handles: Vec<_> = init_rw.into_iter().map(|rw| {
                thread::Builder::new().name(rw.clone()).spawn(move || {
                    log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
                    cfg_extract(rw, 0);
                }).unwrap()
            }).collect();

            log_info("Waiting for all threads to finish execution...\n");
            for handle in handles {
                handle.join().unwrap();
            }

            log_info_raw("\n");
            log_info("Finish context-free grammar extraction\n");
        },
    }
}