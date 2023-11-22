use crate::*;
use std::time::Instant;

/// private global variable to store eclass(es) to skip during extraction
static mut SKIP_ECLS: Option<HashMap<String, f64>> = None;
/// private global variable to store grammar from MathEGraph
static mut GRAMMAR: Option<HashMap<String, Vec<String>>> = None;
/// private global variable to store intermediate extraction states
static mut STATE: Option<HashSet<String>> = None;
/// private global variable to store equivalent expression results
static mut EQUIV_EXPRS: Option<HashSet<String>> = None;

/// ### public function to get private global variable SKIP_ECLS
/// #### Argument
/// * `None`
/// #### Return
/// * `SKIP_ECLS` - immutable reference of global variable SKIP_ECLS
pub unsafe fn get_global_skip_ecls() -> &'static HashMap<String, f64> {
    return SKIP_ECLS.as_ref().unwrap();
}

/// ### public function to get private global variable GRAMMAR
/// #### Argument
/// * `None`
/// #### Return
/// * `GRAMMAR` - immutable reference of global variable GRAMMAR
pub unsafe fn get_global_grammar() -> &'static HashMap<String, Vec<String>> {
    return GRAMMAR.as_ref().unwrap();
}

/// ### public function to get private global variable EQUIV_EXPRS
/// #### Argument
/// * `None`
/// #### Return
/// * `EQUIV_EXPRS` - immutable reference of global variable EQUIV_EXPRS
pub unsafe fn get_global_equiv_exprs() -> &'static HashSet<String> {
    return EQUIV_EXPRS.as_ref().unwrap();
}

/// ### private member function to check if an eclass appears in str
/// #### Arguments
/// * `eclass` - eclass index to search for
/// * `str`    - str to search
/// #### Return
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

/// ### private member function to skip meaningless rewrite rule(s)
/// #### Argument
/// * `rw` - rewrite rule
/// #### Return
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

/// ### private function to check whether tokens contain eclass
/// #### Arguments
/// * `tokens` - tokens (expression)
/// #### Return
/// * `bool` - whether eclass exists in tokens or not
fn contain_ecls(tokens: &Vec<String>) -> bool {
    for token in tokens {
        if token.len() >= 2 && token.starts_with("e") && token.chars().nth(1).unwrap().is_ascii_digit() {
            return true;
        }
    }
    return false;
}

/// ### private function to extract all equivalent mathematical expressions
/// ### Context-Free Grammar
/// #### Arguments
/// * `tokens` - tokenized expression
/// * `idx` - fn call idx for debugging purpose
/// #### Return
/// * `None`
unsafe fn optimized_extract(mut tokens: Vec<String>, idx: u8) {
    let start_time = get_start_time();
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs();
    if elapsed_time >= TIME_LIMIT as u64 {
        return;
    }

    log_trace("-----------------------------------\n");
    log_trace(&format!("Function Call {}\n", idx));
    let global_state = STATE.as_mut().unwrap();
    if global_state.contains(&tokens.join(" ")) {
        return;
    }
    global_state.insert(tokens.join(" "));
    let prev_tokens = tokens.clone();

    let mut term: bool = false;

    for i in 0..tokens.len() {
        if tokens.len() == 1 {
            let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
            let final_expr = tokens.join(" ");
            global_equiv_exprs.insert(final_expr.clone());
            log_trace_raw(&format!("[FINAL]: {}\n", final_expr));
            return;
        }

        let op = &tokens[i];
        let grammar = GRAMMAR.as_ref().unwrap();

        if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") ||
            !grammar.contains_key(op) { continue; }
        log_trace_raw(&format!("[ OP ]:  {}\n", op));
        let rw_list = grammar.get(op).unwrap();

        for k in 0..rw_list.len() {
            let rw = &rw_list[k];
            log_trace_raw(&format!("[INIT]:  {:?}\n", tokens));
            log_trace_raw(&format!("[ RW ]:  {:?}\n", rw));
            if SUPPRESS {
                if skip_rw(&rw) {
                    if k == rw_list.len()-1 {
                        term = true;
                        break;
                    }
                    continue;
                }
            }

            #[allow(unused_doc_comments)]
            /// ```rust
            /// /* Regex will solve indistinct eclass match in str.replacen() */
            /// /* Original Code */
            /// str = str.replacen(op, &*rw, 1);
            /// /* Using Regex (has performance issue since it's slow) */
            /// use regex::Regex;
            /// let mat = Regex::new(&format!(r"\b{}\b", op)).unwrap().find(&str).unwrap();
            /// str.replace_range(mat.start()..mat.end(), &rw);
            /// ```
            // replace_distinct_ecls(op, rw, &mut str);
            let rw_tokens: Vec<String> = rw.split_whitespace().map(|s| s.to_owned()).collect();
            tokens.splice(i..i+1, rw_tokens);
            log_trace_raw(&format!("[AFTER]: {:?}\n", tokens));

            if tokens.len() >= TOKEN_LIMIT as usize {
                log_trace("STR exceeds length limit, Try another RW...\n");
                if k == rw_list.len()-1 {
                    term = true;
                    break;
                }
                tokens = prev_tokens.clone();
                continue;
            }
            if !contain_ecls(&tokens) && k == rw_list.len()-1 {
                let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
                let final_expr = tokens.join(" ");
                global_equiv_exprs.insert(final_expr.clone());
                log_trace_raw(&format!("[FINAL]: {}\n", final_expr));
                term = true;
                break;
            } else if !contain_ecls(&tokens) {
                let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
                let final_expr = tokens.join(" ");
                global_equiv_exprs.insert(final_expr.clone());
                tokens = prev_tokens.clone();
                log_trace_raw(&format!("[FINAL]: {}\n", final_expr));
            } else {
                optimized_extract(tokens.clone(), idx+1);
                let start_time = get_start_time();
                let end_time = Instant::now();
                let elapsed_time = end_time.duration_since(start_time).as_secs();
                if elapsed_time >= TIME_LIMIT as u64 {
                    return;
                }

                log_trace(&format!("Back to Function Call {}\n", idx));
                tokens = prev_tokens.clone();
                if k == rw_list.len()-1 {
                    term = true;
                    break;
                }
            }
        }
        if term { break; }
    }
    log_trace(&format!("Finish Function Call {}\n", idx));
    log_trace("-----------------------------------\n");

    return;
}

/// ### private function to extract all equivalent mathematical expressions
/// ### Context-Sensitive Grammar
/// #### Arguments
/// * `tokens` - tokenized expression
/// * `idx` - fn call idx for debugging purpose
/// #### Return
/// * `None`
unsafe fn exhaustive_extract(mut tokens: Vec<String>, idx: u8) {
    let start_time = get_start_time();
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs();
    if elapsed_time >= TIME_LIMIT as u64 {
        return;
    }

    log_trace("-----------------------------------\n");
    log_trace(&format!("Function Call {}\n", idx));
    let prev_tokens = tokens.clone();
    // let expr: Vec<&str> = prev_expr.split_whitespace().collect();

    let mut term: bool = false;

    for i in 0..tokens.len() {
        if tokens.len() == 1 {
            let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
            let final_expr = tokens.join(" ");
            global_equiv_exprs.insert(final_expr.clone());
            log_trace_raw(&format!("[FINAL]: {}\n", final_expr));
            return;
        }

        let op = &tokens[i];
        let grammar = GRAMMAR.as_ref().unwrap();

        if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") ||
            !grammar.contains_key(op) { continue; }
        log_trace_raw(&format!("[ OP ]:  {}\n", op));
        let rw_list = grammar.get(op).unwrap();

        for k in 0..rw_list.len() {
            let rw = &rw_list[k];
            log_trace_raw(&format!("[INIT]:  {:?}\n", tokens));
            log_trace_raw(&format!("[ RW ]:  {:?}\n", rw));
            if SUPPRESS { if skip_rw(&rw) { continue; } }

            #[allow(unused_doc_comments)]
            /// ```rust
            /// /* Regex will solve indistinct eclass match in str.replacen() */
            /// /* Original Code */
            /// str = str.replacen(op, &*rw, 1);
            /// /* Using Regex (has performance issue since it's slow) */
            /// use regex::Regex;
            /// let mat = Regex::new(&format!(r"\b{}\b", op)).unwrap().find(&str).unwrap();
            /// str.replace_range(mat.start()..mat.end(), &rw);
            /// ```
            // replace_distinct_ecls(op, rw, &mut str);
            let rw_tokens: Vec<String> = rw.split_whitespace().map(|s| s.to_owned()).collect();
            tokens.splice(i..i+1, rw_tokens);
            log_trace_raw(&format!("[AFTER]: {:?}\n", tokens));

            if tokens.len() >= TOKEN_LIMIT as usize {
                log_trace("STR exceeds length limit, Try another RW...\n");
                tokens = prev_tokens.clone();
                continue;
            }
            if !contain_ecls(&tokens) && k == rw_list.len()-1 {
                let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
                let final_expr = tokens.join(" ");
                global_equiv_exprs.insert(final_expr.clone());
                log_trace_raw(&format!("[FINAL]: {:?}\n", final_expr));
                term = true;
                break;
            } else if !contain_ecls(&tokens) {
                let global_equiv_exprs = EQUIV_EXPRS.as_mut().unwrap();
                let final_expr = tokens.join(" ");
                global_equiv_exprs.insert(final_expr.clone());
                tokens = prev_tokens.clone();
                log_trace_raw(&format!("[FINAL]: {:?}\n", final_expr));
            } else {
                exhaustive_extract(tokens.clone(), idx+1);
                let start_time = get_start_time();
                let end_time = Instant::now();
                let elapsed_time = end_time.duration_since(start_time).as_secs();
                if elapsed_time >= TIME_LIMIT as u64 {
                    return;
                }

                log_trace(&format!("Back to Function Call {}\n", idx));
                tokens = prev_tokens.clone();
            }
        }
        if term { break; }
    }
    log_trace(&format!("Finish Function Call {}\n", idx));
    log_trace("-----------------------------------\n");

    return;
}

/// ### public function to start extracting equivalent expressions
/// #### Argument
/// * `clis` command line arguments for hyperparameters
/// * `skip_ecls` - skip eclasses
/// * `grammar` - grammar
/// * `init_exprs` - initial expressions
/// #### Return
/// * `None`
pub fn extract(cli: &Vec<CliDtype>, skip_ecls: &HashMap<String, f64>, grammar: &HashMap<String, Vec<String>>, init_exprs: &Vec<String>) {
    /* setup global variables */
    unsafe {
        if let CliDtype::Bool(optimized) = &cli[0] {
            OPTIMIZED = *optimized;
        }
        if let CliDtype::UInt8(n_equiv_exprs) = &cli[1] {
            N_EQUIV_EXPRS = *n_equiv_exprs;
        }
        if let CliDtype::UInt8(token_limit) = &cli[2] {
            TOKEN_LIMIT = *token_limit;
        }
        if let CliDtype::UInt8(max_token_limit) = &cli[3] {
            MAX_TOKEN_LIMIT = *max_token_limit;
        }
        if let CliDtype::UInt16(time_limit) = &cli[4] {
            TIME_LIMIT = *time_limit;
        }
        SKIP_ECLS = Some(skip_ecls.clone());
        GRAMMAR = Some(grammar.clone());
        STATE = Some(Default::default());
        EQUIV_EXPRS = Some(Default::default());
    }

    let init_token_exprs: Vec<Vec<String>> = init_exprs
        .iter()
        .map(|s| s.split_whitespace().map(|token| token.to_string()).collect())
        .collect();

    unsafe {
        START_TIME = Some(Instant::now());
        /* start extraction */
        if OPTIMIZED {
            for init_token_expr in init_token_exprs {
                optimized_extract(init_token_expr, 0);
            }
        } else {
            for init_token_expr in init_token_exprs {
                exhaustive_extract(init_token_expr, 0);
            }
        }
    }

    return;
}