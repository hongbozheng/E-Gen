use crate::*;
use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use bincode::{serialize, deserialize};

/// max # of threads can be used (not max # of OS threads)
static mut MAX_NUM_THREADS: Option<Arc<Mutex<u32>>> = None;
/// private global variable to store eclass(es) to skip during extraction
static mut SKIP_ECLS: Option<HashMap<String, f64>> = None;
/// private global variable to store grammar from MathEGraph
static mut GRAMMAR: Option<HashMap<String, Vec<String>>> = None;
/// global variable to store equivalent expression results
static mut EQUIV_EXPRS: Option<Arc<Mutex<Vec<String>>>> = None;

pub unsafe fn get_max_num_threads() -> &'static Arc<Mutex<u32>> {
    return MAX_NUM_THREADS.as_ref().unwrap();
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

fn deserialize_data(serialized_data: &[u8]) -> Result<Data, Box<dyn Error>> {
    match bincode::deserialize::<Data>(serialized_data) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
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

        if op.len() == 1 || !op.starts_with('e') || op.starts_with("exp") || !grammar.contains_key(op) { continue; }
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

/// ### public function to start extracting equivalent expressions
/// #### Argument
/// * `args` command line arguments for hyperparameters
/// #### Return
/// * `None`
pub fn extract(args: &Vec<String>) {
    let cli: Vec<CmdLineArg> = args.iter().map(|arg| CmdLineArg::from_string(arg).unwrap()).collect();
    println!("{:?}", cli);

    let mut skip_ecls: HashMap<String, f64> = Default::default();
    let mut grammar: HashMap<String, Vec<String>> = Default::default();

    match TcpStream::connect(&args[5]) {
        Ok(mut stream) => {
            println!("Successfully connected to server in {}", args[5]);

            let mut data: Vec<u8> = vec![];

            match stream.read_to_end(&mut data) {
                Ok(_) => {
                    let data = deserialize_data(&data).unwrap();
                    skip_ecls = data.skip_ecls.into_iter().collect();
                    grammar = data.grammar.into_iter().collect();
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    log_error(&format!("Child process {:?} failed to receive data from parent process {} with error {}", &stream.peer_addr(), &args[5], e));
                }
            }

            // println!("sending ACK back to parent");
            // if let Err(err) = stream.write(b"ACK") {
            //     println!("Error sending data: {}", err);
            // }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        },
    }

    /* setup global variables */
    unsafe {
        if let CmdLineArg::UInt(max_rw_len) = &cli[2] {
            MAX_RW_LEN = *max_rw_len;
        }
        if let CmdLineArg::Bool(exhaustive) = &cli[3] {
            EXHAUSTIVE = *exhaustive;
        }
        SKIP_ECLS = Some(skip_ecls);
        GRAMMAR = Some(grammar);

        let equiv_exprs = Arc::new(Mutex::new(vec![]));
        EQUIV_EXPRS = Some(equiv_exprs);

        MAX_NUM_THREADS = Some(Arc::new(Mutex::new(100000u32)));

        // println!("{:?}", SKIP_ECLS);
        // println!("{:?}", GRAMMAR);
        // println!("{} {}", MAX_RW_LEN, EXHAUSTIVE);
    }

    let init_rw: &str = &cli[4].to_string();
    unsafe {
        /* start extraction */
        optimized_extract(init_rw.to_string(), 0);

        let mut equiv_exprs = (EQUIV_EXPRS.as_ref().unwrap().lock().unwrap()).clone();
        let mut set = HashSet::default();
        equiv_exprs.retain(|e| set.insert(e.clone()));

        /* send results back to parent process */
        match TcpStream::connect("127.0.0.1:8081") {
            Ok(mut stream) => {
                println!("Successfully connected to server in {} again", args[5]);
                let equiv_exprs_serialized = serialize(&equiv_exprs).unwrap();

                if let Err(err) = stream.write_all(&equiv_exprs_serialized) {
                    log_error(&format!("Failed to send data back to parent process with error {}.\n", err));
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            },
        }
    }

    // unsafe {
    //     let mut equiv_exprs = EQUIV_EXPRS.as_ref().clone().unwrap().lock().unwrap();
    //     equiv_exprs.sort_unstable();
    //     equiv_exprs.dedup();
    //     for expr in equiv_exprs.iter() {
    //         log_info(&format!("{}\n", expr));
    //     }
    // }

    println!("Terminated.");
    let pid = process::id();
    println!("PID: {} Terminated.", pid);

    return;
}