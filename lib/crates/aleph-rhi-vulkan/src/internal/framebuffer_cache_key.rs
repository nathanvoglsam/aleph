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

use ash::vk;

/// An optimized cache key for `VkFramebuffer` objects. Skips storing any information not needed to
/// cache framebuffers under our implementation scheme. Specifically:
///
/// - width/height is constant between all attachments, as required by our RHI. The cache lives on
///   a texture object so we can infer width/height from the texture object we've cached on.
///
/// - layer count is constant between all attachments in a begin rendering call so we can hoist the
///   value to the top.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FramebufferCacheKey<'a> {
    /// The layer count as pulled from the 'begin rendering' info. This is constant across all the
    /// attachments so we raise it to a top level item to avoid hashing it multiple times for no
    /// reason.
    pub layer_count: u32,

    /// The information relevant for each attachment in the framebuffer. Some things in the vulkan
    /// struct are not needed for our purposes. We only care about format and the image view usage.
    /// width/height/layers is constant across all attachments.
    pub attachments: &'a [FramebufferCacheKeyItem],
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FramebufferCacheKeyItem {
    /// The format the image in framebuffer slot `n` is being viewed as. This should come from the
    /// image view.
    pub format: vk::Format,

    /// The image flags that the image in framebuffer slot `n` was created with. This comes from the
    /// underlying image, but is cached on the image view.
    pub creation_flags: vk::ImageCreateFlags,

    /// The usage flags the image in framebuffer slot `n` is being viewed as. This should come from
    /// the image view.
    pub usage: vk::ImageUsageFlags,
}
