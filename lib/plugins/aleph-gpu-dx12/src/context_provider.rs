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

use crate::context::Context;
use crate::internal::create_dxgi_factory::create_dxgi_factory;
use crate::internal::debug_interface::DebugInterface;
use crate::internal::dxgi_debug_interface::dxgi_get_debug_interface;
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::anyhow::anyhow;
use interfaces::gpu::{ContextCreateError, ContextOptions, IContext, IContextProvider};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Graphics::Dxgi::*;

pub struct ContextProvider {
    /// Flags whether a context has already been created
    context_made: AtomicBool,

    /// Hack to ensure ContextProvider is not Send/Sync
    no_send_sync: PhantomData<*const ()>,
}

impl ContextProvider {
    pub fn new() -> Self {
        Self {
            context_made: AtomicBool::new(false),
            no_send_sync: Default::default(),
        }
    }
}

impl IContextProvider for ContextProvider {
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                let dxgi_factory =
                    create_dxgi_factory(options.validation).map_err(|e| anyhow!(e))?;

                let is_uwp = cfg!(target_vendor = "uwp");
                let gpu_assisted = !is_uwp;
                let debug = unsafe { setup_debug_layer(options.validation, gpu_assisted) };
                let dxgi_debug = unsafe { setup_dxgi_debug_interface(options.debug) };

                let context = AnyArc::new_cyclic(move |v| Context {
                    this: v.clone(),
                    debug,
                    dxgi_debug,
                    factory: Some(dxgi_factory),
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

declare_interfaces!(ContextProvider, [IContextProvider]);

unsafe fn setup_debug_layer(want_debug: bool, gpu_assisted: bool) -> Option<DebugInterface> {
    if want_debug {
        log::trace!("D3D12 debug layers requested");
        if let Ok(debug) = DebugInterface::new() {
            debug.enable_debug_layer();
            log::trace!("D3D12 debug layers enabled");
            if gpu_assisted {
                log::trace!("D3D12 gpu validation requested");
                if debug.set_enable_gpu_validation(true).is_ok() {
                    log::trace!("D3D12 gpu validation enabled");
                } else {
                    log::trace!("D3D12 gpu validation not enabled");
                }
            }
            Some(debug)
        } else {
            None
        }
    } else {
        None
    }
}

unsafe fn setup_dxgi_debug_interface(debug: bool) -> Option<IDXGIDebug> {
    if debug {
        log::trace!("DXGI debug interface requested");
        match dxgi_get_debug_interface(true) {
            Ok(v) => {
                log::trace!("DXGI debug interface loaded");
                Some(v)
            }
            Err(e) => {
                log::trace!("DXGI debug interface not loaded: {:#?}", e);
                None
            }
        }
    } else {
        None
    }
}
