use std::env;
use egg::extract;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    extract(&args);

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

    

    // unsafe {
    //     println!("{}", THD_PCT);
    //     println!("{}", MAX_RW_LEN);
    //     println!("{}", EXHAUSTIVE);
    // }
    

    // Process the data
    // thread::sleep(Duration::from_secs(5));
}