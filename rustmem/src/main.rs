use std::{
    env::args,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

const DEFAULT_THREADS: usize = 4;
const DEFAULT_MEMORY_SIZE_GB: usize = 4;

fn stress_memory(terminate_flag: Arc<AtomicBool>, memory: Arc<Mutex<Vec<u128>>>) {
    while !terminate_flag.load(Ordering::Relaxed) {
        // introduce some memory-intensive operations here
        let mut memory = memory.lock().unwrap();
        for element in memory.iter_mut() {
            *element = 0;
        }
    }
}

fn main() {
    let memory_size_gb = args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_MEMORY_SIZE_GB);

    let threads = args()
        .nth(2)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_THREADS);

    let terminate_flag = Arc::new(AtomicBool::new(false));

    // spawn threads with separate memory vectors
    let handles: Vec<_> = (0..threads)
        .map(|i| {
            let terminate_flag = Arc::clone(&terminate_flag);
            let memory: Arc<Mutex<Vec<u128>>> = Arc::new(Mutex::new(vec![
                0;
                memory_size_gb
                    * 1024
                    * 1024
                    * 1024
                    / 16        // 128 / 8
                    / threads
            ]));

            println!("spawning thread {} size {}", i, memory.lock().unwrap().len());
                
            thread::spawn(move || {
                stress_memory(terminate_flag, memory);
            })
        })
        .collect();

    // wait for user to press Enter
    println!("press Enter to terminate...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    // set the terminate flag to true
    terminate_flag.store(true, Ordering::Relaxed);

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
