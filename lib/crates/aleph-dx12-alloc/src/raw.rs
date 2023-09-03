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

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::Allocation;
use bitflags::bitflags;
use std::ffi::c_void;
use std::num::NonZeroU64;
use windows::core::{GUID, HRESULT};
use windows::utils::WeakRef;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::IDXGIAdapter;

// ============================================================================================== //
// Allocation
// ============================================================================================== //

/// Unique identifier of single allocation done inside the memory heap.
pub type AllocHandle = NonZeroU64;

bitflags! {
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ALLOCATION_FLAGS: u32 {
        /// Set this flag if the allocation should have its own dedicated memory allocation
        /// (committed resource with implicit heap).
        ///
        /// Use it for special, big resources, like fullscreen textures used as render targets.
        ///
        /// - When used with functions like D3D12MA::Allocator::CreateResource, it will use
        ///   `ID3D12Device::CreateCommittedResource`, so the created allocation will contain a
        ///   resource (D3D12MA::Allocation::GetResource() `!= NULL`) but will not have a heap
        ///   (D3D12MA::Allocation::GetHeap() `== NULL`), as the heap is implicit.
        /// - When used with raw memory allocation like D3D12MA::Allocator::AllocateMemory, it will
        ///   use `ID3D12Device::CreateHeap`, so the created allocation will contain a heap
        ///   (D3D12MA::Allocation::GetHeap() `!= NULL`) and its offset will always be 0.
        const COMMITTED = 0x1;

        /// Set this flag to only try to allocate from existing memory heaps and never create new
        /// such heap.
        ///
        /// If new allocation cannot be placed in any of the existing heaps, allocation fails with
        /// `E_OUTOFMEMORY` error.
        ///
        /// You should not use D3D12MA::ALLOCATION_FLAG_COMMITTED and
        /// D3D12MA::ALLOCATION_FLAG_NEVER_ALLOCATE at the same time. It makes no sense.
        const NEVER_ALLOCATE = 0x2;

        /// Create allocation only if additional memory required for it, if any, won't exceed
        /// memory budget. Otherwise return `E_OUTOFMEMORY`.
        const WITHIN_BUDGET = 0x4;

        /// Allocation will be created from upper stack in a double stack pool.
        ///
        /// This flag is only allowed for custom pools created with #POOL_FLAG_ALGORITHM_LINEAR
        /// flag.
        const UPPER_ADDRESS = 0x8;

        /// Set this flag if the allocated memory will have aliasing resources.
        ///
        /// Use this when calling D3D12MA::Allocator::CreateResource() and similar to guarantee
        /// creation of explicit heap for desired allocation and prevent it from using
        /// `CreateCommittedResource`, so that new allocation object will always have
        /// `allocation->GetHeap() != NULL`.
        const CAN_ALIAS = 0x10;

        /// Allocation strategy that chooses smallest possible free range for the allocation
        /// to minimize memory usage and fragmentation, possibly at the expense of allocation time.
        const STRATEGY_MIN_MEMORY = 0x00010000;

        /// Allocation strategy that chooses first suitable free range for the allocation - not
        /// necessarily in terms of the smallest offset but the one that is easiest and fastest to
        /// find to minimize allocation time, possibly at the expense of allocation quality.
        const STRATEGY_MIN_TIME = 0x00020000;

        /// Allocation strategy that chooses always the lowest offset in available space. This is
        /// not the most efficient strategy but achieves highly packed data. Used internally by
        /// defragmentation, not recomended in typical usage.
        const STRATEGY_MIN_OFFSET = 0x0004000;

        /// Alias to STRATEGY_MIN_MEMORY.
        const STRATEGY_BEST_FIT = Self::STRATEGY_MIN_MEMORY.bits();

        /// Alias to STRATEGY_MIN_TIME.
        const STRATEGY_FIRST_FIT = Self::STRATEGY_MIN_TIME.bits();

        /// A bit mask to extract only `STRATEGY` bits from entire set of flags.
        const STRATEGY_MASK = Self::STRATEGY_MIN_MEMORY.bits()
            | Self::STRATEGY_MIN_TIME.bits()
            | Self::STRATEGY_MIN_OFFSET.bits();
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub struct ALLOCATION_DESC {
    /// Flags.
    pub Flags: ALLOCATION_FLAGS,

    /// The type of memory heap where the new allocation should be placed.
    ///
    /// It must be one of: `D3D12_HEAP_TYPE_DEFAULT`, `D3D12_HEAP_TYPE_UPLOAD`,
    /// `D3D12_HEAP_TYPE_READBACK`.
    ///
    /// When D3D12MA::ALLOCATION_DESC::CustomPool != NULL this member is ignored.
    pub HeapType: D3D12_HEAP_TYPE,

    /// dditional heap flags to be used when allocating memory.
    ///
    /// In most cases it can be 0.
    ///
    /// - If you use D3D12MA::Allocator::CreateResource(), you don't need to care. Necessary flag
    ///   `D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS`, `D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES`, or
    ///   `D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES` is added automatically.
    /// - If you use D3D12MA::Allocator::AllocateMemory(), you should specify one of those
    ///   `ALLOW_ONLY` flags. Except when you validate that
    ///   `D3D12MA::Allocator::GetD3D12Options().ResourceHeapTier == D3D12_RESOURCE_HEAP_TIER_1` -
    ///   then you can leave it 0.
    /// - You can specify additional flags if needed. Then the memory will always be allocated as
    ///   separate block using `D3D12Device::CreateCommittedResource` or `CreateHeap`, not as part
    ///   of an existing larget block.
    ///
    /// When D3D12MA::ALLOCATION_DESC::CustomPool != NULL this member is ignored.
    pub ExtraHeapFlags: D3D12_HEAP_FLAGS,

    /// Custom pool to place the new resource in. Optional.
    ///
    /// When not NULL, the resource will be created inside specified custom pool.
    /// It will then never be created as committed.
    pub Pool: *mut c_void,

    /// Custom general-purpose pointer that will be stored in D3D12MA::Allocation.
    pub pPrivateData: *mut c_void,
}

// ============================================================================================== //
// Allocator
// ============================================================================================== //

bitflags! {
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct ALLOCATOR_FLAGS: u32 {
        /// Allocator and all objects created from it will not be synchronized internally,
        /// so you must guarantee they are used from only one thread at a time or
        /// synchronized by you.
        ///
        /// Using this flag may increase performance because internal mutexes are not used.
        const SINGLE_THREADED = 0b1;

        /// Every allocation will have its own memory block. To be used for debugging purposes.
        const ALWAYS_COMMITTED = 0b10;

        /// Heaps created for the default pools will be created with flag
        /// `D3D12_HEAP_FLAG_CREATE_NOT_ZEROED`, allowing for their memory to be not zeroed by the
        /// system if possible, which can speed up allocation.
        ///
        /// Only affects default pools. To use the flag with @ref custom_pools, you need to add it
        /// manually:
        ///
        /// `poolDesc.heapFlags |= D3D12_HEAP_FLAG_CREATE_NOT_ZEROED;`
        ///
        /// Only avaiable if `ID3D12Device8` is present. Otherwise, the flag is ignored.
        const DEFAULT_POOLS_NOT_ZEROED = 0x4;

        /// Optimization, allocate MSAA textures as committed resources always.
        ///
        /// Specify this flag to create MSAA textures with implicit heaps, as if they were created
        /// with flag ALLOCATION_FLAG_COMMITTED. Usage of this flags enables all default pools
        /// to create its heaps on smaller alignment not suitable for MSAA textures.
        ///
        const MSAA_TEXTURES_ALWAYS_COMMITTED = 0x8;
    }
}

/// Pointer to custom callback function that allocates CPU memory.
pub type ALLOCATE_FN = extern "C" fn(usize, usize, *mut c_void) -> *mut c_void;

/// Pointer to custom callback function that deallocates CPU memory.
///
/// `pMemory = null` should be accepted and ignored.
pub type FREE_FN = extern "C" fn(*mut c_void, *mut c_void);

/// Custom callbacks to CPU memory allocation functions.
#[repr(C)]
pub struct ALLOCATION_CALLBACKS {
    /// Allocation function.
    pub pAllocate: ALLOCATE_FN,

    /// De-allocation function.
    pub pFree: FREE_FN,

    /// Custom data that will be passed to allocation and deallocation functions as `pUserData`
    /// parameter.
    pub pPrivateData: *mut c_void,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub struct ALLOCATOR_DESC {
    /// Flags.
    pub Flags: ALLOCATOR_FLAGS,

    /// Direct3D device object that the allocator should be attached to.
    ///
    /// Allocator is doing `AddRef`/`Release` on this object.
    pub pDevice: Option<ID3D12Device>,

    /// Preferred size of a single `ID3D12Heap` block to be allocated.
    ///
    /// Set to 0 to use default, which is currently 64 MiB.
    pub PreferredBlockSize: u64,

    /// Custom CPU memory allocation callbacks. Optional.
    ///
    /// Optional, can be null. When specified, will be used for all CPU-side memory allocations.
    pub pAllocationCallbacks: *const ALLOCATION_CALLBACKS,

    /// DXGI Adapter object that you use for D3D12 and this allocator.
    ///
    /// Allocator is doing `AddRef`/`Release` on this object.
    ///
    pub pAdapter: Option<IDXGIAdapter>,
}

// ============================================================================================== //
// Pool
// ============================================================================================== //

bitflags! {
    /// Bit flags to be used with D3D12MA_POOL_FLAGS::Flags.
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct POOL_FLAGS: u32 {
        /// Enables alternative, linear allocation algorithm in this pool.
        ///
        /// Specify this flag to enable linear allocation algorithm, which always creates
        /// new allocations after last one and doesn't reuse space from allocations freed in
        /// between. It trades memory consumption for simplified algorithm and data
        /// structure, which has better performance and uses less memory for metadata.
        ///
        /// By using this flag, you can achieve behavior of free-at-once, stack,
        /// ring buffer, and double stack.
        /// For details, see documentation chapter \ref linear_algorithm.
        ///
        const ALGORITHM_LINEAR = 0x1;

        /// Optimization, allocate MSAA textures as committed resources always.
        ///
        /// Specify this flag to create MSAA textures with implicit heaps, as if they were created
        /// with flag ALLOCATION_FLAG_COMMITTED. Usage of this flags enables pool to create its
        /// heaps on smaller alignment not suitable for MSAA textures.
        const MSAA_TEXTURES_ALWAYS_COMMITTED = 0x2;

        /// Bit mask to extract only `ALGORITHM` bits from entire set of flags.
        const ALGORITHM_MASK = Self::ALGORITHM_LINEAR.bits();
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Default)]
pub struct POOL_DESC {
    /// Flags.
    pub Flags: POOL_FLAGS,

    /// The parameters of memory heap where allocations of this pool should be placed.
    ///
    /// In the simplest case, just fill it with zeros and set `Type` to one of:
    /// `D3D12_HEAP_TYPE_DEFAULT`, `D3D12_HEAP_TYPE_UPLOAD`, `D3D12_HEAP_TYPE_READBACK`. Additional
    /// parameters can be used e.g. to utilize UMA.
    pub HeapProperties: D3D12_HEAP_PROPERTIES,

    /// Heap flags to be used when allocating heaps of this pool.
    ///
    /// It should contain one of these values, depending on type of resources you are going to
    /// create in this heap:
    /// - `D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS`
    /// - `D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES`
    /// - `D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES`
    /// Except if ResourceHeapTier = 2, then it may be
    /// `D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES` = 0.
    ///
    /// You can specify additional flags if needed.
    pub HeapFlags: D3D12_HEAP_FLAGS,

    /// Size of a single heap (memory block) to be allocated as part of this pool, in bytes.
    /// Optional.
    ///
    /// Specify nonzero to set explicit, constant size of memory blocks used by this pool.
    /// Leave 0 to use default and let the library manage block sizes automatically.
    /// Then sizes of particular blocks may vary.
    pub BlockSize: u64,

    /// Minimum number of heaps (memory blocks) to be always allocated in this pool, even if they
    /// stay empty. Optional.
    ///
    /// Set to 0 to have no preallocated blocks and allow the pool be completely empty.
    pub MinBlockCount: u32,

    /// Maximum number of heaps (memory blocks) that can be allocated in this pool. Optional.
    ///
    /// Set to 0 to use default, which is `UINT64_MAX`, which means no limit.
    ///
    /// Set to same value as D3D12MA_POOL_DESC::MinBlockCount to have fixed amount of memory
    /// allocated throughout whole lifetime of this pool.
    pub MaxBlockCount: u32,

    /// Additional minimum alignment to be used for all allocations created from this pool. Can be
    /// 0.
    ///
    /// Leave 0 (default) not to impose any additional alignment. If not 0, it must be a power of
    /// two.
    pub MinAllocationAlignment: u64,

    /// Additional parameter allowing pool to create resources with passed protected session.
    ///
    /// If not null then all the heaps and committed resources will be created with this parameter.
    pub pProtectedSession: Option<ID3D12ProtectedResourceSession>,
}

// ============================================================================================== //
// Defragmentation
// ============================================================================================== //

bitflags! {
    /// Bit flags to be used with VIRTUAL_BLOCK_DESC::Flags.
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct DEFRAGMENTATION_FLAGS: u32 {
        /// Use simple but fast algorithm for defragmentation.
        ///
        /// May not achieve best results but will require least time to compute and least
        /// allocations to copy.
        const ALGORITHM_FAST = 0x1;

        /// Default defragmentation algorithm, applied also when no `ALGORITHM` flag is specified.
        ///
        /// Offers a balance between defragmentation quality and the amount of allocations and bytes
        /// that need to be moved.
        const ALGORITHM_BALANCED = 0x2;

        /// Perform full defragmentation of memory.
        ///
        /// Can result in notably more time to compute and allocations to copy, but will achieve
        /// best memory packing.
        const ALGORITHM_FULL = 0x4;

        /// A bit mask to extract only `ALGORITHM` bits from entire set of flags.
        const ALGORITHM_MASK = Self::ALGORITHM_FAST.bits()
            | Self::ALGORITHM_BALANCED.bits()
            | Self::ALGORITHM_FULL.bits();
    }
}

/// Parameters for defragmentation.
///
/// To be used with functions Allocator::BeginDefragmentation() and Pool::BeginDefragmentation().
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct DEFRAGMENTATION_DESC {
    /// Flags.
    pub Flags: DEFRAGMENTATION_FLAGS,

    /// Maximum numbers of bytes that can be copied during single pass, while moving allocations to
    /// different places.
    ///
    /// 0 means no limit.
    pub MaxBytesPerPass: u64,

    /// Maximum number of allocations that can be moved during single pass to a different place.
    ///
    /// 0 means no limit.
    pub MaxAllocationsPerPass: u32,
}

/// Operation performed on single defragmentation move.
#[repr(u32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum DEFRAGMENTATION_MOVE_OPERATION {
    /// Resource has been recreated at `pDstTmpAllocation`, data has been copied, old resource has
    /// been destroyed.
    ///
    /// `pSrcAllocation` will be changed to point to the new place. This is the default value set by
    /// DefragmentationContext::BeginPass().
    COPY = 0,

    /// Set this value if you cannot move the allocation. New place reserved at `pDstTmpAllocation`
    /// will be freed. `pSrcAllocation` will remain unchanged.
    IGNORE = 1,

    /// Set this value if you decide to abandon the allocation and you destroyed the resource. New
    /// place reserved `pDstTmpAllocation` will be freed, along with `pSrcAllocation`.
    DESTROY = 2,
}

impl Default for DEFRAGMENTATION_MOVE_OPERATION {
    #[inline(always)]
    fn default() -> Self {
        Self::COPY
    }
}

/// Single move of an allocation to be done for defragmentation.
#[repr(C)]
pub struct DEFRAGMENTATION_MOVE {
    /// Operation to be performed on the allocation by DefragmentationContext::EndPass().
    ///
    /// Default value is #DEFRAGMENTATION_MOVE_OPERATION_COPY. You can modify it.
    pub Operation: DEFRAGMENTATION_MOVE_OPERATION,

    /// %Allocation that should be moved.
    pub pSrcAllocation: Allocation, // TODO: determine if this is meant to be weak

    /// Temporary allocation pointing to destination memory that will replace `pSrcAllocation`.
    ///
    /// Use it to retrieve new `ID3D12Heap` and offset to create new `ID3D12Resource` and then store
    /// it here via Allocation::SetResource().
    ///
    /// # Warning
    ///
    /// Do not store this allocation in your data structures! It exists only temporarily, for the
    /// duration of the defragmentation pass, to be used for storing newly created resource.
    /// DefragmentationContext::EndPass() will destroy it and make `pSrcAllocation` point to this
    /// memory.
    pub pDstTmpAllocation: Allocation, // TODO: determine if this is meant to be weak
}

/// Parameters for incremental defragmentation steps.
///
/// To be used with function DefragmentationContext::BeginPass().
#[repr(C)]
pub struct DEFRAGMENTATION_PASS_MOVE_INFO {
    /// Number of elements in the `pMoves` array.
    pub MoveCount: u32,

    /// Array of moves to be performed by the user in the current defragmentation pass.
    ///
    /// Pointer to an array of `MoveCount` elements, owned by %D3D12MA, created in
    /// DefragmentationContext::BeginPass(), destroyed in DefragmentationContext::EndPass().
    ///
    /// For each element, you should:
    ///
    /// 1. Create a new resource in the place pointed by `pMoves[i].pDstTmpAllocation->GetHeap()` +
    ///    `pMoves[i].pDstTmpAllocation->GetOffset()`.
    /// 2. Store new resource in `pMoves[i].pDstTmpAllocation` by using Allocation::SetResource().
    ///    It will later replace old resource from `pMoves[i].pSrcAllocation`.
    /// 3. Copy data from the `pMoves[i].pSrcAllocation` e.g. using
    ///    `D3D12GraphicsCommandList::CopyResource`.
    /// 4. Make sure these commands finished executing on the GPU.
    ///
    /// Only then you can finish defragmentation pass by calling DefragmentationContext::EndPass().
    /// After this call, the allocation will point to the new place in memory.
    ///
    /// Alternatively, if you cannot move specific allocation,
    /// you can set DEFRAGMENTATION_MOVE::Operation to
    /// D3D12MA::DEFRAGMENTATION_MOVE_OPERATION_IGNORE.
    ///
    /// Alternatively, if you decide you want to completely remove the allocation,
    /// set DEFRAGMENTATION_MOVE::Operation to D3D12MA::DEFRAGMENTATION_MOVE_OPERATION_DESTROY.
    /// Then, after DefragmentationContext::EndPass() the allocation will be released.
    pub pMoves: *mut DEFRAGMENTATION_MOVE,
}

/// %Statistics returned for defragmentation process by function DefragmentationContext::GetStats().
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct DEFRAGMENTATION_STATS {
    /// Total number of bytes that have been copied while moving allocations to different places.
    pub BytesMoved: u64,

    /// Total number of bytes that have been released to the system by freeing empty heaps.
    pub BytesFreed: u64,

    /// Number of allocations that have been moved to different places.
    pub AllocationsMoved: u32,

    /// Number of empty `ID3D12Heap` objects that have been released to the system.
    pub HeapsFreed: u32,
}

// ============================================================================================== //
// Statistics
// ============================================================================================== //

/// Calculated statistics of memory usage e.g. in a specific memory heap type,
/// memory segment group, custom pool, or total.
///
/// These are fast to calculate.
/// See functions: D3D12MA::Allocator::GetBudget(), D3D12MA::Pool::GetStatistics().
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Statistics {
    /// Number of D3D12 memory blocks allocated - `ID3D12Heap` objects and committed resources.
    pub BlockCount: u32,

    /// Number of D3D12MA::Allocation objects allocated.
    ///
    /// Committed allocations have their own blocks, so each one adds 1 to `AllocationCount` as well
    /// as `BlockCount`.
    pub AllocationCount: u32,

    /// Number of bytes allocated in memory blocks.
    pub BlockBytes: u64,

    /// Total number of bytes occupied by all D3D12MA::Allocation objects.
    ///
    /// Always less or equal than `BlockBytes`.
    /// Difference `(BlockBytes - AllocationBytes)` is the amount of memory allocated from D3D12
    /// but unused by any D3D12MA::Allocation.
    pub AllocationBytes: u64,
}

/// More detailed statistics than D3D12MA::Statistics.
///
/// These are slower to calculate. Use for debugging purposes.
/// See functions: D3D12MA::Allocator::CalculateStatistics(), D3D12MA::Pool::CalculateStatistics().
///
/// Averages are not provided because they can be easily calculated as:
///
/// ```{ignore}
/// let AllocationSizeAvg = DetailedStats.Statistics.AllocationBytes / detailedStats.Statistics.AllocationCount;
/// let UnusedBytes = DetailedStats.Statistics.BlockBytes - DetailedStats.Statistics.AllocationBytes;
/// let UnusedRangeSizeAvg = UnusedBytes / DetailedStats.UnusedRangeCount;
/// ```
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct DetailedStatistics {
    /// Basic statistics.
    pub Stats: Statistics,

    /// Number of free ranges of memory between allocations.
    pub UnusedRangeCount: u32,

    /// Smallest allocation size. `UINT64_MAX` if there are 0 allocations.
    pub AllocationSizeMin: u64,

    /// Largest allocation size. 0 if there are 0 allocations.
    pub AllocationSizeMax: u64,

    /// Smallest empty range size. `UINT64_MAX` if there are 0 empty ranges.
    pub UnusedRangeSizeMin: u64,

    /// Largest empty range size. 0 if there are 0 empty ranges.
    pub UnusedRangeSizeMax: u64,
}

/// General statistics from current state of the allocator -
/// total memory usage across all memory heaps and segments.
///
/// These are slower to calculate. Use for debugging purposes.
/// See function D3D12MA::Allocator::CalculateStatistics().
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct TotalStatistics {
    /// One element for each type of heap located at the following indices:
    ///
    /// - 0 = `D3D12_HEAP_TYPE_DEFAULT`
    /// - 1 = `D3D12_HEAP_TYPE_UPLOAD`
    /// - 2 = `D3D12_HEAP_TYPE_READBACK`
    /// - 3 = `D3D12_HEAP_TYPE_CUSTOM`
    pub HeapType: [DetailedStatistics; 4],

    /// One element for each memory segment group located at the following indices:
    ///
    /// - 0 = `DXGI_MEMORY_SEGMENT_GROUP_LOCAL`
    /// - 1 = `DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL`
    ///
    /// Meaning of these segment groups is:
    ///
    /// - When `IsUMA() == FALSE` (discrete graphics card):
    ///   - `DXGI_MEMORY_SEGMENT_GROUP_LOCAL` (index 0) represents GPU memory
    ///   (resources allocated in `D3D12_HEAP_TYPE_DEFAULT` or `D3D12_MEMORY_POOL_L1`).
    ///   - `DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL` (index 1) represents system memory
    ///     (resources allocated in `D3D12_HEAP_TYPE_UPLOAD`, `D3D12_HEAP_TYPE_READBACK`, or
    ///     `D3D12_MEMORY_POOL_L0`).
    /// - When `IsUMA() == TRUE` (integrated graphics chip):
    ///   - `DXGI_MEMORY_SEGMENT_GROUP_LOCAL` = (index 0) represents memory shared for all the
    ///     resources.
    ///   - `DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL` = (index 1) is unused and always 0.
    ///
    pub MemorySegmentGroup: [DetailedStatistics; 2],

    /// Total statistics from all memory allocated from D3D12.
    pub Total: DetailedStatistics,
}

/// %Statistics of current memory usage and available budget for a specific memory segment group.
///
/// These are fast to calculate. See function D3D12MA::Allocator::GetBudget().
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Budget {
    /// %Statistics fetched from the library.
    pub Stats: Statistics,

    /// Estimated current memory usage of the program.
    ///
    /// Fetched from system using `IDXGIAdapter3::QueryVideoMemoryInfo` if possible.
    ///
    /// It might be different than `BlockBytes` (usually higher) due to additional implicit objects
    /// also occupying the memory, like swapchain, pipeline state objects, descriptor heaps, command
    /// lists, or heaps and resources allocated outside of this library, if any.
    pub UsageBytes: u64,

    /// Estimated amount of memory available to the program.
    ///
    /// Fetched from system using `IDXGIAdapter3::QueryVideoMemoryInfo` if possible.
    ///
    /// It might be different (most probably smaller) than memory capacity returned
    /// by D3D12MA::Allocator::GetMemoryCapacity() due to factors
    /// external to the program, decided by the operating system.
    /// Difference `BudgetBytes - UsageBytes` is the amount of additional memory that can probably
    /// be allocated without problems. Exceeding the budget may result in various problems.
    pub BudgetBytes: u64,
}

// ============================================================================================== //
// Virtual Block
// ============================================================================================== //

/// Represents single memory allocation done inside VirtualBlock.
#[repr(transparent)]
pub struct VirtualAllocation(AllocHandle);

bitflags! {
    /// Bit flags to be used with VIRTUAL_BLOCK_DESC::Flags.
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct VIRTUAL_BLOCK_FLAGS: u32 {
        /// Enables alternative, linear allocation algorithm in this virtual block.
        ///
        /// Specify this flag to enable linear allocation algorithm, which always creates
        /// new allocations after last one and doesn't reuse space from allocations freed in
        /// between. It trades memory consumption for simplified algorithm and data
        /// structure, which has better performance and uses less memory for metadata.
        ///
        /// By using this flag, you can achieve behavior of free-at-once, stack,
        /// ring buffer, and double stack.
        /// For details, see documentation chapter \ref linear_algorithm.
        ///
        const ALGORITHM_LINEAR = POOL_FLAGS::ALGORITHM_LINEAR.bits();

        /// Bit mask to extract only `ALGORITHM` bits from entire set of flags.
        const ALGORITHM_MASK = POOL_FLAGS::ALGORITHM_MASK.bits();
    }
}

/// Parameters of created D3D12MA::VirtualBlock object to be passed to CreateVirtualBlock().
#[repr(C)]
pub struct VIRTUAL_BLOCK_DESC {
    /// Flags.
    pub Flags: VIRTUAL_BLOCK_FLAGS,

    /// Total size of the block.
    ///
    /// Sizes can be expressed in bytes or any units you want as long as you are consistent in using
    /// them. For example, if you allocate from some array of structures, 1 can mean single instance
    /// of entire structure.
    pub Size: u64,

    /// Custom CPU memory allocation callbacks. Optional.
    ///
    /// Optional, can be null. When specified, will be used for all CPU-side memory allocations.
    pub pAllocationCallbacks: *const ALLOCATION_CALLBACKS,
}

bitflags! {
    /// Bit flags to be used with VIRTUAL_ALLOCATION_DESC::Flags.
    #[repr(transparent)]
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct VIRTUAL_ALLOCATION_FLAGS: u32 {
        /// Allocation will be created from upper stack in a double stack pool.
        ///
        /// This flag is only allowed for virtual blocks created with
        /// #VIRTUAL_BLOCK_FLAG_ALGORITHM_LINEAR flag.
        const UPPER_ADDRESS = ALLOCATION_FLAGS::UPPER_ADDRESS.bits();

        /// Allocation strategy that tries to minimize memory usage.
        const STRATEGY_MIN_MEMORY = ALLOCATION_FLAGS::STRATEGY_MIN_MEMORY.bits();

        /// Allocation strategy that tries to minimize allocation time.
        const STRATEGY_MIN_TIME = ALLOCATION_FLAGS::STRATEGY_MIN_TIME.bits();

        /// Allocation strategy that chooses always the lowest offset in available space.
        ///
        /// This is not the most efficient strategy but achieves highly packed data.
        const STRATEGY_MIN_OFFSET = ALLOCATION_FLAGS::STRATEGY_MIN_OFFSET.bits();

        /// A bit mask to extract only `STRATEGY` bits from entire set of flags.
        ///
        /// These strategy flags are binary compatible with equivalent flags in #ALLOCATION_FLAGS.
        const STRATEGY_MASK = ALLOCATION_FLAGS::STRATEGY_MASK.bits();
    }
}

/// Parameters of created virtual allocation to be passed to VirtualBlock::Allocate().
#[repr(C)]
pub struct VIRTUAL_ALLOCATION_DESC {
    /// Flags.
    pub Flags: VIRTUAL_ALLOCATION_FLAGS,

    /// Size of the allocation.
    ///
    /// Cannot be zero.
    pub Size: u64,

    /// Required alignment of the allocation.
    ///
    /// Must be power of two. Special value 0 has the same meaning as 1 - means no special alignment
    /// is required, so allocation can start at any offset.
    pub Alignment: u64,

    /// Custom pointer to be associated with the allocation.
    ///
    /// It can be fetched or changed later.
    pub pPrivateData: *mut c_void,
}

/// Parameters of an existing virtual allocation, returned by VirtualBlock::GetAllocationInfo().
#[repr(C)]
pub struct VIRTUAL_ALLOCATION_INFO {
    /// Offset of the allocation.
    pub Offset: u64,

    /// Size of the allocation.
    ///
    /// Same value as passed in VIRTUAL_ALLOCATION_DESC::Size.
    pub Size: u64,

    /// Custom pointer associated with the allocation.
    ///
    /// Same value as passed in VIRTUAL_ALLOCATION_DESC::pPrivateData or
    /// VirtualBlock::SetAllocationPrivateData().
    pub pPrivateData: *mut c_void,
}

// ============================================================================================== //
// Functions
// ============================================================================================== //

pub type ThisPtr = *mut c_void;
pub type ThisPtrConst = *const c_void;

#[allow(unused)]
extern "C" {
    //
    // ALLOCATION
    //
    pub fn D3D12MA_Allocation_Release(this: ThisPtr);
    pub fn D3D12MA_Allocation_GetOffset(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetAlignment(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetSize(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetResource(this: ThisPtrConst) -> Option<ID3D12Resource>;
    pub fn D3D12MA_Allocation_SetResource(this: ThisPtr, pResource: WeakRef<ID3D12Resource>);
    pub fn D3D12MA_Allocation_GetHeap(this: ThisPtrConst) -> Option<ID3D12Heap>;
    pub fn D3D12MA_Allocation_SetPrivateData(this: ThisPtr, pPrivateData: *mut c_void);
    pub fn D3D12MA_Allocation_GetPrivateData(this: ThisPtrConst) -> *mut c_void;
    pub fn D3D12MA_Allocation_SetName(this: ThisPtr, Name: *const u16);
    pub fn D3D12MA_Allocation_GetName(this: ThisPtrConst) -> *const u16;
    pub fn D3D12MA_Allocation_WasZeroInitialized(this: ThisPtrConst) -> BOOL;

    //
    // POOL
    //
    pub fn D3D12MA_Pool_Release(this: ThisPtr);
    pub fn D3D12MA_Pool_GetDesc(this: ThisPtrConst) -> POOL_DESC;
    pub fn D3D12MA_Pool_GetStatistics(this: ThisPtr, pStats: *mut Statistics);
    pub fn D3D12MA_Pool_CalculateStatistics(this: ThisPtr, pStats: *mut DetailedStatistics);
    pub fn D3D12MA_Pool_SetName(this: ThisPtr, Name: *const u16);
    pub fn D3D12MA_Pool_GetName(this: ThisPtrConst) -> *const u16;
    pub fn D3D12MA_Pool_BeginDefragmentation(
        this: ThisPtr,
        pDesc: *const DEFRAGMENTATION_DESC,
        ppContext: *mut *mut c_void,
    );

    //
    // ALLOCATOR
    //
    pub fn D3D12MA_Allocator_CreateAllocator(
        pDesc: *const ALLOCATOR_DESC,
        ppAllocator: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_Release(this: ThisPtr);
    pub fn D3D12MA_Allocator_GetD3D12Options(
        this: ThisPtrConst,
    ) -> *const D3D12_FEATURE_DATA_D3D12_OPTIONS;
    pub fn D3D12MA_Allocator_IsUMA(this: ThisPtrConst) -> BOOL;
    pub fn D3D12MA_Allocator_IsCacheCoherentUMA(this: ThisPtrConst) -> BOOL;
    pub fn D3D12MA_Allocator_GetMemoryCapacity(this: ThisPtrConst, memorySegmentGroup: u32) -> u64;
    pub fn D3D12MA_Allocator_CreateResource(
        this: ThisPtr,
        pAllocDesc: *const ALLOCATION_DESC,
        pResourceDesc: *const D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        ppAllocation: *mut *mut c_void,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreateResource2(
        this: ThisPtr,
        pAllocDesc: *const ALLOCATION_DESC,
        pResourceDesc: *const D3D12_RESOURCE_DESC1,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        ppAllocation: *mut *mut c_void,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_AllocateMemory(
        this: ThisPtr,
        pAllocDesc: *const ALLOCATION_DESC,
        pAllocInfo: *const D3D12_RESOURCE_ALLOCATION_INFO,
        ppAllocation: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreateAliasingResource(
        this: ThisPtr,
        pAllocation: *mut c_void,
        AllocationLocalOffset: u64,
        pResourceDesc: *const D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreatePool(
        this: ThisPtr,
        pPoolDesc: *const POOL_DESC,
        ppPool: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_SetCurrentFrameIndex(this: ThisPtr, FrameIndex: u32);
    pub fn D3D12MA_Allocator_GetBudget(
        this: ThisPtr,
        pLocalBudget: *mut Budget,
        pNonLocalBudget: *mut Budget,
    );
    pub fn D3D12MA_Allocator_CalculateStatistics(this: ThisPtr, pStats: *mut TotalStatistics);
    pub fn D3D12MA_Allocator_BuildStatsString(
        this: ThisPtrConst,
        ppStatsString: *mut *mut u16,
        DetailedMap: BOOL,
    );
    pub fn D3D12MA_Allocator_FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);
    pub fn D3D12MA_Allocator_BeginDefragmentation(
        this: ThisPtr,
        pDesc: *const DEFRAGMENTATION_DESC,
        ppContext: *mut *mut c_void,
    );

    //
    // VIRTUAL BLOCK
    //
    pub fn D3D12MA_VirtualBlock_CreateVirtualBlock(
        pDesc: *const VIRTUAL_BLOCK_DESC,
        ppAllocator: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_VirtualBlock_Release(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_IsEmpty(this: ThisPtrConst) -> BOOL;
    pub fn D3D12MA_VirtualBlock_GetAllocationInfo(
        this: ThisPtrConst,
        allocation: VirtualAllocation,
        pInfo: *mut VIRTUAL_ALLOCATION_INFO,
    );
    pub fn D3D12MA_VirtualBlock_Allocate(
        this: ThisPtr,
        pDesc: *const VIRTUAL_ALLOCATION_DESC,
        pAllocation: *mut Option<VirtualAllocation>,
        pOffset: *mut u64,
    ) -> HRESULT;
    pub fn D3D12MA_VirtualBlock_FreeAllocation(this: ThisPtr, allocation: VirtualAllocation);
    pub fn D3D12MA_VirtualBlock_Clear(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_SetAllocationPrivateData(
        this: ThisPtr,
        allocation: VirtualAllocation,
        pUserData: *mut c_void,
    );
    pub fn D3D12MA_VirtualBlock_GetStatistics(this: ThisPtrConst, pStats: *mut Statistics);
    pub fn D3D12MA_VirtualBlock_CalculateStatistics(
        this: ThisPtrConst,
        pStats: *mut DetailedStatistics,
    );
    pub fn D3D12MA_VirtualBlock_BuildStatsString(this: ThisPtrConst, ppStatsString: *mut *mut u16);
    pub fn D3D12MA_VirtualBlock_FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);

    //
    // DEFRAGMENTATION CONTEXT
    //
    pub fn D3D12MA_DefragmentationContext_Release(this: ThisPtr);
    pub fn D3D12MA_DefragmentationContext_BeginPass(
        this: ThisPtr,
        pPassInfo: *mut DEFRAGMENTATION_PASS_MOVE_INFO,
    ) -> HRESULT;
    pub fn D3D12MA_DefragmentationContext_EndPass(
        this: ThisPtr,
        pPassInfo: *mut DEFRAGMENTATION_PASS_MOVE_INFO,
    ) -> HRESULT;
    pub fn D3D12MA_DefragmentationContext_GetStats(
        this: ThisPtr,
        pStats: *mut DEFRAGMENTATION_STATS,
    );

}
