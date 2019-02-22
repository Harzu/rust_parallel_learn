use crate::message::Message;

use std::thread;
use std::sync::{ mpsc, Arc, Mutex };

pub struct Worker {
  pub id: usize,
  pub thread: Option<thread::JoinHandle<()>>
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
    let thread = thread::spawn(move || {
      loop {
        let message = receiver.lock().unwrap().recv().unwrap();
        
        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job executing", id);
            job.box_call();
          },
          Message::Terminate => {
            println!("Worker {} terminate", id);
            break
          }
        }
      }
    });

    Worker {
      id,
      thread: Some(thread)
    }
  }
}