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

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use rhi::TextureDimension;

use crate::async_resource_loader::texture_upload_range::WantedTextureRows;
use crate::internal::async_resource_loader::MgAsyncLdrSystem;
use crate::internal::async_resource_loader::upload_memory_manager::UploadMemoryManager;
use crate::resource::texture::physical::{PhysicalTextureDesc, PhysicalTextureLayout};
use crate::resource::texture::simple::{SimpleTextureDesc, SimpleTextureLayout};
use crate::resource::texture::single::SingleTextureDesc;

pub struct TextureLoadState<C> {
    /// The texture object we're uploading into on the GPU.
    pub texture: rhi::TextureHandle,

    /// The layout the texture was created with
    pub layout: SimpleTextureLayout,

    /// Tracks the state associated with loading each mip level the caller should provide for the
    /// texture.
    pub levels: BVec<TextureLevelLoadState, MgAsyncLdrSystem>,

    /// Caller provided cookie that will be sent alongside messages on the loader's notification
    /// channel to identify the associated request.
    pub cookie: C,
}

impl<C> TextureLoadState<C> {
    /// Returns true if there is already an outstanding range for this texture load.
    pub fn has_outstanding_range(&self) -> bool {
        self.levels
            .iter()
            .any(TextureLevelLoadState::has_outstanding_range)
    }

    /// Returns if every needed row has been submitted to the loader. This implies that the load
    /// is complete, and once all uploads are retired that the texture can be messaged to the
    /// renderer as ready for use.
    pub fn is_complete(&self) -> bool {
        self.levels.iter().all(TextureLevelLoadState::is_complete)
    }

    /// Returns true if no blocks have been allocated for the texture yet. This is used by the
    /// loader to know when a submitted block is the first. The loader must perform an initial
    /// discard to prepare the texture to be copied into, this check is used to know when we need
    /// to.
    pub fn needs_discard(&self) -> bool {
        self.levels.iter().all(|v| v.rows_allocated == 0)
    }

    /// Attempt to generate as many 'wanted texture rows' blocks as we can roughly fit within
    /// 'num_bytes'. We make no guarantee of exactly how much memory will end up being allocated.
    /// We may consume more or less data than 'num_bytes', but the real amount should be as close
    /// as we can get and still generate valid upload commands.
    pub fn build_wanted_rows(
        &mut self,
        upload_memory_manager: &UploadMemoryManager,
        pitch_align: u32,
        mut num_bytes: u64,
        granularity: rhi::Extent3D,
    ) -> BVec<WantedTextureRows, MgAsyncLdrSystem> {
        assert!(!self.has_outstanding_range());

        let texture_dimension = self.layout.texture_dimension().unwrap();

        let mut wanted_rows = BVec::new_in(system());
        for (mip_i, level_state) in self.levels.iter_mut().enumerate().rev() {
            // If we've hit the requested soft cap on the number of bytes to upload then we bail.
            if num_bytes == 0 {
                break;
            }

            // Skip trying to allocate data for a level we've already fully uploaded data for.
            if level_state.is_complete() {
                continue;
            }

            // Query the storage size of the texture at the current mip level.
            let level_layout = self.layout.as_level(mip_i as u32);
            let level_physical_layout = level_layout.with_aligned_pitch(pitch_align);

            // Remap the input 'min_image_granularity' to the more restrictive form we require for
            // our upload interface. How we handle our remapping depends on whether we're making a
            // 1D, 2D or 3D texture.
            //
            // # 1D Textures
            // Always upload the whole level
            //
            // # 2D Textures
            // Always upload whole rows, with a minimum number of rows defined by the granularity
            // height.
            //
            // # 3D Textures
            // Always upload whole slices, with the minimum number of slices defined by the
            // granularity depth.
            //
            // # (0,0,0)
            // A special value that a queue may provide if it only supports whole mip copies.
            let granularity = if granularity == rhi::Extent3D::default() {
                // Special value, can only copy whole mips on this queue. Remapped granularity
                // becomes the storage dimensions of this mip level.
                level_layout.storage_extent()
            } else {
                match texture_dimension {
                    // Requires whole mip, so remapped granularity is just the storage size of this
                    // level.
                    TextureDimension::Texture1D => level_layout.storage_extent(),

                    // 2D textures can ignore depth. We always upload whole rows. We simply
                    // remap the granularity to have the storage width of the level but inherit the
                    // queue's height granularity.
                    TextureDimension::Texture2D => {
                        rhi::Extent3D::new(level_layout.width, granularity.height, 1)
                    }

                    // 3D textures are a bit painful. Because we must upload contiguous blocks of
                    // rows we have to inflate our granularity to upload whole 2D slices of the 3D
                    // texture. We remap the input granularity to be the 2D storage dimensions of
                    // the mip level, but inherit the depth granularity of the queue.
                    TextureDimension::Texture3D => rhi::Extent3D::new(
                        level_layout.width,
                        level_layout.height,
                        granularity.depth,
                    ),
                }
            };

            // In all cases this becomes the correct number of rows based on our remapped
            // granularity to handle 1D, 2D or 3D textures. The total number of rows we upload must
            // be a multiple of this number, clamped to the end of the image if the dimension is not
            // an even multiple.
            let row_granularity = granularity.height as u64 * granularity.depth as u64;
            let block_size = row_granularity * level_physical_layout.upload_row_bytes() as u64;

            // First we calculate an integer number of blocks from 'num_bytes', rounding up
            // and potentially over-allocating to ensure we try and allocate a multiple of
            // 'row_granularity' rows. This will be clamped against the number of rows left to
            // provide for the current mip. This will ensure our copy will still respect the
            // copy granularity.
            let wanted_num_blocks = num_bytes.div_ceil(block_size);

            // Deduce 'num_rows', clamped against the number of rows left to be provided for the
            // current level.
            //
            // This should always fit within a u32 because 'max_rows' is our upper limit, and that
            // can't be bigger than a u32 by construction.
            let max_rows = level_state.rows_needed - level_state.rows_submitted;
            let num_rows = wanted_num_blocks * row_granularity;
            let num_rows = u64::min(max_rows as u64, num_rows) as u32;

            // Allocate a sufficiently aligned block of memory that is valid to use for
            // buffer->image copies.
            let alloc_bytes = num_rows as u64 * level_physical_layout.upload_row_bytes() as u64;
            let alloc_bytes_u32 = u32::try_from(alloc_bytes);
            let alloc_bytes_u32 = match alloc_bytes_u32 {
                Ok(v) => v,
                // We can't 'throw' and error with ? here because we may have already allocated some
                // levels. Instead, we bail as if we ran out of 'num_bytes' and return whatever is
                // in the wanted_rows list.
                Err(_) => break,
            };
            let ((allocation, aligned_offset), data) =
                match upload_memory_manager.allocate_upload_range_aligned(alloc_bytes_u32, 512) {
                    Ok(v) => v,
                    // Same story as above. A failure here isn't a failure for the whole function.
                    Err(_) => break,
                };

            // We've now confirmed we have an upload block big enough. We update the level state to
            // reflect the number of rows we just allocated.
            level_state.rows_allocated += num_rows;
            debug_assert!(level_state.rows_allocated > level_state.rows_submitted);

            // Consume 'alloc_bytes' from the soft upper limit of bytes we're supposed to
            // allocate. We are allowed to over run the limit. Saturating sub means we clamp to
            // zero if we consume more bytes than was strictly asked for.
            num_bytes = num_bytes.saturating_sub(alloc_bytes);

            // This defines the dst origin for the texture copy used to copy into the target mip
            // level.
            //
            // The origin is deduced from the number of rows we know to have been submitted for the
            // level so far. 'x' is always zero because we always upload entire rows.
            let origin_y = level_state.rows_submitted % level_layout.height;
            let origin_z = level_state.rows_submitted / level_layout.height;
            let origin = rhi::UOffset3D::new(0, origin_y, origin_z);

            // This calculates the copy extent by calculating the extent as 'end' - 'origin'.
            let end_y = level_state.rows_allocated % level_layout.height;
            let end_z = level_state.rows_allocated / level_layout.height;
            let extent_x = level_layout.width;
            let extent_y = end_y - origin_y;
            let extent_z = end_z - origin_z;

            wanted_rows.push(WantedTextureRows {
                allocation,
                level: mip_i as u32,
                wanted_rows: level_state.rows_submitted..level_state.rows_allocated,
                buffer_offset: aligned_offset,
                region_offset: origin,
                region_size: PhysicalTextureLayout {
                    width: extent_x,
                    height: extent_y,
                    depth: extent_z,
                    row_pitch: level_physical_layout.upload_row_texels(),
                    format: self.layout.format,
                },
                data,
            });
        }

        wanted_rows
    }
}

pub struct TextureLevelLoadState {
    /// Number of rows allocated from the loader's pool associated with this texture load.
    pub rows_allocated: u32,

    /// Number of rows submitted to the upload queue associated with this texture load.
    pub rows_submitted: u32,

    /// Total number of rows needed for the resource to be considered completely uploaded.
    pub rows_needed: u32,
}

impl TextureLevelLoadState {
    pub const fn has_outstanding_range(&self) -> bool {
        self.rows_allocated > self.rows_submitted
    }

    pub fn is_complete(&self) -> bool {
        self.rows_submitted == self.rows_needed
    }
}
