use super::pool::Message;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let message = {
                let rx = receiver.lock().unwrap();
                rx.recv()
            };

            match message {
                Ok(Message::NewJob(job)) => job(),
                Ok(Message::Terminate) | Err(_) => break,
            }
        });

        Worker {
            handle: Some(handle),
        }
    }

    pub fn join(&mut self) {
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}
