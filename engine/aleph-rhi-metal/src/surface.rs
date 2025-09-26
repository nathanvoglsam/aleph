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

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use objc2::rc::{Retained, autoreleasepool};
use objc2_core_foundation::CGSize;
use objc2_quartz_core::CAMetalLayer;
use parking_lot::Mutex;

use crate::context::Context;
use crate::internal::{conv, unwrap};
use crate::swap_chain::{SwapChain, SwapChainObjects, SwapChainState};

pub struct Surface {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) objects: SurfaceObjects,
}

declare_interfaces!(Surface, [ISurface]);

impl IGetPlatformInterface for Surface {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl ISurface for Surface {
    fn upgrade(&self) -> AnyArc<dyn ISurface> {
        AnyArc::map::<dyn ISurface, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn create_swap_chain(
        &self,
        device: &dyn IDevice,
        config: &SwapChainConfiguration,
    ) -> Result<AnyArc<dyn ISwapChain>, SwapChainCreateError> {
        autoreleasepool(|_| {
            let device = unwrap::device(device);

            unsafe {
                let size = self.objects.layer.drawableSize();
                let width = size.width as u32;
                let height = size.height as u32;

                if width != config.width || height != config.height {
                    log::debug!(
                        "CAMetalLayer size was ({}, {}). Setting to ({}, {})",
                        width,
                        height,
                        config.width,
                        config.height
                    );

                    self.objects.layer.setDrawableSize(CGSize {
                        width: config.width as f64,
                        height: config.height as f64,
                    });
                } else {
                    log::debug!("CAMetalLayer size is already ({}, {})", width, height);
                }

                let format = conv::pixel_mtl_to_format(self.objects.layer.pixelFormat());
                if format != config.format {
                    log::debug!(
                        "CAMetalLayer format was {}. Setting to {}",
                        format,
                        config.format
                    );

                    let mtl_format = conv::format_to_pixel_mtl(config.format);
                    self.objects.layer.setPixelFormat(mtl_format);
                } else {
                    log::debug!(
                        "CAMetalLayer format is already {}. Doing nothing",
                        config.format
                    );
                }

                let image_count = self.objects.layer.maximumDrawableCount();
                if image_count != config.buffer_count as usize {
                    log::debug!(
                        "CAMetalLayer maximumDrawableCount was '{}'. Setting to '{}'",
                        image_count,
                        config.buffer_count
                    );

                    self.objects
                        .layer
                        .setMaximumDrawableCount(config.buffer_count as usize);
                } else {
                    log::debug!(
                        "CAMetalLayer maximumDrawableCount is already '{}'",
                        image_count
                    );
                }
            }

            let swap_chain = AnyArc::new_cyclic(move |v| SwapChain {
                this: v.clone(),
                device: device.this.upgrade().unwrap(),
                _surface: self.this.upgrade().unwrap(),
                queue_type: config.present_queue,
                objects: SwapChainObjects {
                    layer: self.objects.layer.clone(),
                },
                inner: Mutex::new(SwapChainState {
                    config: config.clone(),
                }),
            });
            Ok(AnyArc::map::<dyn ISwapChain, _>(swap_chain, |v| v))
        })
    }
}

/// Wrapper struct to limit the scope of our 'unsafe impl Send+Sync'
pub struct SurfaceObjects {
    pub layer: Retained<CAMetalLayer>,
}

// SAFETY: Needed for CAMetalLayer
unsafe impl Send for SurfaceObjects {}
unsafe impl Sync for SurfaceObjects {}
