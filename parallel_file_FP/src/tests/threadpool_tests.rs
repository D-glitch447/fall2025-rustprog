use parallel_file_processor::threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

#[test]
fn threadpool_executes_all_jobs() {
    let pool = ThreadPool::new(4);

    let counter = Arc::new(Mutex::new(0));

    for _ in 0..20 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            let mut lock = c.lock().unwrap();
            *lock += 1;
        });
    }

    drop(pool); // force Drop and worker join

    let final_value = *counter.lock().unwrap();
    assert_eq!(final_value, 20);
}

#[test]
fn threadpool_shutdown_is_clean() {
    for _ in 0..5 {
        let pool = ThreadPool::new(2);

        for _ in 0..5 {
            pool.execute(|| {
                // some small work
                let _ = 2 + 2;
            });
        }

        // Drop should join workers with no panic
        drop(pool);
    }
}
