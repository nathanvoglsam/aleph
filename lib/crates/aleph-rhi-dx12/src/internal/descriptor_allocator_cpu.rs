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

use crossbeam::queue::SegQueue;
use parking_lot::Mutex;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use windows::core::CanInto;
use windows::utils::CPUDescriptorHandle;
use windows::Win32::Graphics::Direct3D12::*;

pub struct DescriptorAllocatorCPU {
    /// Device handle so we can create new blocks on demand
    device: ID3D12Device,

    /// A lock-free 'free list' that is used as a fast path for allocating new descriptors
    free_list: SegQueue<CPUDescriptorHandle>,

    /// State for the bump allocation layer
    bump_state: BumpState,

    /// List of descriptor blocks
    blocks: Mutex<Vec<ID3D12DescriptorHeap>>,

    /// Metadata needed for creating new blocks
    heap_info: HeapInfo,
}

impl DescriptorAllocatorCPU {
    #[allow(unused)]
    pub fn new(device: &impl CanInto<ID3D12Device>, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> Self {
        let device: ID3D12Device = device.can_clone_into();
        let heap_info = HeapInfo::new(&device, heap_type);
        Self {
            device,
            blocks: Mutex::new(Vec::new()),
            free_list: SegQueue::new(),
            bump_state: BumpState {
                ptr: AtomicUsize::new(0),
                end: AtomicUsize::new(0),
                block_size: AtomicU32::new(128),
            },
            heap_info,
        }
    }

    #[allow(unused)]
    pub fn allocate(&self) -> Option<CPUDescriptorHandle> {
        // First we try to pop from the free list, which is lock free.
        if let Some(item) = self.free_list.pop() {
            Some(item)
        } else {
            self.bump_alloc()
        }
    }

    #[allow(unused)]
    pub fn free(&self, slot: CPUDescriptorHandle) {
        // Free-ing a descriptor is as simple as adding it to the free list
        self.free_list.push(slot);
    }

    fn bump_alloc(&self) -> Option<CPUDescriptorHandle> {
        loop {
            // Load the current bump ptr
            let slot = self.bump_state.ptr.load(Ordering::Acquire);
            let end = self.bump_state.end.load(Ordering::Acquire);

            // Check if we've reached the end of the block. If we have then we need to allocate a
            // new one which requires blocking
            if slot >= end {
                // We need to lock the block list so we can allocate and insert the new block, but
                // we can't use blocks.lock because any thread that enters the allocate function
                // while the lock is owned will just queue and then add another block needlessly.
                //
                // Instead we use try_lock so that only a single thread can enter the
                // bump_alloc_new_block phase, and any other racing threads will enter the
                // WouldBlock condition, where they will still block on the lock but they will not
                // attempt to allocate a new block and instead will just spin once the initial
                // winning thread has finished allocating a new block.
                let first_try = self.blocks.try_lock();
                match first_try {
                    Some(mut blocks) => {
                        // It may be possible for a thread to enter this branch of the
                        // 'if slot >= end', meaning it thinks that there is no space in the block,
                        // while another thread has just finished allocating a new block and
                        // released the lock.
                        //
                        // In this case the 'new block' thread has allocated a new block but the
                        // second racing thread has an out of date copy of the bump state, gains
                        // the lock and allocates another new block. The block allocated by
                        // the first thread will be leaked. By reloading the bump state after
                        // acquiring the lock and rechecking if we actually need to load a new block
                        // we avoid this problem.
                        let slot = self.bump_state.ptr.load(Ordering::Acquire);
                        let end = self.bump_state.end.load(Ordering::Acquire);
                        if slot >= end {
                            return self.bump_alloc_new_block(&mut blocks);
                        } else {
                            continue;
                        }
                    }
                    None => {
                        // If we fail to take the lock we have to wait for the winning thread to create
                        // the new block so we perform a blocking lock to sleep and immediately restart
                        // the loop
                        let _block = self.blocks.lock();
                        continue;
                    }
                }
            } else {
                // This is the mid-speed path, where we don't need to allocate a new block but we
                // do need to allocate from the bump allocator.
                //
                // This is where we hit the atomic compare and swap loop where each thread races in
                // an attempt to win taking ownership of a slot. As long as multiple threads aren't
                // allocating descriptors in a hot loop this should be very fast.
                let next = slot + self.heap_info.increment_size as usize;
                if self
                    .bump_state
                    .ptr
                    .compare_exchange_weak(slot, next, Ordering::Acquire, Ordering::Acquire)
                    .is_ok()
                {
                    return NonZeroUsize::new(slot).map(CPUDescriptorHandle::from);
                } else {
                    continue; // If we were racing and lost we have to try again
                }
            }
        }
    }

    fn bump_alloc_new_block(
        &self,
        blocks: &mut Vec<ID3D12DescriptorHeap>,
    ) -> Option<CPUDescriptorHandle> {
        // Set the bump ptr to usize::MAX to "lock" the bump allocator
        self.bump_state.ptr.store(usize::MAX, Ordering::Relaxed);

        // Create a new block size double the size of the old block size
        let last_block_size = self.bump_state.block_size.load(Ordering::Relaxed);
        let new_block_size = last_block_size * 2;
        self.bump_state
            .block_size
            .store(new_block_size, Ordering::Relaxed);

        // Allocate a new block with the new size
        let new_block = Self::new_block(&self.device, new_block_size, self.heap_info.heap_type);

        // Get the CPU handle for the start of the new block and calculate the new initial values
        // for the bump allocator
        let new_base = unsafe {
            // Get the handle for the heap start and assert it's not a null handle
            let base = new_block.GetCPUDescriptorHandleForHeapStart();
            let base = CPUDescriptorHandle::try_from(base).unwrap();
            base.get_inner().get()
        };
        let new_end = new_base + (new_block_size as usize * self.heap_info.increment_size as usize);

        // Very deliberately we write the end value while .ptr is still usize::MAX so no matter what
        // we write into .end the
        self.bump_state.end.store(new_end, Ordering::Relaxed);

        // We offset the base by the size of a single slot to make space for the allocation we need
        // to return.
        //
        // This operation unlocks the bump allocator and leaves it in a state that is valid to
        // allocate from other threads.
        //
        // It is perfectly valid to overlap other threads with this stage of the new block creation
        // as it is not possible for more than one thread to enter this function at any one time
        // anyway. If we're still in this function and the bump allocator was already exhausted by
        // other threads then all over threads will spin on the lock this function holds.
        self.bump_state.ptr.store(
            new_base + self.heap_info.increment_size as usize,
            Ordering::Relaxed,
        );

        // Add new new block to the block list so we can clean it up later
        blocks.push(new_block);

        // Return our new descriptor handle
        NonZeroUsize::new(new_base).map(CPUDescriptorHandle::from)
    }
}

impl DescriptorAllocatorCPU {
    fn new_block(
        device: &ID3D12Device,
        num_descriptors: u32,
        heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    ) -> ID3D12DescriptorHeap {
        unsafe {
            let desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: heap_type,
                NumDescriptors: num_descriptors,
                Flags: Default::default(),
                NodeMask: 0,
            };
            device.CreateDescriptorHeap(&desc).unwrap()
        }
    }
}

struct BumpState {
    ptr: AtomicUsize,
    end: AtomicUsize,
    block_size: AtomicU32,
}

struct HeapInfo {
    heap_type: D3D12_DESCRIPTOR_HEAP_TYPE,
    increment_size: u32,
}

impl HeapInfo {
    pub fn new(device: &ID3D12Device, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> Self {
        // Safety: the function is thread safe, there is no safety issue
        unsafe {
            Self {
                heap_type,
                increment_size: device.GetDescriptorHandleIncrementSize(heap_type),
            }
        }
    }
}
