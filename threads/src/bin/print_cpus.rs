use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    // Get number of logical processors available (std since Rust 1.59+).
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    println!("Detected {} logical CPU(s). Spawning {} thread(s).", cpus, cpus);

    // Spawn one thread per CPU and collect join handles.
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(cpus);

    for i in 0..cpus {
        // Capture i into thread closure
        let handle = thread::spawn(move || {
            // Do any per-thread work here. We print a message, including the OS thread id.
            let tid = thread::current().id();
            println!("Thread #{:2} started (OS thread id = {:?})", i, tid);

            // Simulate some work so prints don't all interleave too tightly.
            // Replace with actual work as needed.
            thread::sleep(Duration::from_millis(50 + (i as u64) * 10));

            println!("Thread #{:2} finished.", i);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    println!("All threads completed.");
}
