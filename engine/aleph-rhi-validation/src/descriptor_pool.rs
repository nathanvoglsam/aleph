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

use std::any::TypeId;
use std::mem::{MaybeUninit, swap};
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_pool::{IBlockFactory, ParameterBlockPool};

use crate::internal::parameter_block::ParameterBlock;
use crate::{ValidationDevice, ValidationParameterBlockLayout};

pub struct ValidationDescriptorPool {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _layout: AnyArc<ValidationParameterBlockLayout>,
    pub(crate) pool: ParameterBlockPool<PoolBlockFactory>,
}

declare_interfaces!(ValidationDescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for ValidationDescriptorPool {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        let inner = self.pool.factory.borrow();
        unsafe { inner.inner_pool.__query_platform_interface(target, out) }
    }
}

impl IDescriptorPool for ValidationDescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool.allocate_blocks(&self._layout, &mut blocks)?;

        unsafe {
            let block = blocks[0].assume_init();
            Ok(block)
        }
    }

    fn allocate_blocks(
        &mut self,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let mut blocks = Box::new_uninit_slice(num_blocks);
        self.pool.allocate_blocks(&self._layout, &mut blocks)?;

        let blocks = Box::leak(blocks);
        let blocks = NonNull::from(blocks);
        let blocks =
            NonNull::slice_from_raw_parts(blocks.cast::<ParameterBlockHandle>(), blocks.len());
        unsafe { Ok(Box::from_raw(blocks.as_ptr())) }
    }

    unsafe fn free(&mut self, blocks: &[ParameterBlockHandle]) {
        self.pool.free_blocks(blocks)
    }

    unsafe fn reset(&mut self) {
        unsafe {
            self.pool.reset_pool();
        }
    }
}

pub struct PoolBlockFactory {
    pub pool_id: u64,
    pub inner_pool: Box<dyn IDescriptorPool>,
}

unsafe impl IBlockFactory for PoolBlockFactory {
    type Param<'a> = &'a ValidationParameterBlockLayout;
    type T = ParameterBlock;

    fn init_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: &mut [MaybeUninit<Self::T>],
    ) -> Result<(), DescriptorAllocateError> {
        for block in blocks {
            let new = ParameterBlock {
                _magic_header: ParameterBlock::MAGIC_HEADER_VAL,
                _pool_id: self.pool_id,
                _layout: p.this.upgrade().unwrap(),
                inner: Some(self.inner_pool.allocate_block()?),
            };
            block.write(new);
        }
        Ok(())
    }

    fn reuse_blocks<'a>(
        &mut self,
        p: Self::Param<'a>,
        blocks: impl Iterator<Item = NonNull<Self::T>>,
    ) -> Result<(), DescriptorAllocateError> {
        for mut block in blocks {
            let mut inner_block = Some(self.inner_pool.allocate_block()?);
            unsafe {
                let block = block.as_mut();
                block._layout = p.this.upgrade().unwrap();
                swap(&mut inner_block, &mut block.inner);
            }
            assert!(inner_block.is_none());
        }
        Ok(())
    }

    fn free_blocks(&mut self, blocks: impl Iterator<Item = NonNull<Self::T>>) {
        for mut block in blocks {
            unsafe {
                assert!(block.is_aligned());

                let block = block.as_mut();
                block.validate(Some(self.pool_id));

                self.inner_pool.free(&[block.inner.take().unwrap()]);
            }
        }
    }

    fn reset_blocks(&mut self, blocks: &mut [Self::T]) {
        for block in blocks {
            let _ = block.inner.take().unwrap();
        }
        unsafe {
            self.inner_pool.reset();
        }
    }
}
