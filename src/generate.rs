use crate::*;
use libc::{c_int, cpu_set_t, CPU_SET, CPU_SETSIZE, sched_setaffinity, sched_getaffinity, pid_t, listen, CPU_ISSET, CPU_ZERO};
use num_cpus;
use std::mem::{zeroed, size_of};
use std::net::{TcpListener, SocketAddrV6};
use std::process::{Command, exit, Stdio, Child};
use std::thread;
// use thread_affinity::ThreaadAffinity;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::error::Error;

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
pub fn generate_expr(cli: &mut Vec<CmdLineArg>) {
    /* initialize ctx_gr struct */
    let expr = cli[3].to_string();
    log_info(&format!("Expression: {}\n", expr));
    let mut ctx_gr = ContextGrammar::new(expr);

    /* create egraph, skip_ecls, grammar, init_rewrite */
    ctx_gr.setup();
    let init_rw = &ctx_gr.init_rw.clone();
    println!("{:?}", init_rw);
    println!("{}", init_rw.len());

    /* get number of processes */
    let num_proc = init_rw.len();

    let tx_addr = "127.0.0.1:8080";
    let tx_listener = TcpListener::bind(&tx_addr).unwrap_or_else(|_| {
        log_error(&format!("[ERROR]: Failed to bind IP address \"{}\"\n.", tx_addr));
        exit(1)
    });
    let rx_addr = "127.0.0.1:8081";
    let rx_listener = TcpListener::bind(&rx_addr).unwrap_or_else(|_| {
        log_error(&format!("[ERROR]: Failed to bind IP address \"{}\"\n.", rx_addr));
        exit(1)
    });

    /* bind the parent process to tcp ports */
    // let tcp_listeners: Vec<TcpListener> = (0..num_proc).map(|proc_idx| {
    //     let addr = format!("127.0.0.1:{}", 8000 + proc_idx);
    //     TcpListener::bind(&addr).unwrap_or_else(|_| {
    //         log_error(&format!("[ERROR]: Failed to bind IP address \"{}\"\n.", addr));
    //         exit(1)
    //     })
    // }).collect();

    /* insert socket address & get CPU's number of logical cores */
    cli.push(CmdLineArg::String("127.0.0.1:8080".to_string()));
    let num_logical_cores = num_cpus::get();

    /* spawn children processes & set process affinity */
    let mut child_procs: Vec<Child> = init_rw.into_iter().zip(0..num_proc).map(|(rw, proc_idx)| {
        // let addr = format!("127.0.0.1:{}", 8000 + proc_idx);
        cli[3] = CmdLineArg::String(rw.clone());
        // cli[4] = CmdLineArg::String(addr.clone());
        let args: Vec<String> = cli.iter().map(|arg| arg.to_string()).collect();

        let child_proc = Command::new("../target/debug/multiproc")
                                    .args(&args)
                                    .spawn()
                                    .unwrap_or_else(|_| {
                                        log_error("[ERROR]: Failed to spawn child process.");
                                        exit(1);
                                    });

        let pid = child_proc.id() as pid_t;
        let processor_id = proc_idx % num_logical_cores;
        let ret = unsafe { set_proc_affinity(pid, processor_id) };
        match ret {
            0 => {
                log_debug(&format!("Set process {}'s process affinity to processor {}.\n", pid, processor_id));
            },
            _ => {
                log_error(&format!("Failed to set process {}'s process affinity to processor {}.\n", pid, processor_id));
                exit(1);
            },
        }

        child_proc
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
            println!("incoming {:?}", tx_listener.incoming());

            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let skip_ecls = ctx_gr.skip_ecls.clone();
                    let grammar = ctx_gr.grammar.clone();

                    let data: Data = Data {
                        skip_ecls,
                        grammar,
                    };
                
                    let data_serialized = bincode::serialize(&data).unwrap();
                    match stream.write_all(&data_serialized) {
                        Ok(_) => {
                            num_acks += 1;
                            log_debug(&format!("Data send to child process {:?} successfully.\n", stream.peer_addr()));
                        },
                        Err(e) => {
                            log_error(&format!("Failed to data to child process {:?} with error {}.\n", stream.peer_addr(), e));
                            exit(1);
                        },
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }

            if num_acks as usize == num_proc { println!("break le"); break; }
        }
    });

    handle.join().unwrap();

    num_acks = 0u8;
    let mut equiv_exprs: Vec<String> = vec![];

    for stream in rx_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut equiv_exprs_proc: Vec<u8> = vec![];
                match stream.read_to_end(&mut equiv_exprs_proc) {
                    Ok(_) => {
                        num_acks += 1;
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        log_error(&format!("Failed to receive data from child process with error {}", e));
                    }
                }

                let mut equiv_exprs_proc: Vec<String> = bincode::deserialize(&equiv_exprs_proc).unwrap();
                // println!("final results received {:?}", equiv_exprs_proc);
                equiv_exprs.append(&mut equiv_exprs_proc);
                // });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
        if num_acks as usize == num_proc { println!("break le"); break; }
        println!("{}", num_acks);
    }

    // for child_proc in &mut child_procs {
    //     let pid = child_proc.id();
    //     child_proc.wait().expect(&format!("[ERROR]: Failed to wait for processor {}.\n", pid));
    //     let exit_status = child_proc.wait().expect("Failed to wait for child process.");
    //     let exit_code = exit_status.code();

    //     if let Some(exit_code) = exit_code {
    //         match exit_code {
    //             0 => { log_debug(&format!("Child process {} terminated successfully with an exit code {}.\n", pid, exit_code)); },
    //             _ => { log_error(&format!("Child process {} terminated with a non-zero exit code {}.\n", pid, exit_code)); },
    //         }
    //     }
    // }

    for expr in equiv_exprs {
        log_info(&format!("{}\n", expr));
    }

    println!("Generate Finished");
}

pub fn generate_file(input_filename: &str, output_filename: &str) {
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
    println!("{:?}", cli);
    if cli.len() == 4 {
        generate_expr(&mut cli);
    } 
    // else {
    //     let input_filename = cli.get("input_filename").unwrap();
    //     let output_filename = cli.get("output_filename").unwrap();
    //     generate_file(input_filename, output_filename);
    // }
}