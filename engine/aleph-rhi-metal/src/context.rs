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
use std::ffi::c_void;
use std::ptr::NonNull;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;

use objc2_metal::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::MetalConfig;
use crate::adapter::Adapter;

pub struct Context {
    pub _this: AnyWeak<Self>,
    pub _config: MetalConfig,
    pub validation: bool,
    pub debug: bool,
}

declare_interfaces!(Context, [IContext]);

impl IGetPlatformInterface for Context {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: expose the instance loader via an arc or something
        None
    }
}

impl IContext for Context {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        // Metal doesn't have software adapaters so the solution here is obvious. We bail.
        if options.deny_hardware_adapters {
            return None;
        }

        let devices = MTLCopyAllDevices();

        // No devices? No adapter...
        if devices.is_empty() {
            return None;
        }

        let mut scores: Vec<_> = (0..devices.len()).map(|i| (i, 0isize)).collect();
        for (device, (_, score)) in devices.iter().zip(scores.iter_mut()) {
            let name = device.name().to_string();
            let architecture = unsafe { device.architecture().name().to_string() };
            log::info!("=====================");
            log::info!("Considering Device: ");
            log::info!("Architecture   : {architecture}");
            log::info!("Name           : {name}");
            match options.power_class {
                AdapterPowerClass::LowPower => {
                    if device.isLowPower() {
                        *score += 10_000
                    }
                }
                AdapterPowerClass::HighPower => {
                    if !device.isLowPower() {
                        *score += 10_000
                    }
                }
            }

            // Check for minimum feature support
            let common_1 = device.supportsFamily(MTLGPUFamily::Common1);
            let common_2 = device.supportsFamily(MTLGPUFamily::Common2);
            let common_3 = device.supportsFamily(MTLGPUFamily::Common3);
            let metal_3 = device.supportsFamily(MTLGPUFamily::Metal3);
            let all = common_1 && common_2 && common_3 && metal_3;

            // We don't want this device if it doesn't support the needed features
            if !all {
                *score = -1_000_000; 
            }
        }

        scores.sort_unstable_by_key(|v| v.1);

        let device_index = scores[0].0;
        let device = devices.objectAtIndex(device_index);

        let name = device.name().to_string();
        let adapter = AnyArc::new_cyclic(move |v| Adapter {
            this: v.clone(),
            context: self._this.upgrade().unwrap(),
            name,
            vendor: AdapterVendor::Apple, // TODO: this may not always be the case
            device,
        });
        Some(AnyArc::map::<dyn IAdapter, _>(adapter, |v| v))
    }

    fn create_surface(
        &self,
        _display: &dyn HasDisplayHandle,
        _window: &dyn HasWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        unimplemented!("Use IContext::create_surface_for_metal_layer")
    }

    fn create_surface_for_metal_layer(
        &self,
        layer: NonNull<c_void>,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        todo!()
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Metal
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        todo!()
    }
}
