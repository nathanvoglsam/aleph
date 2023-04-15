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

use crate::raw;

///
/// Checks for the minimum required functions for vma allocator
///
#[inline]
pub fn allocator_functions_valid(funcs: &raw::VmaVulkanFunctions, requirements_2: bool) -> bool {
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
    if funcs.vkGetBufferMemoryRequirements2KHR.is_none() && requirements_2 {
        return false;
    }
    if funcs.vkGetImageMemoryRequirements2KHR.is_none() && requirements_2 {
        return false;
    }

    true
}
