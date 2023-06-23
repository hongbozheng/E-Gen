// /// ## function to perform rewrite extraction from egraph
// /// ## Argument
// /// * `csg` - context-sentitive grammar flag
// /// ## Return
// /// * `None`
// pub unsafe fn ssextract(init_rw: Vec<String>) {
//     log_info_raw("\n");
//     match EXHAUSTIVE {
//         true => {
//             let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
//             let mutex = global_max_num_threads.lock().unwrap();
//             log_info(&format!("MAX NUM THREADS {}\n", mutex));
//             drop(mutex);
//             log_info("Start multithreaded context-sensitive grammar extraction...\n");

//             let handles: Vec<_> = init_rw.into_iter().map(|rw| {
//                 thread::Builder::new().name(rw.clone()).spawn(move || {
//                     log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
//                     exhaustive_extract(rw, 0);
//                 }).unwrap()
//             }).collect();

//             log_info("Waiting for all threads to finish execution...\n");
//             for handle in handles {
//                 handle.join().unwrap();
//             }

//             log_info_raw("\n");
//             log_info("Finish context-sensitive grammar extraction\n");
//         },
//         false => {
//             let global_max_num_threads = MAX_NUM_THREADS.as_ref().unwrap();
//             let mutex = global_max_num_threads.lock().unwrap();
//             log_info(&format!("MAX NUM THREADS {}\n", mutex));
//             drop(mutex);
//             log_info("Start multithreaded context-free grammar extraction...\n");

//             let handles: Vec<_> = init_rw.into_iter().map(|rw| {
//                 thread::Builder::new().name(rw.clone()).spawn(move || {
//                     log_debug(format!("Extracting initial rewrite {} in a thread...\n", rw).as_str());
//                     optimized_extract(rw, 0);
//                 }).unwrap()
//             }).collect();

//             log_info("Waiting for all threads to finish execution...\n");
//             for handle in handles {
//                 handle.join().unwrap();
//             }

//             log_info_raw("\n");
//             log_info("Finish context-free grammar extraction\n");
//         },
//     }
// }