use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// Trait object that holds the type of closure that `execute` receives.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new [`ThreadPool`].
    ///
    /// The size parameter specifies the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    // FnOnce because closure can only be executed once
    // Send to transfer closure from one thread to another
    // 'static because we don't know how long the thread will take
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Close the channel, indicating no more messages will be sent; any subsequent calls to
        // `recv` will return an error.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // `join` requires ownership of the thread (it takes `self`), so
            // we needed to wrap the threads in an `Option`, then call `take`, on it, giving us
            // ownership of the value inside, and replacing the option with `None`. If the thread
            // is already None, then there's nothing to do.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock on receiver to acquire mutex
            // wrap to panic on any errors doing so (e.g. another thread panicked while holding
            // lock, poisoning it)
            // recv a Job from the channel, and unwrap to panic on any errors; this blocks if no
            // job yet
            //
            // This goes *inside* the loop instead of while let job = ... because we want to
            // release the lock (because the value from lock() goes out of scope when we call
            // further methods on it to receive a job). If we acquired the lock as _part_ of the
            // loop, we'd block the lock while doing the work, preventing the thread-pool from
            // actually being multithreaded. It's the difference between:
            //
            //   (imaginary scenario: there is no unlock method on Rust Mutex)
            //   let lock = receiver.lock().unwrap();
            //   let job = lock.recv().unwrap();
            //   job();
            //   lock.unlock(); // unlock _after_ doing work
            //
            // and
            //
            //   (pseudo-code for what actually happens)
            //   let lock = receiver.lock.unwrap();
            //   let job = lock.recv().unwrap();
            //   lock.unlock(); // unlock _before_ doing work
            //   job();
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Channel closed; worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
