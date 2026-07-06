//! 05 — Scoped threads
//!
//! `thread::scope` (stable since Rust 1.63) lets spawned threads *borrow*
//! data from the enclosing stack frame instead of requiring `Arc`/`'static`
//! data, because the scope guarantees every spawned thread finishes before
//! the scope block exits.
//!
//! Run: cargo run --bin 05_scoped_threads

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let midpoint = data.len() / 2;
    let (left, right) = data.split_at(midpoint);

    let (left_sum, right_sum) = thread::scope(|s| {
        let left_handle = s.spawn(|| {
            let sum: i32 = left.iter().sum();
            println!("left half {left:?} -> sum {sum}");
            sum
        });

        let right_handle = s.spawn(|| {
            let sum: i32 = right.iter().sum();
            println!("right half {right:?} -> sum {sum}");
            sum
        });

        (left_handle.join().unwrap(), right_handle.join().unwrap())
        // <- all spawned threads are joined by the time `scope` returns
    });

    println!("total (computed back in main): {}", left_sum + right_sum);
}
