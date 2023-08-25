use crate::*;
use bincode::{serialize, deserialize};
use libc::{c_int, cpu_set_t, CPU_SET, pid_t, sched_setaffinity};
use num_cpus;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::mem::{size_of, zeroed};
use std::net::TcpListener;
use std::process::{Child, Command, exit};

#[derive(Debug, Serialize, Deserialize)]
/// data for extraction
pub struct Data {
    /// variable to store eclass(es) to skip during extraction
    pub skip_ecls: HashMap<String, f64>,
    /// variable to store grammar from MathEGraph
    pub grammar: HashMap<String, Vec<String>>,
}

/// ### private unsafe function to set process affinity
/// #### Arguments
/// * `pid` - process id
/// * `processor id` - processor id (CPU logic core id)
/// #### Return
/// * `c_int` - return 0 on success, return -1 on failure
unsafe fn set_proc_affinity(pid: pid_t, processor_id: usize) -> c_int {
    let mut cpuset: cpu_set_t = zeroed();
    CPU_SET(processor_id, &mut cpuset);
    sched_setaffinity(pid, size_of::<cpu_set_t>(), &cpuset)
}

/// ### private function generate equivalent expressions
/// ### with 1 input expression
/// #### Argument
/// * `cli` - pre-processed command line arguments
/// #### Return
/// * `equiv_expr` - Vec<String> of equivalent expressions
fn generate_exprs(cli: &mut Vec<CmdLineArg>) -> HashSet<String> {
    /* initialize ctx_gr struct and create egraph, skip_ecls, grammar, init_rewrite */
    let input_expr = cli[3].to_string();
    log_info(&format!("Expression: {}\n", input_expr));
    let mut ctx_gr = ContextGrammar::new(input_expr);
    ctx_gr.setup();
    let init_exprs = &ctx_gr.init_exprs.clone();

    /* get number of processes */
    let num_proc = init_exprs.len();

    /* tx & rx listener */
    let addr = "127.0.0.1:8080";
    let listener = match TcpListener::bind(&addr) {
        Ok(listener) => { listener },
        Err(e) => {
            log_error(&format!("[ERROR]: Failed to bind IP address \"{}\" with error {}.\n", addr, e));
            exit(1);
        },
    };

    /* insert socket address & get CPU's number of logical cores */
    cli.push(CmdLineArg::String(addr.to_string()));
    let num_logical_cores = num_cpus::get();

    /* spawn children processes & set process affinity */
    let mut child_procs: Vec<Child> = init_exprs.into_iter().zip(0..num_proc).map(|(rw, proc_idx)| {
        cli[3] = CmdLineArg::String(rw.clone());

        let args: Vec<String> = cli.iter().map(|arg| arg.to_string()).collect();

        match Command::new("../target/debug/multiproc").args(&args).spawn() {
            Ok(child_proc) => {
                let pid = child_proc.id() as pid_t;
                let processor_id = proc_idx % num_logical_cores;
                let ret = unsafe { set_proc_affinity(pid, processor_id) };
                match ret {
                    0 => { log_debug(&format!("Set process {}'s process affinity to processor {}.\n", pid, processor_id)); },
                    _ => {
                        log_error(&format!("Failed to set process {}'s process affinity to processor {}.\n", pid, processor_id));
                        exit(1);
                    },
                }
                child_proc
            },
            Err(e) => {
                log_error(&format!("Failed to spawn child process with error {}.\n", e));
                exit(1);
            },
        }
    }).collect();

    let mut num_acks: u8 = 0u8;

    /* send data to all children processes through sockets */
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let skip_ecls = ctx_gr.skip_eclasses.clone();
                let grammar = ctx_gr.grammar.clone();

                let data: Data = Data {
                    skip_ecls,
                    grammar,
                };

                let data_bytes = match serialize(&data) {
                    Ok(data_bytes) => { data_bytes },
                    Err(e) => {
                        log_error(&format!("Failed to serialize data with error {}.\n", e));
                        exit(1);
                    },
                };

                match stream.write_all(&data_bytes) {
                    Ok(_) => {
                        num_acks += 1;
                        log_debug(&format!("Data send to child process {:?} successfully.\n", stream.peer_addr()));
                    },
                    Err(e) => {
                        log_error(&format!("Failed to send data to child process {:?} with error {}.\n", stream.peer_addr(), e));
                        exit(1);
                    },
                }
            }
            Err(e) => {
                log_error(&format!("Failed to connect to child process with error {}.\n", e));
                exit(1);
            }
        }

        if num_acks as usize == num_proc { break; }
    }

    num_acks = 0u8;
    let mut equiv_exprs: HashSet<String> = Default::default();

    /* receive equivalent expressions from all children processes */
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut equiv_exprs_bytes: Vec<u8> = vec![];
                match stream.read_to_end(&mut equiv_exprs_bytes) {
                    Ok(_) => {
                        let equiv_exprs_proc: std::collections::HashSet<String> = match deserialize(&equiv_exprs_bytes) {
                            Ok(equiv_exprs_proc) => { equiv_exprs_proc },
                            Err(e) => {
                                log_error(&format!("Failed to deserialize data received from child process with error {}.\n", e));
                                exit(1);
                            },
                        };
                        let ex: HashSet<String> = equiv_exprs_proc.into_iter().collect();
                        equiv_exprs.extend(ex.into_iter());
                        // equiv_exprs.append(&mut equiv_exprs_proc);
                        num_acks += 1;
                    },
                    Err(e) => {
                        log_error(&format!("Failed to receive data from child process socket address {:?} with error {}.\n", stream.peer_addr(), e));
                        exit(1);
                    },
                }
            }
            Err(e) => {
                log_error(&format!("Failed to connect to child process with error {}.\n", e));
                exit(1);
            }
        }
        if num_acks as usize == num_proc {
            drop(listener);
            break;
        }
    }

    /* check if all children processes exit successfully */
    for child_proc in &mut child_procs {
        let pid = child_proc.id();
        match child_proc.wait() {
            Ok(exit_status) => {
                match exit_status.code() {
                    Some(0) => { log_debug(&format!("Child process {} terminated successfully with an exit code 0.\n", pid)); },
                    Some(exit_code) => { log_error(&format!("Child process {} terminated with a non-zero exit code {}.\n", pid, exit_code)); },
                    None => { log_error(&format!("Child process {} terminated with an unknown exit code.\n", pid)); },
                }
            },
            Err(e) => { log_error(&format!("Child process {} is not running with error {}.\n", pid, e)); },
        }
    }

    /* post-processing equivalent expressions */
    // let mut set = HashSet::default();
    // equiv_exprs.retain(|e| set.insert(e.clone()));
    // rm_permutation(&mut equiv_exprs);

    return equiv_exprs;
}

/// ### private function to generate equivalent expressions
/// ### with expressions from an input file
/// #### Argument
/// * `cli` - pre-processed command line arguments
/// #### Return
/// * `None`
fn generate_file(cli: &mut Vec<CmdLineArg>) {
    /* Open the input file and create output file */
    let input_file = match File::options().read(true).write(false).open(&cli[3].to_string()) {
        Ok(input_file) => { input_file },
        Err(e) => {
            log_error(&format!("Failed to open input file \"{}\" with error {}.\n", &cli[3].to_string(), e));
            exit(1);
        },
    };
    let output_file = match File::create(&cli[4].to_string()) {
        Ok(output_file) => { output_file },
        Err(e) => {
            log_error(&format!("Failed to create output file \"{}\" with error {}.\n", &cli[4].to_string(), e));
            exit(1);
        },
    };

    /* Create buffered reader and writer for the input and output files */
    let reader = BufReader::new(&input_file);
    let mut writer = BufWriter::new(&output_file);

    cli.pop();

    for input_expr in reader.lines() {
        /* read 1 expression and write into output file */
        let input_expr = match input_expr {
            Ok(input_expr) => { input_expr },
            Err(e) => {
                log_error(&format!("Error reading line from file with error {}.\n", e));
                exit(1);
            },
        };
        match writeln!(writer, "{}", &input_expr) {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to write input expr {} into output file with error {}.\n", input_expr, e));
                exit(1);
            },
        };

        /* start extraction and get equivalent expressions */
        cli[3] = CmdLineArg::String(input_expr);
        let equiv_exprs = generate_exprs(cli);

        /* write equivalent expressions into output file */
        for expr in &equiv_exprs {
            match writeln!(writer, "{}", expr) {
                Ok(_) => {},
                Err(e) => {
                    log_error(&format!("Failed to write expr {} into output file with error {}.\n", expr, e));
                    exit(1);
                },
            };
        }
        match writeln!(writer, "") {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to write \"\" into output file with error {}.\n", e));
                exit(1);
            },
        };

        /* flush the output stream */
        match writer.flush() {
            Ok(_) => {},
            Err(e) => {
                log_error(&format!("Failed to flush writer with error {}.\n", e));
                exit(1);
            },
        }
    }

    /* flush the output stream */
    match writer.flush() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to flush writer with error {}.\n", e));
            exit(1);
        },
    }

    /* sync all OS-internal metadata to disk */
    match input_file.sync_all() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to sync input file {} with error {}.\n", &cli[3].to_string(), e));
            exit(1);
        },
    }
    match output_file.sync_all() {
        Ok(_) => {},
        Err(e) => {
            log_error(&format!("Failed to sync input file {} with error {}.\n", &cli[4].to_string(), e));
            exit(1);
        },
    }

    /* clean up file descriptors */
    drop(input_file);
    drop(&output_file);

    return;
}

/// ### public function to start generating equivalent expressions
/// #### Argument
/// * `args` - raw command line arguments
/// #### Return
/// * `None`
pub fn generate(args: &Vec<String>) {
    let mut cli = parse_args(&args);

    if cli.len() == 4 {
        let equiv_exprs = generate_exprs(&mut cli);
        for expr in &equiv_exprs {
            log_info(&format!("{}\n", expr));
        }
    }
    else { generate_file(&mut cli); }

    return;
}