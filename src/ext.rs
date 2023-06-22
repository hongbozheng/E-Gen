use crate::*;
use std::net::{TcpStream, SocketAddr};
use std::io::Read;
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;

/// max # of threads can be used (not max # of OS threads)
pub static mut MAX_NUM_THREADS: Option<Arc<Mutex<u32>>> = None;
/// private global variable to store eclass(es) to skip during extraction
static mut SKIP_ECLS: Option<HashMap<String, f64>> = None;
/// private global variable to store grammar from MathEGraph
static mut GRAMMAR: Option<HashMap<String, Vec<String>>> = None;
/// global variable to store equivalent expression results
pub static mut EQUIV_EXPRS: Option<Arc<Mutex<Vec<String>>>> = None;

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
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        },
    }

    /* setup global SKIP_ECLS, GRAMMAR, & EQUIV_EXPRS variables */
    unsafe {
        SKIP_ECLS = Some(skip_ecls);
        GRAMMAR = Some(grammar);

        let equiv_exprs = Arc::new(Mutex::new(vec![]));
        EQUIV_EXPRS = Some(equiv_exprs);

        println!("{:?}", SKIP_ECLS);
        println!("{:?}", GRAMMAR);
    }

    println!("Terminated.");
    let pid = process::id();
    println!("PID: {}", pid);
}