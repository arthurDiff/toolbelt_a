use std::{
    sync::{atomic::AtomicUsize, mpsc},
    thread,
};

use super::Message;

static WORKER_THREAD_SEQ: AtomicUsize = AtomicUsize::new(0);

pub struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Message>>,
}

impl Default for Worker {
    /// Create a default Worker
    ///
    /// # Panics
    ///
    /// `default` func will panic if failed creating new thread
    fn default() -> Self {
        let (thread, sender) = Self::prep(None)
            .unwrap_or_else(|err| panic!("toolbelt_a failed creating worker with err: {}", err));
        Self {
            thread: Some(thread),
            sender: Some(sender),
        }
    }
}

impl Worker {
    /// Create a new Worker
    ///
    /// thread name can be provided
    pub fn new(name: Option<String>) -> crate::Result<Self> {
        let (thread, sender) = Self::prep(name)?;
        Ok(Self {
            thread: Some(thread),
            sender: Some(sender),
        })
    }

    pub fn send<F>(&self, f: F) -> crate::Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let Some(sender) = &self.sender else {
            return Err(crate::Error::SyncError(
                "worker does not have sender".into(),
            ));
        };
        sender
            .send(Message::NewTask(Box::new(f)))
            .map_err(crate::Error::as_sync_send_error)
    }

    fn prep(
        name: Option<String>,
    ) -> crate::Result<(thread::JoinHandle<()>, mpsc::Sender<Message>)> {
        let (sender, receiver) = mpsc::channel::<Message>();
        let thread = thread::Builder::new()
            .name(name.unwrap_or(format!(
                "toolbelt_a worker {}",
                WORKER_THREAD_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            )))
            .spawn(move || {
                while let Ok(msg) = receiver.recv() {
                    match msg {
                        Message::NewTask(task) => {
                            task.run_task();
                        }
                        Message::Terminate => break,
                    }
                }
            })
            .map_err(crate::Error::SyncThreadError)?;

        Ok((thread, sender))
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            sender.send(Message::Terminate).unwrap_or_else(|err| {
                panic!("worker failed sending termination msg with error: {}", err)
            });
            drop(sender)
        }

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap_or_else(|err| {
                panic!("worker failed joining thread on drop with error: {:?}", err)
            })
        }
    }
}
