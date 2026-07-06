//! 01 — Basic threads
//!
//! Spawns several OS threads with `std::thread::spawn`, each doing
//! independent work, then waits for all of them with `.join()`.
//!
//! Run: cargo run --bin 01_threads_basic

use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = Vec::new();

    for id in 0..5 {
        // `move` transfers ownership of `id` into the new thread's closure.
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * (5 - id)));
            println!("worker {id} finished");
            id * id
        });
        handles.push(handle);
    }

    // join() blocks until the thread finishes and gives back its return value
    // (or the panic payload, via Result).
    let mut results = Vec::new();
    for handle in handles {
        match handle.join() {
            Ok(value) => results.push(value),
            Err(e) => eprintln!("a thread panicked: {e:?}"),
        }
    }

    println!("results (in spawn order): {results:?}");
}
