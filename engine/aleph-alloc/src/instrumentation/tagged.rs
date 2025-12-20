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

use std::alloc::{GlobalAlloc, Layout, System, handle_alloc_error};
use std::ptr::NonNull;

use allocator_api2::alloc::{AllocError, Allocator};

use crate::allocator_global_handle::AllocatorGlobalHandle;
use crate::instrumentation::{CategoryInfo, IAllocationCategory, emit_alloc, emit_free};

/// An allocator wrapper that uses a dynamic, thread-local category stack for attributing
/// allocations to a category.
///
/// This allocator has overhead. Each individual allocation will be extended to include a pointer
/// tag to a [`CategoryInfo`] instance. Using this pointer the origin of the allocation can be
/// recovered without exposing the category to the type system.
///
/// This is most useful for use as a global allocator. Registering a [`Tagged`] as the global
/// allocator means you can inject categories into allocations from crates that are not instrumented
/// with 'aleph-alloc' categories. Simply wrap your calls in a [`with_category`] scope and any
/// allocations made within will be tagged with the given category.
pub struct Tagged<A: Allocator = System>(A);

impl<A: Allocator> Tagged<A> {
    /// Constructs a new [`Tagged`] wrapping the given allocator.
    pub const fn new(allocator: A) -> Tagged<A> {
        Self(allocator)
    }

    /// Unwrap the [`Tagged`] and get the inner allocator object.
    #[inline]
    pub fn into_inner(self) -> A {
        self.0
    }
}

unsafe impl<A: AllocatorGlobalHandle> AllocatorGlobalHandle for Tagged<A> {
    fn make_handle() -> Self {
        Self::new(<A as AllocatorGlobalHandle>::make_handle())
    }
}

impl<A: Allocator> From<A> for Tagged<A> {
    fn from(inner: A) -> Self {
        Self::new(inner)
    }
}

impl<A: Allocator + Default> Default for Tagged<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}

unsafe impl<A: Allocator> Allocator for Tagged<A> {
    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if !cfg!(feature = "instrumentation-enabled") {
            self.0.allocate(layout)
        } else {
            let c = category_stack::peek();

            let (actual_layout, offset) = Layout::new::<&'static CategoryInfo>()
                .extend(layout)
                .map_err(|_| AllocError)?;
            match self.0.allocate(actual_layout) {
                Ok(v) => unsafe {
                    let inner_ptr = v.cast::<u8>().add(offset);
                    emit_alloc(c, inner_ptr.as_ptr(), layout.size());

                    v.cast().write(c);

                    Ok(NonNull::slice_from_raw_parts(inner_ptr, layout.size()))
                },
                v @ Err(_) => v,
            }
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if !cfg!(feature = "instrumentation-enabled") {
            self.0.allocate_zeroed(layout)
        } else {
            let c = category_stack::peek();

            let (actual_layout, offset) = Layout::new::<&'static CategoryInfo>()
                .extend(layout)
                .map_err(|_| AllocError)?;
            match self.0.allocate_zeroed(actual_layout) {
                Ok(v) => unsafe {
                    let inner_ptr = v.cast::<u8>().add(offset);
                    emit_alloc(c, inner_ptr.as_ptr(), layout.size());

                    v.cast().write(c);

                    Ok(NonNull::slice_from_raw_parts(inner_ptr, layout.size()))
                },
                v @ Err(_) => v,
            }
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.0.deallocate(ptr, layout);
            }

            let (actual_layout, offset) = Layout::new::<&'static CategoryInfo>()
                .extend(layout)
                .unwrap_unchecked();

            let outer_ptr = ptr.sub(offset);
            let c: &'static CategoryInfo = outer_ptr.cast().read();

            emit_free(c, ptr.as_ptr(), layout.size());
            self.0.deallocate(outer_ptr, actual_layout);
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn grow(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.0.grow(old_ptr, old_layout, new_layout);
            }

            let (actual_old_layout, old_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(old_layout)
                .unwrap_unchecked();
            let (actual_new_layout, new_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(new_layout)
                .map_err(|_| AllocError)?;

            let outer_ptr = old_ptr.sub(old_offset);
            let c: &'static CategoryInfo = outer_ptr.cast().read();

            emit_free(c, old_ptr.as_ptr(), old_layout.size());

            let result = self.0.grow(outer_ptr, actual_old_layout, actual_new_layout);
            handle_resized(c, result, old_ptr, old_layout, new_layout, new_offset)
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn grow_zeroed(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.0.grow_zeroed(old_ptr, old_layout, new_layout);
            }

            let (actual_old_layout, old_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(old_layout)
                .unwrap_unchecked();
            let (actual_new_layout, new_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(new_layout)
                .map_err(|_| AllocError)?;

            let outer_ptr = old_ptr.sub(old_offset);
            let c: &'static CategoryInfo = outer_ptr.cast().read();

            emit_free(c, old_ptr.as_ptr(), old_layout.size());

            let result = self
                .0
                .grow_zeroed(outer_ptr, actual_old_layout, actual_new_layout);
            handle_resized(c, result, old_ptr, old_layout, new_layout, new_offset)
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn shrink(
        &self,
        old_ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return self.0.shrink(old_ptr, old_layout, new_layout);
            }

            let (actual_old_layout, old_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(old_layout)
                .unwrap_unchecked();
            let (actual_new_layout, new_offset) = Layout::new::<&'static CategoryInfo>()
                .extend(new_layout)
                .map_err(|_| AllocError)?;

            let outer_ptr = old_ptr.sub(old_offset);
            let c: &'static CategoryInfo = outer_ptr.cast().read();

            emit_free(c, old_ptr.as_ptr(), old_layout.size());

            let result = self
                .0
                .shrink(outer_ptr, actual_old_layout, actual_new_layout);
            handle_resized(c, result, old_ptr, old_layout, new_layout, new_offset)
        }
    }
}

unsafe impl<A: Allocator + GlobalAlloc> GlobalAlloc for Tagged<A> {
    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::alloc(&self.0, layout);
            }
        }

        match Allocator::allocate(self, layout) {
            Ok(v) => v.cast().as_ptr(),
            Err(_) => handle_alloc_error(layout),
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::dealloc(&self.0, ptr, layout);
            }
        }

        unsafe {
            if let Some(ptr) = NonNull::new(ptr) {
                Allocator::deallocate(self, ptr, layout);
            }
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::alloc_zeroed(&self.0, layout);
            }
        }

        match Allocator::allocate_zeroed(self, layout) {
            Ok(v) => v.cast().as_ptr(),
            Err(_) => handle_alloc_error(layout),
        }
    }

    #[cfg_attr(not(feature = "instrumentation-enabled"), inline(always))]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            if !cfg!(feature = "instrumentation-enabled") {
                return GlobalAlloc::realloc(&self.0, ptr, layout, new_size);
            }
        }

        let result = match NonNull::new(ptr) {
            None => unsafe {
                let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
                Allocator::allocate_zeroed(self, new_layout)
            },
            Some(ptr) => unsafe {
                let old_layout = layout;
                let new_layout = Layout::from_size_align_unchecked(new_size, old_layout.align());
                if old_layout.size() < new_layout.size() {
                    Allocator::grow(self, ptr, old_layout, new_layout)
                } else {
                    Allocator::shrink(self, ptr, old_layout, new_layout)
                }
            },
        };
        match result {
            Ok(v) => v.cast().as_ptr(),
            Err(_) => handle_alloc_error(layout),
        }
    }
}

#[inline]
unsafe fn handle_resized(
    c: &'static CategoryInfo,
    result: Result<NonNull<[u8]>, AllocError>,
    old_ptr: NonNull<u8>,
    old_layout: Layout,
    new_layout: Layout,
    new_offset: usize,
) -> Result<NonNull<[u8]>, AllocError> {
    unsafe {
        match result {
            Ok(outer_ptr) => {
                let outer_ptr = outer_ptr.cast::<u8>();
                outer_ptr.cast().write(c);

                let inner_ptr = outer_ptr.add(new_offset);
                emit_alloc(c, inner_ptr.as_ptr(), new_layout.size());

                Ok(NonNull::slice_from_raw_parts(inner_ptr, new_layout.size()))
            }
            v @ Err(_) => {
                emit_alloc(c, old_ptr.as_ptr(), old_layout.size());
                v
            }
        }
    }
}

/// Functional interface for pushing an allocation category onto the thread-local category stack.
///
/// In general, you should prefer to use [`with_category`] as it ensures the stack will be managed
/// correctly. However, if you have a good use for manually poking at the category stack then this
/// is made available to you. The interface is perfectly safe.
#[inline(always)]
pub fn push_category(c: &'static CategoryInfo) {
    category_stack::push(c);
}

/// Functional interface for popping an allocation category from the thread-local category stack.
///
/// In general, you should prefer to use [`with_category`] as it ensures the stack will be managed
/// correctly. However, if you have a good use for manually poking at the category stack then this
/// is made available to you. The interface is perfectly safe.
#[inline(always)]
pub fn pop_category() {
    category_stack::pop()
}

/// Runs the given closure, adopting the given allocation category for the span of the closure.
///
/// This is a utility to mark all heap allocations that are created within the closure with the
/// adopted category. Very useful for tagging allocations from within non-instrumented crates.
#[inline(always)]
pub fn with_category<T: IAllocationCategory, O>(f: impl FnOnce() -> O) -> O {
    #[cfg(not(feature = "instrumentation-enabled"))]
    {
        f()
    }

    #[cfg(feature = "instrumentation-enabled")]
    {
        with_category_v::<O>(CategoryInfo::get::<T>(), f)
    }
}

/// Alternate form of [`with_category`] that takes a dynamic category reference.
#[inline(always)]
pub fn with_category_v<O>(_info: &'static CategoryInfo, f: impl FnOnce() -> O) -> O {
    #[cfg(not(feature = "instrumentation-enabled"))]
    {
        f()
    }

    #[cfg(feature = "instrumentation-enabled")]
    {
        struct Unwinder;
        impl Drop for Unwinder {
            fn drop(&mut self) {
                pop_category();
            }
        }

        // Awful mess of scopes to make sure that the stack unwinds correctly in all cases, in the event
        // that 'f' panics we still need to pop the category!
        let out;
        {
            push_category(_info);
            {
                let unwinder = Unwinder;
                {
                    out = f();
                }
                drop(unwinder);
            }
        }
        out
    }
}

mod category_stack {
    use crate::instrumentation::{CategoryInfo, Uncategorized};

    #[cfg(feature = "instrumentation-enabled")]
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    mod gnu {
        //! Special windows-gnu specific implementation that sidesteps the standard library's
        //! thread local implementation. The built-in thread_local! hits the global allocator.
        //! However, using this requires using thread_local! inside the global allocator...
        //!
        //! Using thread_local! on windows-gnu for this causes a stack overflow because of infinite
        //! recursion. To work around this we use Windows APIs directly in a way that we never
        //! hit rust's 'Global' allocator.

        use std::sync::LazyLock;

        use aleph_windows_thread_local::ThreadLocal;

        use crate::instrumentation::CategoryInfo;
        use crate::instrumentation::tagged::category_stack::CategoryStack;

        static STACK: LazyLock<ThreadLocal<CategoryStack>> =
            LazyLock::new(|| ThreadLocal::new(CategoryStack::new));

        #[inline]
        pub fn peek() -> &'static CategoryInfo {
            STACK.with(|stack| stack.peek())
        }

        #[inline]
        pub fn push(_info: &'static CategoryInfo) {
            STACK.with(|stack| stack.push(_info))
        }

        #[inline]
        pub fn pop() {
            STACK.with(|stack| stack.pop())
        }
    }

    #[cfg(feature = "instrumentation-enabled")]
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    mod not_gnu {
        use crate::instrumentation::CategoryInfo;
        use crate::instrumentation::tagged::category_stack::CategoryStack;

        thread_local! {
            static STACK: CategoryStack = const { CategoryStack::new() };
        }

        #[inline]
        pub fn peek() -> &'static CategoryInfo {
            STACK.with(|stack| stack.peek())
        }

        #[inline]
        pub fn push(_info: &'static CategoryInfo) {
            STACK.with(|stack| stack.push(_info))
        }

        #[inline]
        pub fn pop() {
            STACK.with(|stack| stack.pop())
        }
    }

    #[cfg(feature = "instrumentation-enabled")]
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    use gnu as backend;
    #[cfg(feature = "instrumentation-enabled")]
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    use not_gnu as backend;

    #[inline]
    pub fn peek() -> &'static CategoryInfo {
        #[cfg(not(feature = "instrumentation-enabled"))]
        {
            use crate::instrumentation::IAllocationCategory;
            Uncategorized::info()
        }

        #[cfg(feature = "instrumentation-enabled")]
        backend::peek()
    }

    #[inline]
    pub fn push(_info: &'static CategoryInfo) {
        #[cfg(feature = "instrumentation-enabled")]
        backend::push(_info)
    }

    #[inline]
    pub fn pop() {
        #[cfg(feature = "instrumentation-enabled")]
        backend::pop()
    }

    #[cfg(feature = "instrumentation-enabled")]
    #[derive(Default)]
    struct CategoryStack {
        inner: std::cell::RefCell<CategoryStackInner>,
    }

    #[cfg(feature = "instrumentation-enabled")]
    impl CategoryStack {
        pub const fn new() -> Self {
            Self {
                inner: std::cell::RefCell::new(CategoryStackInner {
                    stack: [None; _],
                    head: 0,
                }),
            }
        }

        #[inline]
        fn peek(&self) -> &'static CategoryInfo {
            let inner = self.inner.borrow();
            inner.stack[inner.head].unwrap_or(CategoryInfo::get::<Uncategorized>())
        }

        #[inline]
        fn push(&self, info: &'static CategoryInfo) {
            let mut inner = self.inner.borrow_mut();

            if inner.head == inner.stack.len() {
                panic!("Attempted to push past the max size of the category stack");
            }

            let new_head = usize::min(inner.head + 1, inner.stack.len());
            inner.stack[new_head] = Some(info);
            inner.head = new_head;
        }

        #[inline]
        fn pop(&self) {
            let mut inner = self.inner.borrow_mut();
            let old_head = inner.head;
            inner.stack[old_head] = None;
            inner.head = old_head.saturating_sub(1);
        }
    }

    #[cfg(feature = "instrumentation-enabled")]
    struct CategoryStackInner {
        stack: [Option<&'static CategoryInfo>; 1024],
        head: usize,
    }

    #[cfg(feature = "instrumentation-enabled")]
    impl Default for CategoryStackInner {
        fn default() -> Self {
            Self {
                stack: [None; _],
                head: 0,
            }
        }
    }
}
