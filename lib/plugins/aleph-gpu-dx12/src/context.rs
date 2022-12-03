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

use crate::adapter::Adapter;
use crate::internal::adapter_description_decoder::adapter_description_string;
use crate::internal::create_device::create_device;
use crate::internal::debug_interface::DebugInterface;
use crate::internal::feature_support::FeatureSupport;
use crate::internal::swap_chain_creation::dxgi_create_swap_chain;
use crate::surface::Surface;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak, QueryInterface};
use interfaces::gpu::{
    AdapterPowerClass, AdapterRequestOptions, AdapterVendor, BackendAPI, IAdapter, IContext,
    ISurface, SurfaceCreateError,
};
use interfaces::platform::HasRawWindowHandle;
use std::sync::atomic::AtomicBool;
use windows::core::Interface;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct Context {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) debug: Option<DebugInterface>,
    pub(crate) dxgi_debug: Option<IDXGIDebug>,
    pub(crate) factory: IDXGIFactory2,
}

declare_interfaces!(Context, [IContext, IContextExt]);

impl Context {
    /// Checks if a surface is compatible with an adapter by performing a full device initialization
    /// in order to check if it would succeed.
    ///
    /// There's no other way to check if the surface can be used on the device so we just eat some
    /// overhead at init time to do this.
    fn check_surface_compatibility(&self, device: &ID3D12Device, surface: &Surface) -> Option<()> {
        // Create a direct queue so we can create a swapchain
        let desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: 0,
            Flags: Default::default(),
            NodeMask: 0,
        };
        let queue = unsafe {
            device
                .CreateCommandQueue::<ID3D12CommandQueue>(&desc)
                .ok()?
        };

        // Create our swap chain to check if the surface is compatible
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: 128,
            Height: 128,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: BOOL::from(false),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 3,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            AlphaMode: DXGI_ALPHA_MODE_IGNORE,
            Flags: 0,
        };
        unsafe {
            dxgi_create_swap_chain(&self.factory, &queue, &surface, &desc).ok()?;
        }

        Some(())
    }

    /// Checks if the adapter supports all the minimum required features. This requires a full
    /// device initialization because of D3D12's API.
    fn check_mandatory_features(&self, device: ID3D12Device) -> Option<()> {
        let feature_support = FeatureSupport::new(device).ok()?;

        if feature_support.MaxSupportedFeatureLevel().0 < D3D_FEATURE_LEVEL_12_0.0 {
            return None;
        }

        if !feature_support.EnhancedBarriersSupported() {
            return None;
        }

        if feature_support.HighestShaderModel().0 < D3D_SHADER_MODEL_6_0.0 {
            return None;
        }

        if feature_support.HighestRootSignatureVersion().0 < D3D_ROOT_SIGNATURE_VERSION_1_1.0 {
            return None;
        }

        Some(())
    }

    fn adapter_meets_requirements(
        &self,
        options: &AdapterRequestOptions,
        candidate: &IDXGIAdapter1,
    ) -> bool {
        let device = create_device(candidate, D3D_FEATURE_LEVEL_11_0).ok();
        let device = if let Some(device) = device {
            device
        } else {
            return false;
        };

        if let Some(surface) = options.surface {
            let surface = surface.query_interface::<Surface>().unwrap();
            if self
                .check_surface_compatibility((&device).into(), surface)
                .is_none()
            {
                return false;
            }
        }

        if self.check_mandatory_features(device.into()).is_none() {
            return false;
        }

        true
    }

    fn select_hardware_adapter(
        &self,
        gpu_preference: DXGI_GPU_PREFERENCE,
        mut filter: impl FnMut(&IDXGIAdapter1) -> bool,
    ) -> Option<IDXGIAdapter1> {
        unsafe {
            // If possible we can explicitly ask for a "high performance" device.
            let factory_2: &IDXGIFactory2 = &self.factory;
            let factory_6: Option<IDXGIFactory6> = self.factory.cast::<IDXGIFactory6>().ok();

            // Loop over all the available adapters
            let mut i = 0;
            loop {
                // Use the newest available interface to enumerate the adapter
                let adapter = if let Some(factory_6) = factory_6.as_ref() {
                    factory_6.EnumAdapterByGpuPreference::<IDXGIAdapter1>(i, gpu_preference)
                } else {
                    factory_2.EnumAdapters1(i)
                };

                // Check if we've gotten an adapter, or break from the loop if we don't as we've either hit
                // a big error or enumerated all of them already
                if let Ok(adapter) = adapter {
                    // Get the adapter description so we can decide if we want to use it or not
                    let desc = adapter.GetDesc1().unwrap();

                    // We want to skip software adapters as they're going to be *very* slow
                    if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0) != 0 {
                        i += 1;
                        continue;
                    }

                    if filter(&adapter) {
                        return Some(adapter);
                    }
                } else {
                    break;
                }

                i += 1;
            }

            None
        }
    }
}

impl IContext for Context {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        let power_preference = match options.power_class {
            AdapterPowerClass::LowPower => DXGI_GPU_PREFERENCE_MINIMUM_POWER,
            AdapterPowerClass::HighPower => DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
        };
        if let Some(adapter) = self.select_hardware_adapter(power_preference, |candidate| {
            self.adapter_meets_requirements(options, candidate)
        }) {
            let device = create_device(&adapter, D3D_FEATURE_LEVEL_11_0).ok()?;

            if let Some(surface) = options.surface {
                let surface = surface.query_interface::<Surface>().unwrap();
                self.check_surface_compatibility(&device.into(), surface)?;
            }

            let desc = unsafe {
                adapter
                    .GetDesc1()
                    .expect("Failed to get adapter description. Something very wrong")
            };
            let name = adapter_description_string(&desc).unwrap_or_else(|| "Unknown".to_string());
            let vendor = match desc.VendorId {
                0x1002 => AdapterVendor::AMD,
                0x1010 => AdapterVendor::ImaginationTechnology,
                0x10DE => AdapterVendor::NVIDIA,
                0x13B5 => AdapterVendor::ARM,
                0x5143 => AdapterVendor::Qualcomm,
                0x8086 => AdapterVendor::Intel,
                _ => AdapterVendor::Unknown,
            };

            let adapter = AnyArc::new_cyclic(move |v| Adapter {
                this: v.clone(),
                context: self.this.upgrade().unwrap(),
                name,
                vendor,
                adapter,
            });
            Some(AnyArc::map::<dyn IAdapter, _>(adapter, |v| v))
        } else {
            None
        }
    }

    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let surface = AnyArc::new_cyclic(move |v| Surface {
            this: v.clone(),
            _context: self.this.upgrade().unwrap(),
            factory: self.factory.clone(),
            handle: window.raw_window_handle(),
            has_swap_chain: AtomicBool::new(false),
        });
        Ok(AnyArc::map::<dyn ISurface, _>(surface, |v| v))
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if let Some(dxgi_debug) = &self.dxgi_debug {
            unsafe {
                dxgi_debug
                    .ReportLiveObjects(DXGI_DEBUG_ALL, DXGI_DEBUG_RLO_ALL)
                    .unwrap();
            }
        }
    }
}

// SAFETY: Can't be auto marked because of the COM pointers. COM pointers are just Arc, which is
// fine to send across thread boundaries
unsafe impl Send for Context {}

pub trait IContextExt: IContext {
    fn get_raw_handle(&self) -> &IDXGIFactory2;

    fn get_dxgi_debug(&self) -> Option<&IDXGIDebug>;
}

impl IContextExt for Context {
    fn get_raw_handle(&self) -> &IDXGIFactory2 {
        &self.factory
    }

    fn get_dxgi_debug(&self) -> Option<&IDXGIDebug> {
        self.dxgi_debug.as_ref()
    }
}
