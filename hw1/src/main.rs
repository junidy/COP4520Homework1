use rayon::{prelude::*, ThreadPoolBuilder};

const MAX_BOUND: usize = 100_000_000;
const NUM_THREADS: usize = 8;


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
    let mut sieve = vec![true; MAX_BOUND];
    let max_iterations = (MAX_BOUND as f64).sqrt().ceil() as usize;

    use std::time::Instant;
    let now = Instant::now();

    ThreadPoolBuilder::new().num_threads(NUM_THREADS).build().unwrap();

    for i in 2..max_iterations {
        // println!("ITERATION {i}");
        if !sieve[i] { continue }

        let chunk_size = (MAX_BOUND - i) / 8;
        // println!("\tChunk size: {chunk_size}");
        let start_indices: Vec<usize> = (i..sieve.len()).step_by(chunk_size).collect();
        sieve[i..].par_chunks_mut(chunk_size).enumerate().for_each(|(index, chunk)| {
            // print!("*");
            markoff_sieve(chunk, start_indices[index], i)
        });
        sieve[i] = true;
        // print!("\n\n");
        // // println!("Iteration {i}");

        // // i is the next prime to sieve.
        // // Launch threads to mark off 1/8 of the vector
        // pool.install(|| {

        // });

        // for j in ((i * 2)..MAX_BOUND).step_by(i) {
        //     // println!("\tSetting {j} to false");
        //     sieve[j] = false;
        // }
        // for thread_id in (0..7)

        // // Wait until all threads have completed
        // pool.join();

    }

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

    let mut num_primes = 0;
    for index in 2..MAX_BOUND {
        if sieve[index] {
            num_primes += 1;
            // println!("{index}");
        }
    }
    println!("{num_primes}");
}