use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool::new
// pool.execute(job)

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(f) => {
                        println!("worker {} got a job; executing.", id);
                        f()
                    }
                    Err(_) => {
                        println!("worker {} shutting down.", id);
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::SyncSender<Job>>,
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        assert!(n > 0, "ThreadPool must have at least 1 worker");

        let (sender, receiver) = mpsc::sync_channel(2 * n);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(n);
        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
