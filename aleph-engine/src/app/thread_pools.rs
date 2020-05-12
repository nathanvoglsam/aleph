//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use once_cell::sync::OnceCell;

pub static LONG_RUNNING_THREAD_POOL: OnceCell<rayon::ThreadPool> = OnceCell::new();

pub fn init_long_thread_pool(num_threads: usize) {
    let long_running_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .thread_name(|id| format!("Long Running Task Thread {}", id))
        .build()
        .expect("Failed to create long running thread pool");
    super::thread_pools::LONG_RUNNING_THREAD_POOL
        .set(long_running_pool)
        .expect("Long Running thread pool already initialized");
}

pub static SHORT_RUNNING_THREAD_POOL: OnceCell<rayon::ThreadPool> = OnceCell::new();

pub fn init_short_thread_pool(num_threads: usize) {
    let short_running_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .thread_name(|id| format!("Short Running Task Thread {}", id))
        .build()
        .expect("Failed to create short running thread pool");
    SHORT_RUNNING_THREAD_POOL
        .set(short_running_pool)
        .expect("Short Running thread pool already initialized");
}
