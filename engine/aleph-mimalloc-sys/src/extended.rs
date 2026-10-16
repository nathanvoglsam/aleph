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

#![allow(nonstandard_style)]

use core::ffi::{c_char, c_int, c_long, c_ulonglong, c_void};

/// The maximum number of bytes which may be used as an argument to a function
/// in the `_small` family ([`mi_malloc_small`], [`mi_zalloc_small`], etc).
pub const MI_SMALL_SIZE_MAX: usize = 128 * core::mem::size_of::<*mut c_void>();

pub const MI_BIN_HUGE: usize = 73;

pub const MI_STAT_VERSION: usize = 5;

// Size bins for chunks
#[repr(C)]
pub enum mi_chunkbin_t {
    /// slice_count == 1
    MI_CBIN_SMALL,
    /// slice_count: any other from the other bins, and 1 <= slice_count <= MI_BCHUNK_BITS
    MI_CBIN_OTHER,
    /// slice_count == 8
    MI_CBIN_MEDIUM,
    /// slice_count == MI_SIZE_BITS  (only used if MI_ENABLE_LARGE_PAGES is 1)
    MI_CBIN_LARGE,
    /// slice_count > MI_BCHUNK_BITS
    MI_CBIN_HUGE,
    /// no bin assigned yet (the chunk is completely free)
    MI_CBIN_NONE,
    /// count
    MI_CBIN_COUNT,
}

/// count allocation over time
#[repr(C)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct mi_stat_count_t {
    /// total allocated
    pub total: i64,

    /// peak allocation
    pub peak: i64,

    /// current allocation
    pub current: i64,
}

/// counters only increase
#[repr(C)]
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct mi_stat_counter_t {
    /// total count
    pub total: i64,
}

#[repr(C)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct mi_stats_t {
    /// size of the mi_stats_t structure
    pub size: usize,
    pub version: usize,

    pub pages: mi_stat_count_t,
    pub reserved: mi_stat_count_t,
    pub committed: mi_stat_count_t,
    pub reset: mi_stat_counter_t,
    pub purged: mi_stat_counter_t,
    pub page_committed: mi_stat_count_t,
    pub pages_abandoned: mi_stat_count_t,
    pub threads: mi_stat_count_t,
    pub malloc_normal: mi_stat_count_t,
    pub malloc_huge: mi_stat_count_t,
    pub malloc_requested: mi_stat_counter_t,

    pub mmap_calls: mi_stat_counter_t,
    pub commit_calls: mi_stat_counter_t,
    pub reset_calls: mi_stat_counter_t,
    pub purge_calls: mi_stat_counter_t,
    pub arena_count: mi_stat_counter_t,
    pub malloc_normal_count: mi_stat_counter_t,
    pub malloc_huge_count: mi_stat_counter_t,
    pub malloc_guarded_count: mi_stat_counter_t,

    pub arena_rollback_count: mi_stat_counter_t,
    pub arena_purges: mi_stat_counter_t,
    pub pages_extended: mi_stat_counter_t,
    pub pages_retire: mi_stat_counter_t,
    pub page_searches: mi_stat_counter_t,
    pub page_searches_count: mi_stat_counter_t,

    pub segments: mi_stat_count_t,
    pub segments_abandoned: mi_stat_count_t,
    pub segments_cache: mi_stat_count_t,
    pub _segments_reserved: mi_stat_count_t,

    pub heaps: mi_stat_count_t,
    pub theaps: mi_stat_count_t,
    pub pages_os_abandoned: mi_stat_count_t,
    pub pages_os_allocated: mi_stat_count_t,
    pub pages_reclaim_on_alloc: mi_stat_counter_t,
    pub pages_reclaim_on_free: mi_stat_counter_t,
    pub pages_reabandon_full: mi_stat_counter_t,
    pub pages_unabandon_busy_wait: mi_stat_counter_t,
    pub heaps_delete_wait: mi_stat_counter_t,
    pub pages_stat_updates: mi_stat_counter_t,
    pub pages_stat_update_count: mi_stat_counter_t,
    pub profile_samples: mi_stat_counter_t,

    // future extension
    _stat_reserved: [mi_stat_count_t; 4],
    _stat_counter_reserved: [mi_stat_counter_t; 4],

    // size segregated statistics
    /// allocation per size bin
    pub malloc_bins: [mi_stat_count_t; MI_BIN_HUGE + 1],
    /// pages allocated per size bin
    pub page_bins: [mi_stat_count_t; MI_BIN_HUGE + 1],
    /// chunks per page sizes
    pub chunk_bins: [mi_stat_count_t; mi_chunkbin_t::MI_CBIN_COUNT as usize],
}

impl Default for mi_stats_t {
    fn default() -> Self {
        Self {
            size: size_of::<Self>(),
            version: MI_STAT_VERSION,
            pages: Default::default(),
            reserved: Default::default(),
            committed: Default::default(),
            reset: Default::default(),
            purged: Default::default(),
            page_committed: Default::default(),
            pages_abandoned: Default::default(),
            threads: Default::default(),
            malloc_normal: Default::default(),
            malloc_huge: Default::default(),
            malloc_requested: Default::default(),
            mmap_calls: Default::default(),
            commit_calls: Default::default(),
            reset_calls: Default::default(),
            purge_calls: Default::default(),
            arena_count: Default::default(),
            malloc_normal_count: Default::default(),
            malloc_huge_count: Default::default(),
            malloc_guarded_count: Default::default(),
            arena_rollback_count: Default::default(),
            arena_purges: Default::default(),
            pages_extended: Default::default(),
            pages_retire: Default::default(),
            page_searches: Default::default(),
            page_searches_count: Default::default(),
            segments: Default::default(),
            segments_abandoned: Default::default(),
            segments_cache: Default::default(),
            _segments_reserved: Default::default(),
            heaps: Default::default(),
            theaps: Default::default(),
            pages_os_abandoned: Default::default(),
            pages_os_allocated: Default::default(),
            pages_reclaim_on_alloc: Default::default(),
            pages_reclaim_on_free: Default::default(),
            pages_reabandon_full: Default::default(),
            pages_unabandon_busy_wait: Default::default(),
            heaps_delete_wait: Default::default(),
            pages_stat_updates: Default::default(),
            pages_stat_update_count: Default::default(),
            profile_samples: Default::default(),
            _stat_reserved: core::array::from_fn(|_| Default::default()),
            _stat_counter_reserved: core::array::from_fn(|_| Default::default()),
            malloc_bins: core::array::from_fn(|_| Default::default()),
            page_bins: core::array::from_fn(|_| Default::default()),
            chunk_bins: core::array::from_fn(|_| Default::default()),
        }
    }
}

unsafe extern "C" {
    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory.
    ///
    /// All items are initialized to zero.
    pub fn mi_calloc(count: usize, size: usize) -> *mut c_void;

    /// Allocate `count` items of `size` length each.
    ///
    /// Returns `null` if `count * size` overflows or on out-of-memory,
    /// otherwise returns the same as [`mi_malloc(count *
    /// size)`](crate::mi_malloc).
    /// Equivalent to [`mi_calloc`], but returns uninitialized (and not zeroed)
    /// bytes.
    pub fn mi_mallocn(count: usize, size: usize) -> *mut c_void;

    /// Re-allocate memory to `count` elements of `size` bytes.
    ///
    /// The realloc equivalent of the [`mi_mallocn`] interface. Returns `null`
    /// if `count * size` overflows or on out-of-memory, otherwise returns the
    /// same as [`mi_realloc(p, count * size)`](crate::mi_realloc).
    pub fn mi_reallocn(p: *mut c_void, count: usize, size: usize) -> *mut c_void;

    /// Try to re-allocate memory to `newsize` bytes _in place_.
    ///
    /// Returns null on out-of-memory or if the memory could not be expanded in
    /// place. On success, returns the same pointer as `p`.
    ///
    /// If `newsize` is larger than the original `size` allocated for `p`, the
    /// bytes after `size` are uninitialized.
    ///
    /// If null is returned, the original pointer is not freed.
    ///
    /// Note: Conceptually, this is a realloc-like which returns null if it
    /// would be forced to reallocate memory and copy. In practice it's
    /// equivalent testing against [`mi_usable_size`](crate::mi_usable_size).
    pub fn mi_expand(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes.
    ///
    /// This differs from [`mi_realloc`](crate::mi_realloc) in that on failure,
    /// `p` is freed.
    pub fn mi_reallocf(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Allocate and duplicate a nul-terminated C string.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strdup(s: *const c_char) -> *mut c_char;

    /// Allocate and duplicate a nul-terminated C string, up to `n` bytes.
    ///
    /// This can be useful for Rust code when interacting with the FFI.
    pub fn mi_strndup(s: *const c_char, n: usize) -> *mut c_char;

    /// Resolve a file path name, producing a `C` string which can be passed to
    /// [`mi_free`](crate::mi_free).
    ///
    /// `resolved_name` should be null, but can also point to a buffer of at
    /// least `PATH_MAX` bytes.
    ///
    /// If successful, returns a pointer to the resolved absolute file name, or
    /// `null` on failure (with `errno` set to the error code).
    ///
    /// If `resolved_name` was `null`, the returned result should be freed with
    /// [`mi_free`](crate::mi_free).
    ///
    /// This can rarely be useful in FFI code, but is mostly included for
    /// completeness.
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;

    /// Allocate `size * count` bytes aligned by `alignment`.
    ///
    /// Return pointer to the allocated memory or null if out of memory or if
    /// `size * count` overflows.
    ///
    /// Returns a unique pointer if called with `size * count` 0.
    pub fn mi_calloc_aligned(count: usize, size: usize, alignment: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`.
    ///
    /// Note that the resulting pointer itself is not aligned by the alignment,
    /// but after `offset` bytes it will be. This can be useful for allocating
    /// data with an inline header, where the data has a specific alignment
    /// requirement.
    ///
    /// Specifically, if `p` is the returned pointer `p.add(offset)` is aligned
    /// to `alignment`.
    pub fn mi_malloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` bytes aligned by `alignment` at a specified `offset`,
    /// zero-initialized.
    ///
    /// This is a [`mi_zalloc`](crate::mi_zalloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_zalloc_aligned_at(size: usize, alignment: usize, offset: usize) -> *mut c_void;

    /// Allocate `size` of bytes aligned by `alignment` and place the address of the
    /// allocated memory to `ptr`.
    ///
    /// Returns zero on success, invalid argument for invalid alignment, or out-of-memory.
    pub fn mi_posix_memalign(ptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;

    /// Allocate `size` bytes aligned by `alignment` with alignment as the first
    /// parameter.
    ///
    /// Return pointer to the allocated memory or null if out of memory.
    pub fn mi_aligned_alloc(alignment: usize, size: usize) -> *mut c_void;

    /// Allocate `size * count` bytes aligned by `alignment` at a specified
    /// `offset`, zero-initialized.
    ///
    /// This is a [`calloc`](crate::mi_calloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_calloc_aligned_at(
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Re-allocate memory to `newsize` bytes aligned by `alignment` at a
    /// specified `offset`.
    ///
    /// This is a [`realloc`](crate::mi_realloc) equivalent of [`mi_malloc_aligned_at`].
    pub fn mi_realloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Zero initialized [re-allocation](crate::mi_realloc).
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`](crate::mi_calloc),
    /// [`mi_zalloc`](crate::mi_zalloc),
    /// [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), ...
    pub fn mi_rezalloc(p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Zero initialized [re-allocation](crate::mi_realloc), following `calloc`
    /// paramater conventions.
    ///
    /// In general, only valid on memory originally allocated by zero
    /// initialization: [`mi_calloc`](crate::mi_calloc),
    /// [`mi_zalloc`](crate::mi_zalloc),
    /// [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), ...
    pub fn mi_recalloc(p: *mut c_void, newcount: usize, size: usize) -> *mut c_void;

    /// Aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned(p: *mut c_void, newsize: usize, alignment: usize) -> *mut c_void;

    /// Offset-aligned version of [`mi_rezalloc`].
    pub fn mi_rezalloc_aligned_at(
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Offset-aligned version of [`mi_recalloc`].
    pub fn mi_recalloc_aligned_at(
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Allocate an object of no more than [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_malloc`](crate::mi_malloc) checks if `size` is
    /// small and calls this if
    /// so at runtime, so its' only worth using if you know for certain.
    pub fn mi_malloc_small(size: usize) -> *mut c_void;

    /// Allocate an zero-initialized object of no more than
    /// [`MI_SMALL_SIZE_MAX`] bytes.
    ///
    /// Does not check that `size` is indeed small.
    ///
    /// Note: Currently [`mi_zalloc`](crate::mi_zalloc) checks if `size` is
    /// small and calls this if so at runtime, so its' only worth using if you
    /// know for certain.
    pub fn mi_zalloc_small(size: usize) -> *mut c_void;

    /// Free a small object. Only use to free objects from [`mi_malloc_small`]
    /// or [`mi_zalloc_small`]. Potentially a tiny bit faster than [`mi_free`](crate::mi_free).
    pub fn mi_free_small(p: *mut c_void);

    /// Return the available bytes in a memory block.
    ///
    /// The returned size can be used to call `mi_expand` successfully.
    pub fn mi_usable_size(p: *const c_void) -> usize;

    /// Return the used allocation size.
    ///
    /// Returns the size `n` that will be allocated, where `n >= size`.
    ///
    /// Generally, `mi_usable_size(mi_malloc(size)) == mi_good_size(size)`. This
    /// can be used to reduce internal wasted space when allocating buffers for
    /// example.
    ///
    /// See [`mi_usable_size`](crate::mi_usable_size).
    pub fn mi_good_size(size: usize) -> usize;

    /// Eagerly free memory.
    ///
    /// If `force` is true, aggressively return memory to the OS (can be
    /// expensive!)
    ///
    /// Regular code should not have to call this function. It can be beneficial
    /// in very narrow circumstances; in particular, when a long running thread
    /// allocates a lot of blocks that are freed by other threads it may improve
    /// resource usage by calling this every once in a while.
    pub fn mi_collect(force: bool);

    /// Checked free: If `p` came from mimalloc's heap (as decided by
    /// [`mi_is_in_heap_region`]), this is [`mi_free(p)`](crate::mi_free), but
    /// otherwise it is a no-op.
    pub fn mi_cfree(p: *mut c_void);

    /// Returns true if this is a pointer into a memory region that has been
    /// reserved by the mimalloc heap.
    ///
    /// This function is described by the mimalloc documentation as "relatively
    /// fast".
    ///
    /// See also [`mi_heap_check_owned`], which is (much) slower and slightly
    /// more precise, but only concerns a single `mi_heap`.
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;

    /// Layout-aware deallocation: Like [`mi_free`](crate::mi_free), but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function, and you are not required
    /// to use this to deallocate memory from an aligned allocation function.
    pub fn mi_free_size_aligned(p: *mut c_void, size: usize, alignment: usize);

    /// Size-aware deallocation: Like [`mi_free`](crate::mi_free), but accepts
    /// the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_size(p: *mut c_void, size: usize);

    /// Alignment-aware deallocation: Like [`mi_free`](crate::mi_free), but
    /// accepts the size and alignment as well.
    ///
    /// Note: unlike some allocators that require this information for
    /// performance, mimalloc doesn't need it (as of the current version,
    /// v2.0.0), and so it currently implements this as a (debug) assertion that
    /// verifies that `p` is actually aligned to `alignment` and is usable for
    /// at least `size` bytes, before delegating to `mi_free`.
    ///
    /// However, currently there's no way to have this crate enable mimalloc's
    /// debug assertions, so these checks aren't particularly useful.
    ///
    /// Note: It's legal to pass null to this function.
    pub fn mi_free_aligned(p: *mut c_void, alignment: usize);

    /// Print the main statistics.
    ///
    /// Ignores the passed in argument, and outputs to the registered output
    /// function or stderr by default.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print(_: *mut c_void);

    /// Print the main statistics.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter.
    ///
    /// Most detailed when using a debug build.
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Reset statistics.
    ///
    /// Note: This function is thread safe.
    pub fn mi_stats_reset();

    /// Return the mimalloc version number.
    ///
    /// For example version 1.6.3 would return the number `163`.
    pub fn mi_version() -> c_int;

    /// Initialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically.
    pub fn mi_thread_init();

    /// Initialize the process.
    ///
    /// Should not be used on most systems, as it's called by thread_init or the
    /// process loader.
    pub fn mi_process_init();

    /// Return process information (time and memory usage). All parameters are
    /// optional (nullable) out-params:
    ///
    /// | Parameter        | Description |
    /// | :-               | :- |
    /// | `elapsed_msecs`  | Elapsed wall-clock time of the process in milli-seconds. |
    /// | `user_msecs`     | User time in milli-seconds (as the sum over all threads). |
    /// | `system_msecs`   | System time in milli-seconds. |
    /// | `current_rss`    | Current working set size (touched pages). |
    /// | `peak_rss`       | Peak working set size (touched pages). |
    /// | `current_commit` | Current committed memory (backed by the page file). |
    /// | `peak_commit`    | Peak committed memory (backed by the page file). |
    /// | `page_faults`    | Count of hard page faults. |
    ///
    /// The `current_rss` is precise on Windows and MacOSX; other systems
    /// estimate this using `current_commit`. The `commit` is precise on Windows
    /// but estimated on other systems as the amount of read/write accessible
    /// memory reserved by mimalloc.
    pub fn mi_process_info(
        elapsed_msecs: *mut usize,
        user_msecs: *mut usize,
        system_msecs: *mut usize,
        current_rss: *mut usize,
        peak_rss: *mut usize,
        current_commit: *mut usize,
        peak_commit: *mut usize,
        page_faults: *mut usize,
    );

    /// Uninitialize mimalloc on a thread.
    ///
    /// Should not be used as on most systems (pthreads, windows) this is done
    /// automatically. Ensures that any memory that is not freed yet (but will
    /// be freed by other threads in the future) is properly handled.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_done();

    /// Print out heap statistics for this thread.
    ///
    /// Pass `None` for `out` to use the default. If `out` is provided, `arc` is
    /// passed as it's second parameter
    ///
    /// Most detailed when using a debug build.
    ///
    /// Note: This function is thread safe.
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);

    /// Register an output function.
    ///
    /// - `out` The output function, use `None` to output to stderr.
    /// - `arg` Argument that will be passed on to the output function.
    ///
    /// The `out` function is called to output any information from mimalloc,
    /// like verbose or warning messages.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);

    /// Register a deferred free function.
    ///
    /// - `deferred_free` Address of a deferred free-ing function or `None` to
    ///   unregister.
    /// - `arg` Argument that will be passed on to the deferred free function.
    ///
    /// Some runtime systems use deferred free-ing, for example when using
    /// reference counting to limit the worst case free time.
    ///
    /// Such systems can register (re-entrant) deferred free function to free
    /// more memory on demand.
    ///
    /// - When the `force` parameter is `true` all possible memory should be
    ///   freed.
    ///
    /// - The per-thread `heartbeat` parameter is monotonically increasing and
    ///   guaranteed to be deterministic if the program allocates
    ///   deterministically.
    ///
    /// - The `deferred_free` function is guaranteed to be called
    ///   deterministically after some number of allocations (regardless of
    ///   freeing or available free memory).
    ///
    /// At most one `deferred_free` function can be active.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_deferred_free(out: mi_deferred_free_fun, arg: *mut c_void);

    /// Register an error callback function.
    ///
    /// The `errfun` function is called on an error in mimalloc after emitting
    /// an error message (through the output function).
    ///
    /// It as always legal to just return from the `errfun` function in which
    /// case allocation functions generally return null or ignore the condition.
    ///
    /// The default function only calls abort() when compiled in secure mode
    /// with an `EFAULT` error. The possible error codes are:
    ///
    /// - `EAGAIN` (11): Double free was detected (only in debug and secure
    ///   mode).
    /// - `EFAULT` (14): Corrupted free list or meta-data was detected (only in
    ///   debug and secure mode).
    /// - `ENOMEM` (12): Not enough memory available to satisfy the request.
    /// - `EOVERFLOW` (75): Too large a request, for example in `mi_calloc`, the
    ///   `count` and `size` parameters are too large.
    /// - `EINVAL` (22): Trying to free or re-allocate an invalid pointer.
    ///
    /// Note: This function is thread safe.
    pub fn mi_register_error(out: mi_error_fun, arg: *mut c_void);

    /// Get the statistics for the current subprocess aggregated over all its heaps as JSON.
    ///
    /// Returns pointer to the buffer or NULL on failure. Use mi_free() to free the buffer if the buf parameter was NULL.
    pub fn mi_stats_get_json(buf_size: usize, buf: *mut c_char) -> *mut c_char;

    /// Get the statistics for the current subprocess aggregated over all its heaps.
    pub fn mi_stats_get(stats: *mut mi_stats_t) -> bool;
}

/// An output callback. Must be thread-safe.
///
/// See [`mi_stats_print_out`], [`mi_thread_stats_print_out`], [`mi_register_output`]
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;

/// Type of deferred free functions. Must be thread-safe.
///
/// - `force`: If true, all outstanding items should be freed.
/// - `heartbeat` A monotonically increasing count.
/// - `arg` Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_deferred_free`]
pub type mi_deferred_free_fun =
    Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;

/// Type of error callback functions. Must be thread-safe.
///
/// - `err`: Error code (see [`mi_register_error`] for a list).
/// - `arg`: Argument that was passed at registration to hold extra state.
///
/// See [`mi_register_error`]
pub type mi_error_fun = Option<unsafe extern "C" fn(code: c_int, arg: *mut c_void)>;

/// Runtime options. All options are false by default.
pub type mi_option_t = c_int;

// Note: mimalloc doc website seems to have the order of show_stats and
// show_errors reversed as of 1.6.3, however what I have here is correct:
// https://github.com/microsoft/mimalloc/issues/266#issuecomment-653822341

pub const mi_option_show_errors: mi_option_t = 0;
pub const mi_option_show_stats: mi_option_t = 1;
pub const mi_option_verbose: mi_option_t = 2;
pub const mi_option_deprecated_eager_commit: mi_option_t = 3;
pub const mi_option_arena_eager_commit: mi_option_t = 4; // eager commit arenas? Use 2 to enable just on overcommit systems (=2)
pub const mi_option_purge_decommits: mi_option_t = 5; // should a memory purge decommit? (=1). Set to 0 to use memory reset on a purge (instead of decommit)
pub const mi_option_allow_large_os_pages: mi_option_t = 6; // allow use of large (2 or 4 MiB) OS pages, implies eager commit.
pub const mi_option_reserve_huge_os_pages: mi_option_t = 7; // reserve N huge OS pages (1GiB pages) at startup
pub const mi_option_reserve_huge_os_pages_at: mi_option_t = 8; // reserve huge OS pages at a specific NUMA node
pub const mi_option_reserve_os_memory: mi_option_t = 9; // reserve specified amount of OS memory in an arena at startup (internally, this value is in KiB; use `mi_option_get_size`)
pub const mi_option_deprecated_segment_cache: mi_option_t = 10;
pub const mi_option_deprecated_page_reset: mi_option_t = 11;
pub const mi_option_deprecated_abandoned_page_purge: mi_option_t = 12;
pub const mi_option_deprecated_segment_reset: mi_option_t = 13;
pub const mi_option_deprecated_eager_commit_delay: mi_option_t = 14;
pub const mi_option_purge_delay: mi_option_t = 15; // memory purging is delayed by N milli seconds; use 0 for immediate purging or -1 for no purging at all. (=10)
pub const mi_option_use_numa_nodes: mi_option_t = 16; // 0 = use all available numa nodes, otherwise use at most N nodes.
pub const mi_option_disallow_os_alloc: mi_option_t = 17; // 1 = do not use OS memory for allocation (but only programmatically reserved arenas)
pub const mi_option_os_tag: mi_option_t = 18; // tag used for OS logging (macOS only for now) (=100)
pub const mi_option_max_errors: mi_option_t = 19; // issue at most N error messages
pub const mi_option_max_warnings: mi_option_t = 20; // issue at most N warning messages
pub const mi_option_deprecated_max_segment_reclaim: mi_option_t = 21; // max. percentage of the abandoned segments can be reclaimed per try (=10%)
pub const mi_option_destroy_on_exit: mi_option_t = 22; // if set, release all memory on exit; sometimes used for dynamic unloading but can be unsafe
pub const mi_option_arena_reserve: mi_option_t = 23; // initial memory size for arena reservation (= 1 GiB on 64-bit) (internally, this value is in KiB; use `mi_option_get_size`)
pub const mi_option_arena_purge_mult: mi_option_t = 24; // multiplier for `purge_delay` for the purging delay for arenas (=10)
pub const mi_option_deprecated_purge_extend_delay: mi_option_t = 25;
pub const mi_option_disallow_arena_alloc: mi_option_t = 26; // 1 = do not use arena's for allocation (except if using specific arena id's)
pub const mi_option_retry_on_oom: mi_option_t = 27; // retry on out-of-memory for N milli seconds (=400), set to 0 to disable retries. (only on windows)
pub const mi_option_deprecated_visit_abandoned: mi_option_t = 28; // allow visiting theap blocks from abandoned threads (=0)
pub const mi_option_guarded_min: mi_option_t = 29; // only used when building with MI_GUARDED: minimal rounded object size for guarded objects (=0)
pub const mi_option_guarded_max: mi_option_t = 30; // only used when building with MI_GUARDED: maximal rounded object size for guarded objects (=0)
pub const mi_option_guarded_precise: mi_option_t = 31; // disregard minimal alignment requirement to always place guarded blocks exactly in front of a guard page (=0)
pub const mi_option_guarded_sample_rate: mi_option_t = 32; // 1 out of N allocations in the min/max range will be guarded (=1000)
pub const mi_option_guarded_sample_seed: mi_option_t = 33; // can be set to allow for a (more) deterministic re-execution when a guard page is triggered (=0)
pub const mi_option_generic_collect: mi_option_t = 34; // collect theaps every N (=10000) generic allocation calls
pub const mi_option_page_reclaim_on_free: mi_option_t = 35; // reclaim abandoned pages on a free (=0). -1 disallow always, 0 allows if the page originated from the current theap, 1 allow always
pub const mi_option_page_full_retain: mi_option_t = 36; // retain N full (small) pages per size class (=2)
pub const mi_option_page_max_candidates: mi_option_t = 37; // max candidate pages to consider for allocation (=4)
pub const mi_option_max_vabits: mi_option_t = 38; // max user space virtual address bits to consider (=48)
pub const mi_option_pagemap_commit: mi_option_t = 39; // commit the full pagemap (to always catch invalid pointer uses) (=0)
pub const mi_option_page_commit_on_demand: mi_option_t = 40; // commit page memory on-demand
pub const mi_option_page_max_reclaim: mi_option_t = 41; // don't reclaim pages of the same originating theap if we already own N pages (in that size class) (=-1 (unlimited))
pub const mi_option_page_cross_thread_max_reclaim: mi_option_t = 42; // don't reclaim pages across threads if we already own N pages (in that size class) (=32)
pub const mi_option_allow_thp: mi_option_t = 43; // allow transparent huge pages? (=1) (on Android =0 by default). Set to 0 to disable THP for the process.
pub const mi_option_minimal_purge_size: mi_option_t = 44; // set minimal purge size (in KiB) (=0). By default set to either 64 or 2048 if THP is enabled.
pub const mi_option_arena_max_object_size: mi_option_t = 45; // set maximal object size that can be allocated in an arena (in KiB) (=2GiB on 64-bit). 
pub const mi_option_arena_is_numa_local: mi_option_t = 46; // experimental: associate local numa node with an initial arena allocation
pub const mi_option_collect_merges_stats: mi_option_t = 47; // on each theap collection, the theap stats are merged automatically with the parent heap

unsafe extern "C" {
    // Note: mi_option_{enable,disable} aren't exposed because they're redundant
    // and because of https://github.com/microsoft/mimalloc/issues/266.

    /// Returns true if the provided option is enabled.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;

    /// Enable or disable the given option.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], enables or disables the option. If it has,
    /// this function does nothing.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);

    /// Returns the value of the provided option.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed, see the mimalloc documentation for
    /// details: https://microsoft.github.io/mimalloc/group__options.html
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_get(option: mi_option_t) -> c_long;

    /// Set the option to the given value.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed,
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set(option: mi_option_t, value: c_long);

    /// If the given option has not yet been initialized with [`mi_option_set`]
    /// or [`mi_option_set_enabled`], sets the option to the given value. If it
    /// has, this function does nothing.
    ///
    /// The value of boolean options is 1 or 0, however experimental options
    /// exist which take a numeric value, which is the intended use of this
    /// function.
    ///
    /// These options are not exposed as constants for stability reasons,
    /// however you can still use them as arguments to this and other
    /// `mi_option_` functions if needed.
    ///
    /// Note: this function is not thread safe.
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
}

/// First-class heaps that can be destroyed in one go.
///
/// Note: The pointers allocated out of a heap can be be freed using
/// [`mi_free`](crate::mi_free) -- there is no `mi_heap_free`.
///
/// # Example
///
/// ```
/// use aleph_mimalloc_sys as mi;
/// unsafe {
///     let h = mi::mi_heap_new();
///     assert!(!h.is_null());
///     let p = mi::mi_heap_malloc(h, 50);
///     assert!(!p.is_null());
///
///     // use p...
///     mi::mi_free(p);
///
///     // Clean up the heap. Note that pointers allocated from `h`
///     // are *not* invalided by `mi_heap_delete`. You would have
///     // to use (the very dangerous) `mi_heap_destroy` for that
///     // behavior
///     mi::mi_heap_delete(h);
/// }
/// ```
pub enum mi_heap_t {}

pub enum mi_theap_t {}

/// An area of heap space contains blocks of a single size.
///
/// The bytes in freed blocks are `committed - used`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mi_heap_area_t {
    /// Start of the area containing heap blocks.
    pub blocks: *mut c_void,
    /// Bytes reserved for this area.
    pub reserved: usize,
    /// Current committed bytes of this area.
    pub committed: usize,
    /// Bytes in use by allocated blocks.
    pub used: usize,
    /// Size in bytes of one block.
    pub block_size: usize,
    /// Size in bytes of a full block including padding and metadata.
    pub full_block_size: usize,
    /// Reserved / internal (replaces `heap_tag` in v3)
    pub reserved1: *mut c_void,
}

/// Visitor function passed to [`mi_heap_visit_blocks`]
///
/// Should return `true` to continue, and `false` to stop visiting (i.e. break)
///
/// This function is always first called for every `area` with `block` as a null
/// pointer. If `visit_all_blocks` was `true`, the function is then called for
/// every allocated block in that area.
pub type mi_block_visit_fun = Option<
    unsafe extern "C" fn(
        heap: *const mi_heap_t,
        area: *const mi_heap_area_t,
        block: *mut c_void,
        block_size: usize,
        arg: *mut c_void,
    ) -> bool,
>;

unsafe extern "C" {
    /// Create a new heap that can be used for allocation.
    pub fn mi_heap_new() -> *mut mi_heap_t;

    /// Delete a previously allocated heap.
    ///
    /// This will release resources and migrate any still allocated blocks in
    /// this heap (efficienty) to the default heap.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_delete(heap: *mut mi_heap_t);

    /// Destroy a heap, freeing all its still allocated blocks.
    ///
    /// Use with care as this will free all blocks still allocated in the heap.
    /// However, this can be a very efficient way to free all heap memory in one
    /// go.
    ///
    /// If `heap` is the default heap, the default heap is set to the backing
    /// heap.
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);

    /// Release outstanding resources in a specific heap.
    ///
    /// See also [`mi_collect`].
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);

    /// Equivalent to [`mi_malloc`](crate::mi_malloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_zalloc`](crate::mi_zalloc), but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_calloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_mallocn`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: usize, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_malloc_small`], but allocates out of the specific
    /// heap instead of the default.
    ///
    /// `size` must be smaller or equal to [`MI_SMALL_SIZE_MAX`].
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_small`], but allocates out of the specific
    /// heap instead of the default.
    ///
    /// `size` must be smaller or equal to [`MI_SMALL_SIZE_MAX`].
    pub fn mi_heap_zalloc_small(heap: *mut mi_heap_t, size: usize) -> *mut c_void;

    /// Equivalent to [`mi_realloc`](crate::mi_realloc), but allocates out of
    /// the specific heap instead of the default.
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_reallocn`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocn(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        count: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_reallocf`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_strdup`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;

    /// Equivalent to [`mi_strndup`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: usize) -> *mut c_char;

    /// Equivalent to [`mi_realpath`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_realpath(
        heap: *mut mi_heap_t,
        fname: *const c_char,
        resolved_name: *mut c_char,
    ) -> *mut c_char;

    /// Equivalent to [`mi_malloc_aligned`](crate::mi_malloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_malloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_malloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_malloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned`](crate::mi_zalloc_aligned), but
    /// allocates out of the specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_zalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_zalloc_aligned_at(
        heap: *mut mi_heap_t,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned`], but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_calloc_aligned(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_calloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_calloc_aligned_at(
        heap: *mut mi_heap_t,
        count: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned`](crate::mi_realloc_aligned), but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_realloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_realloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_realloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: usize) -> *mut c_void;

    /// Equivalent to [`mi_recalloc`], but allocates out of the specific heap
    /// instead of the default.
    pub fn mi_heap_recalloc(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned`], but allocates out of the specific
    /// heap instead of the default.
    pub fn mi_heap_rezalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_rezalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_rezalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newsize: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
    ) -> *mut c_void;

    /// Equivalent to [`mi_recalloc_aligned_at`], but allocates out of the
    /// specific heap instead of the default.
    pub fn mi_heap_recalloc_aligned_at(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        newcount: usize,
        size: usize,
        alignment: usize,
        offset: usize,
    ) -> *mut c_void;

    /// Check safely if any pointer is part of the default heap of this thread.
    ///
    /// `p` may be any pointer -- not required to be previously allocated by the
    /// default heap for this thread, or any other mimalloc heap. Returns `true`
    /// if `p` points to a block in the default heap, false otherwise.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    ///
    /// See [`mi_heap_contains_block`], [`mi_heap_get_default`]
    pub fn mi_check_owned(p: *const c_void) -> bool;

    /// Visit all areas and blocks in `heap`.
    ///
    /// If `visit_all_blocks` is false, the `visitor` is only called once for
    /// every heap area. If it's true, the `visitor` is also called for every
    /// allocated block inside every area (with `!block.is_null()`). Return
    /// `false` from the `visitor` to return early.
    ///
    /// `arg` is an extra argument passed into the `visitor`.
    ///
    /// Returns `true` if all areas and blocks were visited.
    ///
    /// Passing a `None` visitor is allowed, and is a no-op.
    ///
    /// Note: in v3 the `heap` parameter is non-const compared to v2.
    pub fn mi_heap_visit_blocks(
        heap: *mut mi_heap_t,
        visit_all_blocks: bool,
        visitor: mi_block_visit_fun,
        arg: *mut c_void,
    ) -> bool;

    /// Check if the heap page containing `p` is under-utilized.
    ///
    /// # Safety
    /// Assumes the page belonging to `p` is only accessed by the calling thread.
    pub fn mi_unsafe_heap_page_is_under_utilized(
        heap: *mut mi_heap_t,
        p: *mut c_void,
        perc_threshold: usize,
    ) -> bool;

    /// Return the minimum size for an arena (v3 only).
    pub fn mi_arena_min_size() -> usize;

    /// Equivalent to [`mi_heap_zalloc_small`], but for a thread-local heap (`theap`) in v3.
    ///
    /// `size` must be smaller or equal to [`MI_SMALL_SIZE_MAX`].
    pub fn mi_theap_zalloc_small(theap: *mut mi_theap_t, size: usize) -> *mut c_void;

    /// This is done automatically for threads in the Windows threadpool, but if using a custom
    /// threadpool it is good to call this on worker threads. Internally, mimalloc uses different
    /// locality heuristics for worker threads to try to reduce non-local access.
    pub fn mi_thread_set_in_threadpool();

    /// Reserve OS memory for use by mimalloc.
    ///
    /// Reserved areas are used before allocating from the OS again. By reserving a large area
    /// upfront, allocation can be more efficient, and can be better managed on systems without
    /// mmap/VirtualAlloc (like WASM for example).
    ///
    /// # Parameters
    ///
    /// - size: The size to reserve.
    /// - commit: Commit the memory upfront.
    /// - allow_large: Allow large OS pages (2MiB) to be used?
    ///
    /// # Returns
    ///
    /// 0 if successful, and an error code otherwise (e.g. ENOMEM).
    pub fn mi_reserve_os_memory(size: usize, commit: bool, allow_large: bool) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::super::mi_malloc;
    use super::*;

    #[test]
    fn it_calculates_usable_size() {
        let ptr = unsafe { mi_malloc(32) } as *mut u8;
        let usable_size = unsafe { mi_usable_size(ptr as *mut c_void) };
        assert!(
            usable_size >= 32,
            "usable_size should at least equal to the allocated size"
        );
    }

    #[test]
    fn runtime_stable_option() {
        unsafe {
            assert_eq!(mi_option_get(mi_option_show_errors), 0);
            mi_option_set(mi_option_show_errors, 1);
            assert_eq!(mi_option_get(mi_option_show_errors), 1);

            assert_eq!(mi_option_get(mi_option_show_stats), 0);
            mi_option_set(mi_option_show_stats, 1);
            assert_eq!(mi_option_get(mi_option_show_stats), 1);

            assert_eq!(mi_option_get(mi_option_verbose), 0);
            mi_option_set(mi_option_verbose, 1);
            assert_eq!(mi_option_get(mi_option_verbose), 1);
        }
    }
}
