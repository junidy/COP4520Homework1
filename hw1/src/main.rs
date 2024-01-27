use std::time::Duration;

use rayon::{prelude::*, ThreadPoolBuilder};

const MAX_BOUND: usize = 100_000_000;
const NUM_THREADS: usize = 8;


fn test_serial_sieve(sieve: &mut [bool]) -> Duration {
    use std::time::Instant;
    let now = Instant::now();

    let max_iterations = (MAX_BOUND as f64).sqrt() as usize;

    for i in 2..max_iterations {
        // println!("Iteration {i}");
        if !sieve[i] { continue }
        for j in ((i * 2)..MAX_BOUND).step_by(i) {
            // println!("\tSetting {j} to false");
            sieve[j] = false;
        }
    }

    return now.elapsed();
}

fn test_parallel_sieve(sieve: &mut [bool]) -> Duration {
    use std::time::Instant;
    let now = Instant::now();

    let max_iterations = (MAX_BOUND as f64).sqrt().ceil() as usize;
    ThreadPoolBuilder::new().num_threads(NUM_THREADS).build().unwrap();

    for i in 2..max_iterations {
        if !sieve[i] { continue }

        let chunk_size = (MAX_BOUND - i) / 8;
        let start_indices: Vec<usize> = (i..sieve.len()).step_by(chunk_size).collect();
        sieve[i..].par_chunks_mut(chunk_size).enumerate().for_each(|(index, chunk)| {
            markoff_sieve(chunk, start_indices[index], i)
        });
        sieve[i] = true;
    }

    return now.elapsed();
}

fn markoff_sieve(sieve: &mut [bool], slice_start: usize, step_size: usize) {
    let start_index = if slice_start % step_size == 0 {
        0
    } else {
        step_size - (slice_start % step_size)
    };
    for index in (start_index..sieve.len()).step_by(step_size) {
        sieve[index] = false;
    }
    // for element in sieve.iter().skip(start_index).step_by(step_size) {
    //     *element = false;
    // }
}

fn main() {
    println!("Finding all primes between 2 and {MAX_BOUND}\n");

    println!("Serial algorithm");
    let mut sieve = vec![true; MAX_BOUND];
    let serial_time = test_serial_sieve(&mut sieve);
    println!("\tElapsed time: {:.2?}", serial_time);

    println!("Parallel algorithm");
    let mut sieve = vec![true; MAX_BOUND];
    let parallel_time = test_parallel_sieve(&mut sieve);
    println!("\tElapsed time: {:.2?}", parallel_time);
    
    let speedup = (serial_time.as_millis() as f64) / (parallel_time.as_millis() as f64) * 100.0;
    println!("Speedup of {:.2}%\n", speedup);

    let mut num_primes = 0;
    let mut sum = 0;
    let mut ten_highest = Vec::new();
    let mut num_highest = 10;
    for index in (2..MAX_BOUND).rev() {
        if sieve[index] {
            if num_highest > 0 {
                ten_highest.push(index);
                num_highest -= 1;
            }
            num_primes += 1;
            sum += index;
        }
    }
    println!("Number of primes found: {num_primes}");
    println!("Sum of primes found: {sum}");

    println!("10 highest primes found: ");
    println!("{:?}", ten_highest);
}