use std::sync::{Arc, Mutex};
use std::thread;
use crate::*;
// use regex::Regex;

/// global variable to store grammar from ctx_gr so that it is visible by all threads
static mut GRAMMAR: Option<HashMap<String, Vec<String>>> = None;
/// global variable visible by all threads to store rewrite results
pub static mut RW_VEC: Option<Arc<Mutex<Vec<String>>>> = None;

/// ## function to set global variable GRAMMAR
/// ## to be identical to grammar in ctx_gr
/// ## so that GRAMMAR is visible by all threads
/// ## Argument
/// * `grammar` grammar from ctx_gr
unsafe fn set_glob_gr(grammar: &HashMap<String, Vec<String>>) {
    let glob_grammar = GRAMMAR.get_or_insert(HashMap::default());
    for (ecls, rw_rules) in grammar {
        glob_grammar.insert(ecls.clone(), rw_rules.clone());
    }
}

/// ## function to initialize global variable RW_VEC
/// ## to store rewrite results from all threads
unsafe fn set_rw_vec() {
    RW_VEC.get_or_insert(Arc::new(Mutex::new(Vec::new())));
}

/// ## function to perform rewrite extraction from egraph
/// ## Argument
/// `csg` context-sentitive grammar flag
/// `ctx_gr` context grammar struct
pub fn extract(csg: bool, ctx_gr: &ContextGrammar) {
    let grammar = ctx_gr.get_grammar();
    let init_rw = ctx_gr.get_init_rw().clone();

    unsafe {
        set_glob_gr(grammar);
        set_rw_vec();
    }

    match csg {
        true => unsafe {
            log_info_raw("\n");
            log_info("Start multithreaded context-sensitive grammar extraction...\n");

            let handles: Vec<_> = init_rw.into_iter().map(|rw| {
                thread::spawn(move || {
                    log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
                    csg_extract(rw, 0);
                })
            }).collect();

            log_info("Waiting for all threads to finish execution...\n");
            for handle in handles {
                handle.join().unwrap();
            }

            log_info_raw("\n");
            log_info("Finish context-sensitive grammar extraction\n");
        },
        false => {
            log_info_raw("\n");
            // log_info("Start context-free grammar extraction...\n");
            // let init_rw = self.ctx_gr.get_init_rw().clone();
            // for i in 0..init_rw.len() {
            //     log_info_raw("\n");
            //     log_info(format!("Extracting with No.{} initial rewrite {}...\n", i+1, init_rw[i]).as_str());
            //     self.cfg_extract(init_rw[i].clone(), 0);
            // }
            // self.cfg_extract("/ e3 e1".to_string().clone(), 0);
            // log_info_raw("\n");
            // log_info("Finish context-free grammar extraction\n");
        },
    }
}

/// ## private member function to check if an eclass appears in str
/// ## Argument
/// * `self`
/// * `eclass` - eclass index to search for
/// * `str`    - str to search
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
/// * `self`
/// * `rw` - rewrite rule
fn skip_rw(skip_ecls: &HashMap<String, f64>, rw: &String) -> bool {
    for (eclass, constant) in skip_ecls {
        if contain_distinct_ecls(eclass, rw) {
            match constant {
                0.0 => {
                    if rw.contains('+') { return true; }
                }
                1.0 => {
                    if rw.contains('*') { return true; }
                    else if rw.contains("pow") { return true; }
                }
                _ => { log_fatal("Invalid Pattern in fn skip_rw !\n"); }
            }
        }
    }
    return false;
}

/// ## private member function to update the frequency of rewrite rules
/// ## and check if it needs to skip the rewrite rule
/// ## Argument
/// `self`
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

/// ## private member function to replace distinct eclass with rewrite rule
/// ## Argument
/// * `self`
/// * `op`  - operand that needs to be replaced
/// * `rw`  - rewrite rule that is going to be replaced with
/// * `str` - original expression
fn replace_distinct_ecls(op: &str, rw: &String, str: &mut String) {
    let matches: Vec<_> = str.match_indices(op).collect();
    for mat in matches {
        let start_idx = &mat.0;
        let end_idx = &(start_idx + op.len());
        if (*end_idx != str.len() && str.chars().nth(*end_idx).unwrap() == ' ') ||
            *end_idx == str.len() {
            str.replace_range(start_idx..end_idx,rw);
            break;
        }
    }
}

/// ## private member function to check if any eclass is in str
/// ## Argument
/// `str` - current str
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

/// ## private member function to extract all equivalent mathematical expressions
/// ## Context-Sensitive Grammar
/// ## Argument
/// * `self`
/// * `str` - rewrite expression
/// * `idx` - fn call idx for debugging purpose
unsafe fn csg_extract(mut str: String, idx: u8) {
    // let grammar = Arc::new(grammar);
    log_trace("-----------------------------------\n");
    log_trace(format!("Function Call {}\n", idx).as_str());
    let prev_str = str.clone();
    let expr: Vec<&str> = prev_str.split(" ").collect();

    let mut term: bool = false;

    for i in 0..expr.len() {
        if expr.len() == 1 {
            let vec = RW_VEC.take().unwrap();
            vec.lock().unwrap().push(str.clone());
            RW_VEC = Some(vec);
            log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
            return;
        }
        let op = expr[i];

        let grammar = GRAMMAR.as_ref().unwrap();
        if !grammar.contains_key(op) {continue;}
        log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
        let rw_list = grammar.get(op).unwrap();

        for k in 0..rw_list.len() {
            let rw = &rw_list[k];
            log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
            log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());

            // if SUPPRESS { if skip_rw(rw) { continue; } }

            // if rw.contains('e') {
            //     if self.update_freq(rw, true) {
            //         // println!("[INFO]:  Freq exceeds limit, Switching RW...");
            //         continue;
            //     }
            // }

            #[warn(unused_doc_comments)]
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
                let vec = RW_VEC.take().unwrap();
                vec.lock().unwrap().push(str.clone());
                RW_VEC = Some(vec);
                log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
                term = true;
                break;
            } else if !contain_ecls(&str) {
                let vec = RW_VEC.take().unwrap();
                vec.lock().unwrap().push(str.clone());
                RW_VEC = Some(vec);
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

                // TODO: PROBABLY IMPLEMENT MULTI-THREAD HERE TOO TO INC EFFICIENCY
                csg_extract(str.clone(), idx+1);

                log_trace(format!("Back to Function Call {}\n", idx).as_str());
                // if rw.contains('e') {
                //     // println!("[INFO]:  Freq exceeds limit, try another rw...");
                //     self.update_freq(rw, false);
                // }
                str = prev_str.clone();
            }
            if term { break; }
        }

    }
    log_trace(format!("Finish Function Call {}\n", idx).as_str());
    log_trace("-----------------------------------\n");
}

/// ## private member function to extract all equivalent mathematical expressions
/// ## Context-Free Grammar
/// ## Argument
/// * `self`
/// * `str` - rewrite expression
/// * `idx` - fn call idx for debugging purpose
// fn cfg_extract(&mut self, mut str: String, idx: u8) {
//     log_trace("-----------------------------------\n");
//     log_trace(format!("Function Call {}\n", idx).as_str());
//     let prev_str = str.clone();
//     let expr: Vec<&str> = prev_str.split(" ").collect();
//
//     let mut term: bool = false;
//
//     let grammar = self.ctx_gr.get_grammar().clone();
//
//     for i in 0..expr.len() {
//         if expr.len() == 1 {
//             self.rw.push(str.clone());
//             log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
//             return;
//         }
//         let op = expr[i];
//         if !grammar.contains_key(op) { continue; }
//         log_trace_raw(format!("[ OP ]:  {}\n", op).as_str());
//         let rw_list = grammar.get(op).unwrap();
//
//         for k in 0..rw_list.len() {
//             let rw = &rw_list[k];
//             log_trace_raw(format!("[INIT]:  {}\n", str).as_str());
//             log_trace_raw(format!("[ RW ]:  {}\n", rw).as_str());
//             /// ```rust
//             /// /* Regex will solve indistinct eclass match in str.replacen() */
//             /// /* Original Code */
//             /// str = str.replacen(op, &*rw, 1);
//             /// /* Using Regex (has performance issue since it's slow) */
//             /// use regex::Regex;
//             /// let mat = Regex::new(format!(r"\b{}\b", op).as_str()).unwrap().find(str.as_str()).unwrap();                ///
//             /// str.replace_range(mat.start()..mat.end(), &rw);
//             /// ```
//             self.replace_distinct_ecls(op, rw, &mut str);
//             log_trace_raw(format!("[AFTER]: {}\n", str).as_str());
//
//             if str.len() >= self.max_rw_len as usize {
//                 log_trace("STR exceeds length limit, Try another RW...\n");
//                 str = prev_str.clone();
//                 continue;
//             }
//             if !str.contains('e') && k == rw_list.len()-1 {
//                 self.rw.push(str.clone());
//                 log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
//                 term = true;
//                 break;
//             } else if !str.contains('e') {
//                 self.rw.push(str.clone());
//                 str = prev_str.clone();
//                 log_trace_raw(format!("[FINAL]: {}\n", str).as_str());
//             } else {
//                 self.cfg_extract(str.clone(), idx+1);
//                 log_trace(format!("Back to Function Call {}\n", idx).as_str());
//                 str = prev_str.clone();
//                 if k == rw_list.len()-1 {
//                     term = true;
//                     break;
//                 }
//             }
//         }
//         if term { break; }
//     }
//     log_trace(format!("Finish Function Call {}\n", idx).as_str());
//     log_trace("-----------------------------------\n");
// }