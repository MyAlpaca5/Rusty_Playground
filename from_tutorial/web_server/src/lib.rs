#![allow(dead_code)]

use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(num_of_workers: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(num_of_workers);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..num_of_workers {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // manually clean up all workers
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("{} is working", id);
                    job();
                }
                Err(_) => {
                    println!("{} is shutting down", id);
                    break;
                }
            }

            // it's tempting to write code like following,
            // but it will not work as expected, because how Mutex release
            // the lock. More info: https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {id} got a job; executing.");

            //     job();
            // }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
