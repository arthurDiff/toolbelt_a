// #[cfg(any(feature = "sync", feature = "worker"))]
pub mod worker;

pub mod result_worker;

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
