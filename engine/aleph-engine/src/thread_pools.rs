//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
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
