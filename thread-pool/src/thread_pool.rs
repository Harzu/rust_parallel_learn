use crate::worker::*;
use crate::message::*;

use std::thread;
use std::sync::{ mpsc, Arc, Mutex };

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("worker {} shut down", worker.id);
      if let Some(thr) = worker.thread.take() {
        thr.join().unwrap();
      }
    }
  }
}

impl ThreadPool {
  pub fn new(count: usize) -> Self {
    assert!(count > 0);
    let mut workers = Vec::with_capacity(count);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    
    for id in 0..count {
      workers.push(Worker::new(id, receiver.clone()));
    }

    ThreadPool {
      workers,
      sender
    }
  }

  pub fn execute<T>(&self, f: T)
    where 
      T: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}