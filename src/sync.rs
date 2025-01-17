#[cfg(feature = "worker")]
mod worker;
#[cfg(feature = "worker")]
pub use self::worker::Worker;

#[cfg(feature = "result_worker")]
mod result_worker;

#[cfg(feature = "result_worker")]
pub use self::result_worker::ResultWorker;

#[cfg(feature = "thread_pool")]
mod thread_pool;

#[cfg(feature = "thread_pool")]
pub use self::thread_pool::ThreadPool;

trait FnBox<T = ()> {
    fn run_task(self: Box<Self>) -> T;
}

impl<T, F: FnOnce() -> T> FnBox<T> for F {
    fn run_task(self: Box<Self>) -> T {
        (*self)()
    }
}

type Task<T = ()> = Box<dyn FnBox<T> + Send + 'static>;

enum Message<T = Task> {
    NewTask(T),
    Terminate,
}
