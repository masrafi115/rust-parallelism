//! 02 — Shared state across threads
//!
//! Demonstrates two ways to share and mutate state safely across threads:
//!   1. `Arc<Mutex<T>>`  — for arbitrary shared data.
//!   2. `Arc<AtomicUsize>` — a lock-free counter, cheaper for simple numbers.
//!
//! Run: cargo run --bin 02_shared_state

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // --- Arc<Mutex<T>>: shared, lockable counter ---
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // lock() blocks until the mutex is free, returns a guard
                let mut guard = counter.lock().unwrap();
                *guard += 1;
                // guard is dropped here, releasing the lock
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Mutex counter: {}", *counter.lock().unwrap());

    // --- Arc<AtomicUsize>: lock-free counter ---
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let atomic_counter = Arc::clone(&atomic_counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                atomic_counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("Atomic counter: {}", atomic_counter.load(Ordering::SeqCst));
}
