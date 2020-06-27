//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::defer::{AllocatorDeferList, IntoAllocatorDeferBox};
use crate::utils;
use crate::vulkan_functions::VulkanFunctionsBuilder;
use crate::Allocation;
use crate::AllocationCreateInfo;
use crate::AllocationInfo;
use crate::Stats;
use aleph_vulkan_core::erupt::utils::VulkanResult;
use aleph_vulkan_core::erupt::vk1_0::{
    Buffer, BufferCreateInfo, DeviceMemory, DeviceSize, Image, ImageCreateInfo, MemoryRequirements,
    PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, FALSE, TRUE,
};
use aleph_vulkan_core::Device;
use core::mem;
use core::ptr;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;
use vulkan_alloc_sys::raw;

///
/// A rusty wrapper around the raw VmaAllocatorCreateFlag constants
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AllocatorCreateFlag(u32);

impl AllocatorCreateFlag {
    ///
    /// VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT
    ///
    pub const EXTERNALLY_SYNCHRONIZED_BIT: AllocatorCreateFlag = AllocatorCreateFlag(
        raw::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT as u32,
    );

    ///
    /// VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT
    ///
    pub const KHR_DEDICATED_ALLOCATION_BIT: AllocatorCreateFlag = AllocatorCreateFlag(
        raw::VmaAllocatorCreateFlagBits_VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT as u32,
    );
}

impl From<u32> for AllocatorCreateFlag {
    fn from(input: u32) -> Self {
        AllocatorCreateFlag(input)
    }
}

impl Into<u32> for AllocatorCreateFlag {
    fn into(self) -> u32 {
        self.0
    }
}

///
/// Errors that can be produced by trying to construct a vma Allocator incorrectly
///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AllocatorBuilderError {
    InvalidFunctionPointers,
    InvalidExtensionFunctionPointers,
}

///
/// Builder utility for simple, safe construction of a vma Allocator
///
pub struct AllocatorBuilder {
    create_info: raw::VmaAllocatorCreateInfo,
}

impl AllocatorBuilder {
    ///
    /// New instance of the builder
    ///
    /// User must give builder an instance, device and physical device otherwise it will fail to
    /// build
    ///
    pub fn new() -> Self {
        let create_info = raw::VmaAllocatorCreateInfo {
            flags: 0,
            physicalDevice: ptr::null_mut(),
            device: ptr::null_mut(),
            preferredLargeHeapBlockSize: 0,
            pAllocationCallbacks: ptr::null_mut(),
            pDeviceMemoryCallbacks: ptr::null_mut(),
            frameInUseCount: 0,
            pHeapSizeLimit: ptr::null_mut(),
            pVulkanFunctions: ptr::null_mut(),
            pRecordSettings: ptr::null_mut(),
        };

        AllocatorBuilder { create_info }
    }

    ///
    /// Build the VmaAllocator
    ///
    /// ## Errors
    ///
    /// - If there is no valid physical device handle
    /// - If there is no valid device handle
    /// - If not all required function pointers are provided
    /// - If KHR_DEDICATED_ALLOCATION_BIT is set and the required function pointers are not given
    ///
    pub fn build(mut self, device: &Arc<Device>) -> Result<Arc<Allocator>, AllocatorBuilderError> {
        self.create_info.device = device.loader().handle.0 as *mut _;
        self.create_info.physicalDevice = device.physical_device().0 as *mut _;

        let instance_loader = device.instance().loader();
        let device_loader = device.loader();
        let functions = VulkanFunctionsBuilder::new()
            .erupt_tables(&instance_loader, &device_loader)
            .build();

        if !utils::allocator_functions_valid(&functions) {
            return Err(AllocatorBuilderError::InvalidFunctionPointers);
        }

        self.create_info.pVulkanFunctions = &functions;

        let a_val: u32 = AllocatorCreateFlag::KHR_DEDICATED_ALLOCATION_BIT.into();
        if self.create_info.flags & a_val != 0
            && (functions.vkGetBufferMemoryRequirements2KHR == None
                || functions.vkGetImageMemoryRequirements2KHR == None)
        {
            return Err(AllocatorBuilderError::InvalidExtensionFunctionPointers);
        }

        aleph_log::trace!("Creating Vulkan allocator");
        let mut raw_alloc: raw::VmaAllocator = ptr::null_mut();
        unsafe {
            let result = raw::vmaCreateAllocator(&self.create_info, &mut raw_alloc as *mut _);
            if result != raw::VkResult::VK_SUCCESS {
                panic!(
                    "Failed on call to vmaCreateAllocator with error: {:?}",
                    result
                );
            }
        }

        let alloc = Allocator {
            allocator: raw_alloc,
            device: device.clone(),
            defer_list: AllocatorDeferList::new(),
        };

        Ok(Arc::new(alloc))
    }
}

impl Default for AllocatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

///
/// Holds the internal reference to the vma allocator as well as managing correct lifetime
///
pub struct Allocator {
    allocator: raw::VmaAllocator,
    device: Arc<Device>,
    defer_list: AllocatorDeferList,
}

impl Allocator {
    ///
    /// Return a builder for the vma allocator
    ///
    pub fn builder() -> AllocatorBuilder {
        AllocatorBuilder::new()
    }

    ///
    /// Get the raw VmaAllocator handle
    ///
    pub fn as_raw(&self) -> raw::VmaAllocator {
        self.allocator
    }

    ///
    /// Appends a function to the deferred destruction list. This can be used to defer destruction
    /// of some items until the `Allocator` it self is being dropped. This can be used to guarantee
    /// that objects live as long as the allocator as an invariant when dealing with unsafe code.
    ///
    /// This should be used to uphold invariants for unsafe code and not as a general purpose tool
    /// for object destruction.
    ///
    pub fn defer_destruction<T: IntoAllocatorDeferBox>(&self, item: T) {
        self.defer_list.add(item);
    }

    ///
    /// vmaGetPhysicalDeviceProperties
    ///
    pub unsafe fn get_physical_device_properties<'alloc>(
        &'alloc self,
    ) -> &'alloc PhysicalDeviceProperties {
        let mut reference: *const PhysicalDeviceProperties = ptr::null_mut();
        let reference_ptr = &mut reference as *mut *const PhysicalDeviceProperties;
        let reference_ptr = reference_ptr as *mut *const raw::VkPhysicalDeviceProperties;

        raw::vmaGetPhysicalDeviceProperties(self.allocator, reference_ptr);

        reference
            .as_ref::<'alloc>()
            .expect("Given nullptr by vmaGetPhysicalDeviceProperties")
    }

    ///
    /// vmaGetPhysicalDeviceProperties
    ///
    pub unsafe fn get_memory_properties<'alloc>(
        &'alloc self,
    ) -> &'alloc PhysicalDeviceMemoryProperties {
        let mut reference: *const PhysicalDeviceMemoryProperties = ptr::null_mut();
        let reference_ptr = &mut reference as *mut *const PhysicalDeviceMemoryProperties;
        let reference_ptr = reference_ptr as *mut *const raw::VkPhysicalDeviceMemoryProperties;

        raw::vmaGetMemoryProperties(self.allocator, reference_ptr);

        reference
            .as_ref::<'alloc>()
            .expect("Given nullptr by vmaGetMemoryProperties")
    }

    ///
    /// vmaCalculateStats
    ///
    pub unsafe fn calculate_stats(&self) -> Stats {
        let mut stats = Stats::default();
        let stats_ptr = &mut stats as *mut Stats;
        let stats_ptr = stats_ptr as *mut raw::VmaStats;

        raw::vmaCalculateStats(self.allocator, stats_ptr);

        stats
    }

    ///
    /// Returns a rusty String object that is much easier to pass around than a CStr reference. It
    /// will also automatically de-allocate making it much safer, but will incur an extra allocation
    /// and utf-8 conversion so be aware of the overhead.
    ///
    pub unsafe fn get_stats_string(&self, detailed_map: bool) -> String {
        let c_str = self.build_stats_string(detailed_map);

        let string = c_str.to_str().expect("Invalid utf-8 chars in stats string");
        let string = string.to_string();

        self.free_stats_string(c_str);

        string
    }

    ///
    /// vmaBuildStatsString
    ///
    pub unsafe fn build_stats_string(&self, detailed_map: bool) -> &CStr {
        let mut c_str_ptr: *mut c_char = ptr::null_mut();

        raw::vmaBuildStatsString(
            self.allocator,
            &mut c_str_ptr as *mut *mut c_char,
            if detailed_map { TRUE } else { FALSE },
        );

        CStr::from_ptr(c_str_ptr)
    }

    ///
    /// vmaFreeStatsString
    ///
    pub unsafe fn free_stats_string(&self, str: &CStr) {
        let c_str_ptr = str.as_ptr();
        raw::vmaFreeStatsString(self.allocator, c_str_ptr as *mut c_char);
    }

    ///
    /// vmaFindMemoryTypeIndex
    ///
    pub unsafe fn find_memory_type_index(
        &self,
        memory_type_bits: u32,
        allocation_create_info: &AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let alloc_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaFindMemoryTypeIndex(
            self.allocator,
            memory_type_bits,
            alloc_ptr,
            &mut idx as *mut u32,
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(idx)
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaFindMemoryTypeIndexForBufferInfo
    ///
    pub unsafe fn find_memory_type_index_for_buffer_info(
        &self,
        buffer_create_info: &BufferCreateInfo,
        allocation_create_info: &AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let alloc_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let buffer_ptr = buffer_create_info as *const BufferCreateInfo;
        let buffer_ptr = buffer_ptr as *const raw::VkBufferCreateInfo;

        let result = raw::vmaFindMemoryTypeIndexForBufferInfo(
            self.allocator,
            buffer_ptr,
            alloc_ptr,
            &mut idx as *mut u32,
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(idx)
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaFindMemoryTypeIndexForImageInfo
    ///
    pub unsafe fn find_memory_type_index_for_image_info(
        &self,
        image_create_info: &ImageCreateInfo,
        allocation_create_info: &AllocationCreateInfo,
    ) -> VulkanResult<u32> {
        let mut idx = 0u32;

        let allocation_create_info = allocation_create_info.into_raw();
        let alloc_ptr = &allocation_create_info as *const raw::VmaAllocationCreateInfo;

        let image_ptr = image_create_info as *const ImageCreateInfo;
        let image_ptr = image_ptr as *const raw::VkImageCreateInfo;

        let result = raw::vmaFindMemoryTypeIndexForImageInfo(
            self.allocator,
            image_ptr,
            alloc_ptr,
            &mut idx as *mut u32,
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(idx)
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaAllocateMemory
    ///
    pub unsafe fn allocate_memory(
        &self,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
    ) -> VulkanResult<Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let requirements = memory_requirements as *const MemoryRequirements;
        let requirements = requirements as *const raw::VkMemoryRequirements;

        let result = raw::vmaAllocateMemory(
            self.allocator,
            requirements,
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(Allocation::from_raw(allocation))
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaAllocateMemoryPages
    ///
    pub unsafe fn allocate_memory_pages(
        &self,
        memory_requirements: &MemoryRequirements,
        create_info: &AllocationCreateInfo,
        allocation_count: usize,
    ) -> VulkanResult<Vec<Allocation>> {
        let mut ret: Vec<Allocation> = Vec::with_capacity(allocation_count);
        ret.resize(allocation_count, Allocation::from_raw(ptr::null_mut()));

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let requirements = memory_requirements as *const MemoryRequirements;
        let requirements = requirements as *const raw::VkMemoryRequirements;

        let result = raw::vmaAllocateMemoryPages(
            self.allocator,
            requirements,
            create_info_ptr,
            allocation_count as _,
            ret.as_mut_ptr() as *mut raw::VmaAllocation,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(ret)
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaAllocateMemoryForBuffer
    ///
    pub unsafe fn allocate_memory_for_buffer(
        &self,
        buffer: Buffer,
        create_info: &AllocationCreateInfo,
    ) -> VulkanResult<Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemoryForBuffer(
            self.allocator,
            mem::transmute(buffer),
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(Allocation::from_raw(allocation))
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaAllocateMemoryforImage
    ///
    pub unsafe fn allocate_memory_for_image(
        &self,
        image: Image,
        create_info: &AllocationCreateInfo,
    ) -> VulkanResult<Allocation> {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();

        let create_info = create_info.into_raw();
        let create_info_ptr = &create_info as *const raw::VmaAllocationCreateInfo;

        let result = raw::vmaAllocateMemoryForImage(
            self.allocator,
            mem::transmute(image),
            create_info_ptr,
            &mut allocation,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(Allocation::from_raw(allocation))
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaFreeMemory
    ///
    pub unsafe fn free_memory(&self, allocation: Allocation) {
        raw::vmaFreeMemory(self.allocator, allocation.into_raw());
    }

    ///
    /// vmaFreeMemoryPages
    ///
    pub unsafe fn free_memory_pages(&self, allocations: &[Allocation]) {
        let pointer = allocations.as_ptr();
        let pointer = pointer as *mut Allocation;
        let pointer = pointer as *mut raw::VmaAllocation;

        raw::vmaFreeMemoryPages(self.allocator, allocations.len() as _, pointer)
    }

    ///
    /// vmaResizeAllocation
    ///
    pub unsafe fn resize_allocation(
        &self,
        allocation: &Allocation,
        new_size: DeviceSize,
    ) -> VulkanResult<()> {
        let result =
            raw::vmaResizeAllocation(self.allocator, allocation.clone().into_raw(), new_size);

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaGetAllocationInfo
    ///
    pub unsafe fn get_allocation_info(&self, allocation: &Allocation) -> AllocationInfo {
        let mut info = AllocationInfo {
            memory_type: 0,
            device_memory: DeviceMemory::default(),
            offset: 0,
            size: 0,
            p_mapped_data: ptr::null_mut(),
            p_user_data: ptr::null_mut(),
        };
        let info_ptr = &mut info as *mut AllocationInfo;
        let info_ptr = info_ptr as *mut raw::VmaAllocationInfo;

        raw::vmaGetAllocationInfo(self.allocator, allocation.clone().into_raw(), info_ptr);

        info
    }

    ///
    /// vmaTouchAllocation
    ///
    pub unsafe fn touch_allocation(&self, allocation: &Allocation) -> bool {
        let result = raw::vmaTouchAllocation(self.allocator, allocation.clone().into_raw());

        result == TRUE
    }

    // TODO : vmaSetAllocationUserData (PROBABLY WONT EXPOSE THIS, NOT VERY RUSTY)

    ///
    /// vmaCreateLostAllocation
    ///
    pub unsafe fn create_lost_allocation(&self) -> Allocation {
        let mut allocation: raw::VmaAllocation = ptr::null_mut();
        raw::vmaCreateLostAllocation(self.allocator, &mut allocation as *mut raw::VmaAllocation);

        let allocation: Allocation = mem::transmute(allocation);
        allocation
    }

    ///
    /// vmaMapMemory
    ///
    pub unsafe fn map_memory(&self, allocation: &Allocation) -> VulkanResult<*mut u8> {
        let mut pointer: *mut u8 = ptr::null_mut();
        let pointer_pointer = &mut pointer as *mut *mut u8;
        let pointer_pointer = pointer_pointer as *mut *mut c_void;

        let result = raw::vmaMapMemory(
            self.allocator,
            allocation.clone().into_raw(),
            pointer_pointer,
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(pointer)
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaUnmapMemory
    ///
    pub unsafe fn unmap_memory(&self, allocation: &Allocation) {
        raw::vmaUnmapMemory(self.allocator, allocation.clone().into_raw());
    }

    ///
    /// vmaFlushAllocation
    ///
    pub unsafe fn flush_allocation(
        &self,
        allocation: &Allocation,
        offset: DeviceSize,
        size: DeviceSize,
    ) {
        raw::vmaFlushAllocation(self.allocator, allocation.clone().into_raw(), offset, size);
    }
    ///
    /// vmaInvalidateAllocation
    ///
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &Allocation,
        offset: DeviceSize,
        size: DeviceSize,
    ) {
        raw::vmaInvalidateAllocation(self.allocator, allocation.clone().into_raw(), offset, size);
    }

    ///
    /// vmaCheckCorruption
    ///
    pub unsafe fn check_corruption(&self, memory_type_bits: u32) -> VulkanResult<()> {
        let result = raw::vmaCheckCorruption(self.allocator, memory_type_bits);

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    // TODO : vmaDefragmentationBegin
    // TODO : vmaDefragmentationEnd
    // TODO : vmaDefragment (Deprecated)

    ///
    /// vmaBindBufferMemory
    ///
    pub unsafe fn bind_buffer_memory(
        &self,
        allocation: &Allocation,
        buffer: Buffer,
    ) -> VulkanResult<()> {
        let result = raw::vmaBindBufferMemory(
            self.allocator,
            allocation.clone().into_raw(),
            mem::transmute(buffer),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaBindImageMemory
    ///
    pub unsafe fn bind_image_memory(
        &self,
        allocation: &Allocation,
        image: Image,
    ) -> VulkanResult<()> {
        let result = raw::vmaBindImageMemory(
            self.allocator,
            allocation.clone().into_raw(),
            mem::transmute(image),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaCreateBuffer
    ///
    pub unsafe fn create_buffer(
        &self,
        buffer_create_info: &BufferCreateInfo,
        alloc_create_info: &AllocationCreateInfo,
    ) -> VulkanResult<(Buffer, Allocation)> {
        let b_create_info_ptr = buffer_create_info as *const BufferCreateInfo;
        let b_create_info_ptr = b_create_info_ptr as *const raw::VkBufferCreateInfo;

        let alloc_create_info = alloc_create_info.into_raw();
        let a_create_info_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let mut buffer: Buffer = Buffer::null();
        let buffer_ptr = &mut buffer as *mut Buffer;
        let buffer_ptr = buffer_ptr as *mut raw::VkBuffer;

        let mut allocation: raw::VmaAllocation = ptr::null_mut();
        let allocation_ptr = &mut allocation as *mut raw::VmaAllocation;

        let result = raw::vmaCreateBuffer(
            self.allocator,
            b_create_info_ptr,
            a_create_info_ptr,
            buffer_ptr,
            allocation_ptr,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok((buffer, Allocation::from_raw(allocation)))
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaDestroyBuffer
    ///
    pub unsafe fn destroy_buffer(&self, buffer: Buffer, alloc: Allocation) {
        raw::vmaDestroyBuffer(
            self.allocator,
            mem::transmute(buffer),
            alloc.clone().into_raw(),
        );
        drop(alloc);
    }

    ///
    /// vmaCreateImage
    ///
    pub unsafe fn create_image(
        &self,
        image_create_info: &ImageCreateInfo,
        alloc_create_info: &AllocationCreateInfo,
    ) -> VulkanResult<(Image, Allocation)> {
        let i_create_info_ptr = image_create_info as *const ImageCreateInfo;
        let i_create_info_ptr = i_create_info_ptr as *const raw::VkImageCreateInfo;

        let alloc_create_info = alloc_create_info.into_raw();
        let a_create_info_ptr = &alloc_create_info as *const raw::VmaAllocationCreateInfo;

        let mut image: Image = Image::null();
        let image_ptr = &mut image as *mut Image;
        let image_ptr = image_ptr as *mut raw::VkImage;

        let mut allocation: raw::VmaAllocation = ptr::null_mut();
        let allocation_ptr = &mut allocation as *mut raw::VmaAllocation;

        let result = raw::vmaCreateImage(
            self.allocator,
            i_create_info_ptr,
            a_create_info_ptr,
            image_ptr,
            allocation_ptr,
            ptr::null_mut(),
        );

        if result as i32 == 0 {
            VulkanResult::new_ok((image, Allocation::from_raw(allocation)))
        } else {
            VulkanResult::new_err(aleph_vulkan_core::erupt::vk1_0::Result(result as i32))
        }
    }

    ///
    /// vmaDestroyImage
    ///
    pub unsafe fn destroy_image(&self, buffer: Image, alloc: Allocation) {
        raw::vmaDestroyImage(
            self.allocator,
            mem::transmute(buffer),
            alloc.clone().into_raw(),
        );
        drop(alloc);
    }

    ///
    /// vmaSetCurrentFrameIndex
    ///
    pub unsafe fn set_current_frame_index(self, index: u32) {
        raw::vmaSetCurrentFrameIndex(self.allocator, index);
    }

    ///
    /// Get a reference to the device this allocator was created with
    ///
    pub fn device(&self) -> &Device {
        &self.device
    }
}

//
// Implementing these is safe because vma internally synchronizes access
//
unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            self.defer_list.consume(self);
            aleph_log::trace!("Destroying Vulkan allocator");
            raw::vmaDestroyAllocator(self.allocator);
        }
    }
}
