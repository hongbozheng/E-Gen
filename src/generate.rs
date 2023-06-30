use crate::*;
use bincode::{serialize, deserialize};
use libc::{c_int, cpu_set_t, CPU_SET, pid_t, sched_setaffinity};
use num_cpus;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::mem::{zeroed, size_of};
use std::net::TcpListener;
use std::process::{Child, Command, exit};
use std::thread;

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

/// ### public function to start extraction of a single expression
/// #### Argument
/// * `cli` - pre-processed command line arguments
/// #### Return
/// * `None`
fn generate_exprs(cli: &mut Vec<CmdLineArg>) {
    /* initialize ctx_gr struct and create egraph, skip_ecls, grammar, init_rewrite */
    let expr = cli[3].to_string();
    log_info(&format!("Expression: {}\n", expr));
    let mut ctx_gr = ContextGrammar::new(expr);
    ctx_gr.setup();
    let init_rw = &ctx_gr.init_rw.clone();

    /* get number of processes */
    let num_proc = init_rw.len();

    /* tx listener */
    let tx_addr = "127.0.0.1:8080";
    let tx_listener = match TcpListener::bind(&tx_addr) {
        Ok(tx_listener) => { tx_listener },
        Err(e) => {
            log_error(&format!("[ERROR]: Failed to bind IP address \"{}\" with error {}.\n", tx_addr, e));
            exit(1);
        },
    };

    /* rx listener */
    let rx_addr = "127.0.0.1:8081";
    let rx_listener = match TcpListener::bind(&rx_addr) {
        Ok(rx_listener) => { rx_listener },
        Err(e) => {
            log_error(&format!("Failed to bind IP address \"{}\" with error {}.\n", rx_addr, e));
            exit(1);
        },
    };

    /* insert socket address & get CPU's number of logical cores */
    cli.push(CmdLineArg::String(tx_addr.to_string()));
    cli.push(CmdLineArg::String(rx_addr.to_string()));
    let num_logical_cores = num_cpus::get();

    /* spawn children processes & set process affinity */
    let mut child_procs: Vec<Child> = init_rw.into_iter().zip(0..num_proc).map(|(rw, proc_idx)| {
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
                log_error(&format!("[ERROR]: Failed to spawn child process with error {}.\n", e));
                exit(1);
            },
        }
    }).collect();

    let mut num_acks: u8 = 0u8;

    /* send data to all children processes through sockets */
    let handle = thread::spawn(move || {
        match tx_listener.set_nonblocking(true) {
            Ok(_) => { log_debug("Non-blocking mode set successfully.\n"); },
            Err(e) => {
                log_error(&format!("Failed to set non-blocking mode with error {}.\n", e));
                exit(1);
            },
        }

        for stream in tx_listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let skip_ecls = ctx_gr.skip_ecls.clone();
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
    });

    handle.join().unwrap();

    num_acks = 0u8;
    let mut equiv_exprs: Vec<String> = vec![];

    /* receive equivalent expressions from all children processes */
    for stream in rx_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut equiv_exprs_bytes: Vec<u8> = vec![];
                match stream.read_to_end(&mut equiv_exprs_bytes) {
                    Ok(_) => {
                        let mut equiv_exprs_proc = match deserialize(&equiv_exprs_bytes) {
                            Ok(equiv_exprs_proc) => { equiv_exprs_proc },
                            Err(e) => {
                                log_error(&format!("Failed to deserialize data received from child process with error {}.\n", e));
                                exit(1);
                            },
                        };
                        equiv_exprs.append(&mut equiv_exprs_proc);
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
        if num_acks as usize == num_proc { break; }
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
    let mut set = HashSet::default();
    equiv_exprs.retain(|e| set.insert(e.clone()));
    for expr in equiv_exprs {
        log_info(&format!("{}\n", expr));
    }

    return;
}

fn generate_file(input_filename: &str, output_filename: &str) {
    // Open the input file and create output file
    let input_file = File::open(input_filename)
        .expect(&format!("[ERROR]: Failed to open input file \"{}\".", input_filename));
    let output_file = File::create(output_filename)
        .expect(&format!("[ERROR]: Failed to create output file \"{}\".", output_filename));

    // Create buffered reader and writer for the input and output files
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for expr in reader.lines() {
        let expr = expr.expect("[ERROR]: Error reading line from file.");

        log_info(&format!("Expression: {}\n", expr));
        let mut ctx_gr = ContextGrammar::new(expr);

        /* create egraph, skip_ecls, grammar, init_rewrite */
        ctx_gr.setup();

        let root_ecls = &ctx_gr.root_ecls.clone();
        println!("{:?}", root_ecls);

        /* TODO: Start multiprocessing here */
        // Step 3. get the root-eclass id
        // Step 4. get all the root-enodes in root-eclass
        // Step 5. create # of processes based on
        //         # of root-enodes or # of CPUs
        // Step 6. create corresponding # of socket addresses
        // Step 7. create connections with children processes multi-threading
        // Step 8. send hyperparameters & hashmap to all children processes
    }
}

pub fn generate(args: &Vec<String>) {
    let mut cli = parse_args(&args);

    if cli.len() == 4 {
        generate_exprs(&mut cli);
    } 
    // else {
    //     let input_filename = cli.get("input_filename").unwrap();
    //     let output_filename = cli.get("output_filename").unwrap();
    //     generate_file(input_filename, output_filename);
    // }
    return;
}