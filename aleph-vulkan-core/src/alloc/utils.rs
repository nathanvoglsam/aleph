//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vma_sys::raw;

///
/// Checks for the minimum required functions for vma allocator
///
pub fn allocator_functions_valid(funcs: &raw::VmaVulkanFunctions) -> bool {
    if funcs.vkGetPhysicalDeviceProperties.is_none() {
        return false;
    }
    if funcs.vkGetPhysicalDeviceMemoryProperties.is_none() {
        return false;
    }
    if funcs.vkAllocateMemory.is_none() {
        return false;
    }
    if funcs.vkFreeMemory.is_none() {
        return false;
    }
    if funcs.vkMapMemory.is_none() {
        return false;
    }
    if funcs.vkUnmapMemory.is_none() {
        return false;
    }
    if funcs.vkFlushMappedMemoryRanges.is_none() {
        return false;
    }
    if funcs.vkInvalidateMappedMemoryRanges.is_none() {
        return false;
    }
    if funcs.vkBindBufferMemory.is_none() {
        return false;
    }
    if funcs.vkBindImageMemory.is_none() {
        return false;
    }
    if funcs.vkGetBufferMemoryRequirements.is_none() {
        return false;
    }
    if funcs.vkGetImageMemoryRequirements.is_none() {
        return false;
    }
    if funcs.vkCreateBuffer.is_none() {
        return false;
    }
    if funcs.vkDestroyBuffer.is_none() {
        return false;
    }
    if funcs.vkCreateImage.is_none() {
        return false;
    }
    if funcs.vkDestroyImage.is_none() {
        return false;
    }
    if funcs.vkCmdCopyBuffer.is_none() {
        return false;
    }
    //if funcs.vkGetBufferMemoryRequirements2KHR.is_none() { return false; }
    //if funcs.vkGetImageMemoryRequirements2KHR.is_none() { return false; }

    true
}
