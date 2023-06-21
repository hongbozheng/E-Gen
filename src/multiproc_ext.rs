use std::env;
use std::io::Read;
// use crate::*;
// use std::env;
// use std::io::Read;
use std::time::Duration;
use std::thread;
use std::net::{TcpStream, SocketAddr};
use std::process;
use egg::{THD_PCT, MAX_RW_LEN, EXHAUSTIVE};
use egg::Data;
use ipc_channel::ipc::{self, IpcReceiver};
use std::error::Error;
use egg::CmdLineArg;
use std::sync::{Arc};
use tokio::sync::mpsc::{self, Receiver};
// use bincode::{serialize, deserialize};

// use crate::set_hyperparam;

// pub fn multiproc_extract(args: Vec<String>) {
//     set_hyperparam(&args);
// }

fn deserialize_data(serialized_data: &[u8]) -> Result<Data, Box<dyn Error>> {
    match bincode::deserialize::<Data>(serialized_data) {
        Ok(data) => Ok(data),
        Err(err) => Err(Box::new(err)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cli: Vec<CmdLineArg> = args.iter().map(|arg| CmdLineArg::from_string(arg).unwrap()).collect();
    println!("{:?}", cli);


    match TcpStream::connect(&args[5]) {
        Ok(mut stream) => {
            println!("Successfully connected to server in {}", args[5]);

            let mut data: Vec<u8> = vec![];

            match stream.read_to_end(&mut data) {
                Ok(_) => {
                    println!("{:?}", data);
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
    println!("Terminated.");


    // // Get the address of the listener passed from the parent process
    // let listener_address: SocketAddr = args[5].parse().expect("[ERROR]: Failed to parse listener address.");
    // // Connect to the parent process
    // let mut stream = TcpStream::connect("127.0.0.1:8000").expect("[ERROR]: Failed to connect to parent process.");
    
    // let mut data_serialized = vec![];
    // stream.read_to_end(&mut data_serialized);

    // let mut data: Data = deserialize_data(&data_serialized).unwrap();

    // // Receive data from the parent process
    // let received_data = receiver.recv().expect("Failed to receive data from parent process.");

    // // Process the received data
    // println!("Received data: {}", received_data);

    let pid = process::id();

    // unsafe {
    //     println!("{}", THD_PCT);
    //     println!("{}", MAX_RW_LEN);
    //     println!("{}", EXHAUSTIVE);
    // }
    

    // Process the data
    println!("PID: {}", pid);
    // thread::sleep(Duration::from_secs(5));
}