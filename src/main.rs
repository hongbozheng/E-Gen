use std::env;
use std::io::Write;
use std::net::{TcpListener};
use std::sync::mpsc;
use std::process::{Command, exit};
use std::thread;

use egg::{ContextGrammar, Language, Extractor, AstSize};
/* import hyperparameter set up */
use egg::parse_args;
/* import extraction functions */
use egg::{get_global_skip_ecls, get_global_grammar, get_global_equiv_exprs};
/* import log level & logger functions */
use egg::{log_info, log_info_raw};
/* import refactor function */
use egg::refactor;
/* import utils functions */
use egg::{pt_egraph_info, pt_root_ecls_info, pt_grammar, pt_init_rw, pt_skip_ecls, pt_rw};

use egg::{generate, generate_file};
// fn create_hashmap() -> HashMap<String, String> {
//     let mut hashmap = HashMap::new();
//     hashmap.insert("key1".to_string(), "value1".to_string());
//     hashmap.insert("key2".to_string(), "value2".to_string());
//     hashmap
// }

pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    generate(&args);
    
    exit(0);
    // 

    // // Create buffered reader and writer for the input and output files
    // let reader = BufReader::new(input_file);
    // let mut writer = BufWriter::new(output_file);

    // generate_dataset(input_filename, output_filename);

    // /// Step 1. get the e-graph
    // /// Step 2. saturate the e-graph
    // /// Step 3. get the root-eclass id
    // /// Step 4. get all the root-enodes in root-eclass
    // /// Step 5. create # of processes based on
    // ///         # of root-enodes or # of CPUs
    // /// Step 6. create corresponding # of socket addresses
    // /// Step 7. create connections with children processes multi-threading
    // /// Step 8. send hyperparameters & hashmap to all children processes
    // /// Step 9. check results
    // let args: Vec<String> = env::args().collect();

    // set_hyperparam(&args);

    // // Create the data
    // let data = (
    //     (3.14f32, 2.718f32, 42u8),
    //     create_hashmap(),
    //     "Hello, World!".to_string(),
    // );

    // // Serialize the data
    // let serialized_data = serialize(&data).expect("Failed to serialize data");

    // let num_proc = 4;

    // // Define the socket addresses of the child processes
    // let child_addresses = vec![
    //     "127.0.0.1:8001".parse().unwrap(),
    //     "127.0.0.1:8002".parse().unwrap(),
    //     "127.0.0.1:8003".parse().unwrap(),
    //     "127.0.0.1:8004".parse().unwrap(),
    //     "127.0.0.1:8005".parse().unwrap(),
    //     "127.0.0.1:800".parse().unwrap(),
    //     // Add more socket addresses as needed
    // ];

    // for _ in 0..num_proc {
    //     // Create a TCP listener to accept incoming connections from child processes
    //     let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind listener");

    //     // Get the address of the listener to pass to the child process
    //     let listener_address = listener.local_addr().expect("Failed to get listener address");

    //     // Spawn a child process
    //     let child = Command::new("target/debug/multiproc")
    //         .arg(listener_address.to_string())
    //         .spawn();

    //     match child {
    //         Ok(mut child) => {
    //             // Accept the incoming connection from the child process
    //             // let (mut stream, _) = listener.accept().expect("Failed to accept connection");

    //             // Spawn a thread to send data to the child process
    //             // let data_clone = data.clone();
    //             thread::spawn(move || {
    //                 // Connect to the child process
    //                 let mut stream = TcpStream::connect(listener_address).expect("Failed to connect to child process");

    //                 // Send the serialized data to the child process
    //                 stream.write_all(&serialized_data).expect("Failed to send data");
    //             });

    //             // Wait for the child process to finish
    //             let result = child.wait();
    //             if let Err(e) = result {
    //                 println!("Child process exited with an error: {}", e);
    //             }
    //         }
    //         Err(e) => {
    //             println!("Failed to spawn child process: {}", e);
    //             exit(1);
    //         }
    //     }
    // }
    // exit(0);
    // let init_expr: &str = "(d x (+ (pow x 2) (pow (sin x) 2)))";

    // let mut ctx_gr = ContextGrammar::new(init_expr);
    // log_info("Creating egraph with initial expression & rewrite rules...\n");
    // ctx_gr.set_egraph();

    // let egraph = ctx_gr.get_egraph();
    // log_info_raw("\n");
    // log_info(format!("EGraph total size {}\n", egraph.total_size()).as_str());
    // log_info(format!("EGraph contains {} node(s)\n", egraph.total_number_of_nodes()).as_str());
    // log_info(format!("EGraph contains {} eclass(es)\n", egraph.number_of_classes()).as_str());

    // /* TODO: DEBUG */
    // // pt_egraph_info(&egraph);

    // let root_ecls = ctx_gr.get_root_ecls();
    // pt_root_ecls_info(&root_ecls);

    // /* TODO: DEBUG */
    // // log_debug_raw("\n");
    // // log_debug("------------ Extractor -----------\n");
    // // let extractor = Extractor::new(&egraph, AstSize);
    // // let (best_cost, simpl_expr) = extractor.find_best(root_ecls[0]);
    // // log_debug(format!("Simplified Expression to {} with Cost {}\n",simpl_expr, best_cost).as_str());
    // // log_debug("----------------------------------\n");

    // unsafe {
    //     log_info_raw("\n");
    //     log_info("Creating grammar & setting initial rewrite...\n");
    //     setup_extract(&mut ctx_gr);

    //     let skip_ecls = get_global_skip_ecls();
    //     pt_skip_ecls(skip_ecls);

    //     let grammar = get_global_grammar();
    //     pt_grammar(grammar);

    //     log_info_raw("\n");
    //     log_info(format!("Total # of grammar {}\n", grammar.len()).as_str());
    // }

    // let init_rw = ctx_gr.get_init_rw();
    // pt_init_rw(init_rw);
    // log_info_raw("\n");
    // log_info(format!("Total # of initial rw {}\n", init_rw.len()).as_str());
    // unsafe { extract(init_rw.clone());}

    // unsafe {
    //     let mutex = get_global_equiv_exprs();
    //     pt_rw(mutex);
    // }
}