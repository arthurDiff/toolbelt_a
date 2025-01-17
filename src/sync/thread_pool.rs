use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<super::Task>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// # Panics
    ///
    /// Will panic if pool size is 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn send<F>(&self, f: F) -> crate::Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let Some(sender) = &self.sender else {
            return Err(crate::Error::SyncError(
                "thread pool does not have sender".into(),
            ));
        };
        sender
            .send(Box::new(f))
            .map_err(crate::Error::as_sync_send_error)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        self.workers.iter_mut().for_each(|w| {
            println!("Shutting down worker {}", w.id);

            if let Some(thread) = w.thread.take() {
                thread.join().expect("Failed joing thread from worker");
            }
        })
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<super::Task>>>) -> Self {
        let thread = thread::spawn(move || {
            while let Ok(receiver) = receiver.lock() {
                match receiver.recv() {
                    Ok(task) => {
                        println!("Worker {id} got a task; executing.");

                        task.run_task()
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
