// const MAX_BOUND: usize = 100_000_000;
const MAX_BOUND: usize = 1000;

fn main() {
    let mut sieve = vec![true; MAX_BOUND];
    let max_iterations = (MAX_BOUND as f64).sqrt() as usize;
    for i in 2..max_iterations {
        // println!("Iteration {i}");
        if !sieve[i] { continue }
        for j in ((i * 2)..MAX_BOUND).step_by(i) {
            // println!("\tSetting {j} to false");
            sieve[j] = false;
        }
    }
}