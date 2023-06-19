use crate::*;
use libc::{c_int, cpu_set_t, CPU_SET, CPU_SETSIZE, sched_setaffinity, pid_t, listen};
use num_cpus;
use std::mem;
use std::net::TcpListener;
use std::process::{Command, exit, Stdio};
use std::thread;
// use thread_affinity::ThreaadAffinity;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use bincode::{serialize, deserialize};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    skip_ecls: HashMap<String, f64>,
    grammar: HashMap<String, Vec<String>>,
}

fn set_proc_affinity(pid: pid_t, processor_id: usize) -> Result<(), String> {
    let mut cpuset: cpu_set_t = unsafe { mem::zeroed() };

    unsafe {
        if processor_id >= CPU_SETSIZE as usize {
            return Err(format!("Invalid core ID: {}", processor_id));
        }

        CPU_SET(processor_id, &mut cpuset);

        let ret = sched_setaffinity(pid, mem::size_of::<cpu_set_t>(), &cpuset);

        if ret == 0 {
            Ok(())
        } else {
            Err("Failed to set process affinity".to_owned())
        }
    }
}

fn deserialize_data(serialized_data: &[u8]) -> Result<Data, Box<dyn Error>> {
    match bincode::deserialize::<Data>(serialized_data) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}

fn send_data(skip_ecls: &HashMap<String, f64>, grammar: &HashMap<String, Vec<String>>) {    
    let data: Data = Data {
        skip_ecls: skip_ecls.clone(),
        grammar: grammar.clone(),
    };

    let serialized_data = bincode::serialize(&data).unwrap();
    // println!("{:?}", serialized_data);


    let data: Result<Data, Box<dyn std::error::Error>> = Ok(deserialize_data(&serialized_data).unwrap());
    // println!("{:?}", data);
}

pub fn generate_expr(cli: &mut Vec<CmdLineArg>) {
    let mut expr: &str = "";
    if let CmdLineArg::String(init_expr) = &cli[3] {
        expr = init_expr;
    }
    log_info(format!("Expression: {}\n", expr).as_str());
    let mut ctx_gr = ContextGrammar::new(&expr);

    /* create egraph, skip_ecls, grammar, init_rewrite */
    ctx_gr.setup();

    let skip_ecls = Arc::new(ctx_gr.skip_ecls.clone());
    let grammar = Arc::new(ctx_gr.grammar.clone());
    let init_rw = &ctx_gr.init_rw.clone();
    println!("{:?}", init_rw);
    println!("{}", init_rw.len());

    let num_proc = init_rw.len();
    let num_logical_cores = num_cpus::get();
    cli.push(CmdLineArg::String("".to_string()));

    let handles: Vec<_> = (0..num_proc).map(|proc_idx| {
        let addr = format!("127.0.0.1:{}", 8000 + proc_idx);
        let listener = TcpListener::bind(&addr)
            .expect(format!("[ERROR]: Failed to bind TCP listener with address {}.", &addr).as_str());
        let listener_addr = listener.local_addr().expect("[ERROR]: Failed to get local address.");

        let processor_id = proc_idx % num_logical_cores;

        cli[4] = CmdLineArg::String(listener_addr.to_string());
        let args: Vec<String> = cli.iter().map(|arg| arg.to_string()).collect();
        let skip_ecls_clone = Arc::clone(&skip_ecls);
        let grammar_clone = Arc::clone(&grammar);

        thread::spawn(move || {
            let mut child_proc = Command::new("../target/debug/multiproc").args(&args).spawn()
                .expect("[ERROR]: Failed to spawn process.");

            let pid = child_proc.id();

            if let Err(err) = set_proc_affinity(pid as pid_t, processor_id) {
                println!("Error setting affinity for process {}: {}", pid, err);
            } else {
                println!("Process {} affinity set to core {}", pid, processor_id);
            }

            send_data(&skip_ecls_clone, &grammar_clone);

            let exit_status = child_proc.wait().expect("Failed to wait for child process.");
            let exit_code = exit_status.code().unwrap_or(1);
            println!("Process {} finished with exit code {}", pid, exit_code);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Generate Finished");
}

pub fn generate_file(input_filename: &str, output_filename: &str) {
    // Open the input file and create output file
    let input_file = File::open(input_filename)
        .expect(format!("[ERROR]: Failed to open input file \"{}\".", input_filename).as_str());
    let output_file = File::create(output_filename)
        .expect(format!("[ERROR]: Failed to create output file \"{}\".", output_filename).as_str());

    // Create buffered reader and writer for the input and output files
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for expr in reader.lines() {
        let expr = expr.expect("[ERROR]: Error reading line from file.");

        log_info(format!("Expression: {}\n", expr).as_str());
        let mut ctx_gr = ContextGrammar::new(&expr);

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