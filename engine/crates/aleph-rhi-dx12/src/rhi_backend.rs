use std::sync::atomic::{AtomicBool, Ordering};

use aleph_any::AnyArc;
use aleph_rhi_api::{BackendAPI, IContext};
use aleph_rhi_loader_api::{ContextCreateError, ContextOptions, IRhiBackend};
use parking_lot::Mutex;
use windows::Win32::Graphics::Dxgi::*;

use crate::context::Context;
use crate::internal::create_dxgi_factory::create_dxgi_factory;
use crate::internal::debug_interface::DebugInterface;
use crate::internal::dxgi_debug_interface::dxgi_get_debug_interface;
use crate::internal::{create_device, create_dxgi_factory};

pub static RHI_BACKEND: &'static dyn IRhiBackend = &RHI_BACKEND_OBJECT;

static RHI_BACKEND_OBJECT: RhiBackend = RhiBackend {
    context_made: AtomicBool::new(false),
};

struct RhiBackend {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl IRhiBackend for RhiBackend {
    fn backend(&self) -> BackendAPI {
        BackendAPI::D3D12
    }

    fn is_available(&self) -> bool {
        unsafe {
            create_dxgi_factory::CREATE_FN.get().is_ok() && create_device::CREATE_FN.get().is_ok()
        }
    }

    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                let dxgi_factory = create_dxgi_factory(options.validation)
                    .map_err(|e| log::error!("Platform Error: {:#?}", e))?;

                let gpu_assisted = !cfg!(target_vendor = "uwp");
                let debug = unsafe { setup_debug_layer(options.validation, gpu_assisted) };
                let dxgi_debug = unsafe { setup_dxgi_debug_interface(options.debug) };

                let context = AnyArc::new_cyclic(move |v| Context {
                    this: v.clone(),
                    debug,
                    dxgi_debug: dxgi_debug.map(Mutex::new),
                    factory: Some(Mutex::new(dxgi_factory)),
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

unsafe fn setup_debug_layer(want_debug: bool, gpu_assisted: bool) -> Option<DebugInterface> {
    if want_debug {
        log::debug!("D3D12 debug layers requested");
        if let Ok(debug) = DebugInterface::new() {
            debug.enable_debug_layer();
            log::debug!("D3D12 debug layers enabled");
            if gpu_assisted {
                log::debug!("D3D12 gpu validation requested");
                if debug.set_enable_gpu_validation(true).is_ok() {
                    log::debug!("D3D12 gpu validation enabled");
                } else {
                    log::debug!("D3D12 gpu validation not enabled");
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
        log::debug!("DXGI debug interface requested");
        match dxgi_get_debug_interface(true) {
            Ok(v) => {
                log::debug!("DXGI debug interface loaded");
                Some(v)
            }
            Err(e) => {
                log::debug!("DXGI debug interface not loaded: {:#?}", e);
                None
            }
        }
    } else {
        None
    }
}
