//! 03 — Message passing with channels
//!
//! Instead of sharing memory, worker threads *send* their results to a
//! central receiver over an `mpsc` (multi-producer, single-consumer) channel.
//! "Do not communicate by sharing memory; share memory by communicating."
//!
//! Run: cargo run --bin 03_channels

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for worker_id in 0..4 {
        let tx = tx.clone(); // each worker gets its own sending handle
        thread::spawn(move || {
            for i in 0..3 {
                let msg = format!("worker {worker_id} says hello #{i}");
                tx.send(msg).expect("receiver dropped");
            }
        });
    }

    // Drop the original sender so the channel closes once all workers finish.
    drop(tx);

    // rx behaves like an iterator: it yields values until every Sender is gone.
    for received in rx {
        println!("main received: {received}");
    }

    println!("channel closed, all workers done");
}
