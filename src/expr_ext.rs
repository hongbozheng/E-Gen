use crate::*;

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
/// ## Return
/// * `bool` - whether skip the current rewrite or not
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
unsafe fn exhaustive_extract(mut str: String, idx: u8) {
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
            } else if !contain_ecls(&str) {
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
                        exhaustive_extract(str.clone(), idx+1);
                    }).unwrap();
                    handle.join().unwrap();
                } else {
                    drop(mutex);
                    exhaustive_extract(str.clone(), idx+1);
                }

                log_trace(format!("Back to Function Call {}\n", idx).as_str());
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
unsafe fn optimized_extract(mut str: String, idx: u8) {
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
                        optimized_extract(str.clone(), idx+1);
                    }).unwrap();
                    handle.join().unwrap();
                } else {
                    drop(mutex);
                    optimized_extract(str.clone(), idx+1);
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

// /// ## function to perform rewrite extraction from egraph
// /// ## Argument
// /// * `csg` - context-sentitive grammar flag
// /// ## Return
// /// * `None`
// pub unsafe fn ssextract(init_rw: Vec<String>) {
//     log_info_raw("\n");
//     match EXHAUSTIVE {
//         true => {
//             let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
//             let mutex = global_max_num_threads.lock().unwrap();
//             log_info(&format!("MAX NUM THREADS {}\n", mutex));
//             drop(mutex);
//             log_info("Start multithreaded context-sensitive grammar extraction...\n");

//             let handles: Vec<_> = init_rw.into_iter().map(|rw| {
//                 thread::Builder::new().name(rw.clone()).spawn(move || {
//                     log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
//                     exhaustive_extract(rw, 0);
//                 }).unwrap()
//             }).collect();

//             log_info("Waiting for all threads to finish execution...\n");
//             for handle in handles {
//                 handle.join().unwrap();
//             }

//             log_info_raw("\n");
//             log_info("Finish context-sensitive grammar extraction\n");
//         },
//         false => {
//             let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
//             let mutex = global_max_num_threads.lock().unwrap();
//             log_info(&format!("MAX NUM THREADS {}\n", mutex));
//             drop(mutex);
//             log_info("Start multithreaded context-free grammar extraction...\n");

//             let handles: Vec<_> = init_rw.into_iter().map(|rw| {
//                 thread::Builder::new().name(rw.clone()).spawn(move || {
//                     log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
//                     optimized_extract(rw, 0);
//                 }).unwrap()
//             }).collect();

//             log_info("Waiting for all threads to finish execution...\n");
//             for handle in handles {
//                 handle.join().unwrap();
//             }

//             log_info_raw("\n");
//             log_info("Finish context-free grammar extraction\n");
//         },
//     }
// }