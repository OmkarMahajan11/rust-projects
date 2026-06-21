mod threadpool;

use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    println!("Creating thread pool with 2 workers (bounded channel capacity 4)");
    let pool = ThreadPool::new(2);

    // Submit many long-running jobs to fill the queue and block the main thread
    for i in 0..10 {
        println!("main: submitting job {}", i);
        pool.execute(move || {
            println!("job {}: starting", i);
            thread::sleep(Duration::from_secs(1));
            println!("job {}: finished", i);
        });
        println!("main: job {} accepted", i);
    }

    println!("All jobs submitted.");
}
