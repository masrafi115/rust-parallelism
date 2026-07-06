//! 04 — Data parallelism with Rayon
//!
//! `rayon` gives you parallel iterators: change `.iter()` to `.par_iter()`
//! and the work is automatically split across a thread pool. Great for
//! CPU-bound, data-parallel work like map/filter/reduce over large collections.
//!
//! Run: cargo run --release --bin 04_rayon_data_parallel

use rayon::prelude::*;
use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let numbers: Vec<u64> = (0..2_000_000).collect();

    // Sequential baseline
    let start = Instant::now();
    let seq_count = numbers.iter().filter(|&&n| is_prime(n)).count();
    let seq_time = start.elapsed();

    // Parallel version — same logic, just .par_iter() instead of .iter()
    let start = Instant::now();
    let par_count = numbers.par_iter().filter(|&&n| is_prime(n)).count();
    let par_time = start.elapsed();

    println!("primes below 2,000,000: {seq_count} (sequential should match: {par_count})");
    println!("sequential time: {seq_time:?}");
    println!("parallel time:   {par_time:?}");
    println!(
        "speedup: {:.2}x (threads available: {})",
        seq_time.as_secs_f64() / par_time.as_secs_f64(),
        rayon::current_num_threads()
    );

    // A quick parallel map + sum example too:
    let sum_of_squares: u64 = numbers.par_iter().map(|&n| n * n % 1_000_000_007).sum();
    println!("sum of squares (mod 1e9+7): {sum_of_squares}");
}
