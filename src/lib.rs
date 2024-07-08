
use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

type Job = Box<dyn FnOnce() + Send +'static>;
pub struct ThreadPool{

}

struct Worker {
    id:usize,
    thread:Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id:usize, receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} has a job execution");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} discontinued; shutting down.");
                    break;
                }
            }
        });
        Worker{
            id,
            thread:Some(thread),
        }
    }
}