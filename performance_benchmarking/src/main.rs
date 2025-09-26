use std::time::Instant;

fn main() {
    let n: u64 = 1_000_000_000;

    let start = Instant::now();
    let mut sum: u64 = 0;

    for i in 1..=n {
        sum += i;
    }

    let duration = start.elapsed();

    println!("Sum from 1 to {} = {}", n, sum);
    println!("Execution time: {:?}", duration);
}
