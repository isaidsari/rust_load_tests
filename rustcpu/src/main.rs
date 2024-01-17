use std::{
    env::args,
    io::stdin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

const DEFAULT_THREADS: usize = 4;

fn simulate_cpu_load(terminate_flag: Arc<AtomicBool>) {
    while !terminate_flag.load(Ordering::Relaxed) {
        // introduce some CPU-intensive computation here
        for _ in 0..10_000 {
            let _ = 4.0 * 3.0; // example calculation
        }
    }
}

fn main() {
    let threads = args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_THREADS);

    let terminate_flag = Arc::new(AtomicBool::new(false));

    // spawn threads
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let terminate_flag = Arc::clone(&terminate_flag);
            thread::spawn(move || {
                simulate_cpu_load(terminate_flag);
            })
        })
        .collect();

    // wait for user to press Enter
    println!("press enter to terminate...");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read input");

    // set the terminate flag to true
    terminate_flag.store(true, Ordering::Relaxed);

    // wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
