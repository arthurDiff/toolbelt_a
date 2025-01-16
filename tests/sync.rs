extern crate rand;
extern crate toolbelt_a;

#[cfg(feature = "sync")]
use rand::Rng;

#[cfg(feature = "sync")]
use toolbelt_a::sync::*;

#[cfg(feature = "sync")]
#[test]
fn test_sync_worker_should_update_global_from_worker_thread() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static TEST_ATOMIC: AtomicUsize = AtomicUsize::new(0);

    let rand = rand::thread_rng().gen_range(2..10);
    let worker = worker::Worker::default();

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
