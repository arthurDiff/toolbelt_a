use std::{
    sync::{atomic::AtomicUsize, mpsc},
    thread,
};

use super::{Message, Task};

static RESULTWORKER_THREAD_SEQ: AtomicUsize = AtomicUsize::new(0);

type ResultWorkerPrepReturns<R> = crate::Result<(
    mpsc::Sender<Message<Task<R>>>,
    mpsc::Receiver<R>,
    thread::JoinHandle<()>,
)>;

pub struct ResultWorker<R: Send + 'static> {
    sender: Option<mpsc::Sender<Message<Task<R>>>>,
    receiver: Option<mpsc::Receiver<R>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl<R: Send + 'static> ResultWorker<R> {
    pub fn new(name: Option<String>) -> crate::Result<Self> {
        let (sender, receiver, thread) = Self::prep(name)?;
        Ok(Self {
            sender: Some(sender),
            receiver: Some(receiver),
            thread: Some(thread),
        })
    }

    pub fn send<F>(&self, f: F) -> crate::Result<()>
    where
        F: FnOnce() -> R + Send + 'static,
    {
        let Some(sender) = &self.sender else {
            return Err(crate::Error::SyncError(
                "result worker doesnot have sender".into(),
            ));
        };
        sender
            .send(Message::NewTask(Box::new(f)))
            .map_err(crate::Error::as_sync_send_error)
    }

    pub fn recv(&self) -> crate::Result<R> {
        let Some(receiver) = &self.receiver else {
            return Err(crate::Error::SyncError(
                "result worker doesnot have receiver".into(),
            ));
        };
        receiver.recv().map_err(crate::Error::SyncRecvError)
    }

    pub fn try_recv(&self) -> crate::Result<R> {
        let Some(receiver) = &self.receiver else {
            return Err(crate::Error::SyncError(
                "result worker doesnot have receiver".into(),
            ));
        };
        receiver.try_recv().map_err(crate::Error::SyncTryRecvError)
    }

    fn prep(name: Option<String>) -> ResultWorkerPrepReturns<R> {
        let (outer_sender, inner_receiver) = mpsc::channel::<Message<Task<R>>>();
        let (inner_sender, outer_receiver) = mpsc::channel::<R>();
        let thread = thread::Builder::new()
            .name(name.unwrap_or(format!(
                "toolbelt_a result worker {}",
                RESULTWORKER_THREAD_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            )))
            .spawn(move || {
                while let Ok(msg) = inner_receiver.recv() {
                    match msg {
                        Message::NewTask(task) => {
                            _ = inner_sender.send(task.run_task());
                        }
                        Message::Terminate => break,
                    }
                }
            })
            .map_err(crate::Error::SyncThreadError)?;

        Ok((outer_sender, outer_receiver, thread))
    }
}

impl<R: Send + 'static> Drop for ResultWorker<R> {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            sender.send(Message::Terminate).unwrap_or_else(|err| {
                panic!(
                    "result worker failed sending termination msg with error: {}",
                    err
                )
            });
            drop(sender);
        }

        drop(self.receiver.take());

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap_or_else(|err| {
                panic!(
                    "result worker failed joining thread on drop with error: {:?}",
                    err
                )
            })
        }
    }
}
