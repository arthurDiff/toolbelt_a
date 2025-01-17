extern crate rand;
extern crate toolbelt_a;

#[cfg(feature = "sync")]
use rand::Rng;

#[cfg(feature = "sync")]
use toolbelt_a::sync::*;

#[cfg(feature = "sync")]
#[test]
fn test_worker_should_update_global_from_worker_thread() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static TEST_ATOMIC: AtomicUsize = AtomicUsize::new(0);

    let rand = rand::thread_rng().gen_range(2..10);
    let worker = Worker::default();

    for _ in 0..rand {
        worker
            .send(|| {
                _ = TEST_ATOMIC.fetch_add(1, Ordering::Relaxed);
            })
            .expect("Failed to send msg to worker");
    }

    // runs join and msgs should have been queued
    drop(worker);

    assert_eq!(TEST_ATOMIC.load(Ordering::Relaxed), rand);
}

#[cfg(feature = "sync")]
#[test]
fn test_result_worker_should_send_expected_result_from_message_func() {
    let mut rand = rand::thread_rng();
    let result_worker = ResultWorker::<usize>::default();

    for _ in 0..10 {
        let send_this = rand.gen_range(0..100);
        result_worker.send(move || send_this).unwrap();

        assert_eq!(send_this, result_worker.recv().unwrap());
    }
}

#[cfg(feature = "sync")]
#[test]
fn test_thread_pool_should_process_atomic_from_threads() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static TEST_ATOMIC: AtomicUsize = AtomicUsize::new(0);

    let rand = rand::thread_rng().gen_range(0..20);
    let thread_pool = ThreadPool::new(5);

    for _ in 0..rand {
        thread_pool
            .send(|| {
                TEST_ATOMIC.fetch_add(1, Ordering::Relaxed);
            })
            .unwrap();
    }

    // joins threads
    drop(thread_pool);

    assert_eq!(rand, TEST_ATOMIC.load(Ordering::Relaxed));
}
