use std::sync::atomic::{AtomicBool, Ordering};

use aleph_any::AnyArc;
use aleph_rhi_api::{ContextCreateError, IContext};
use parking_lot::Mutex;
use windows::Win32::Graphics::Dxgi::*;

use crate::context::Context;
use crate::internal::debug_interface::DebugInterface;
use crate::internal::dxgi_debug_interface::dxgi_get_debug_interface;
use crate::internal::{DEVICE_CREATE_FN, DXGI_CREATE_FN, create_dxgi_factory};

pub static RHI_BACKEND_OBJECT: D3D12Loader = D3D12Loader {
    context_made: AtomicBool::new(false),
};

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct D3D12Config {}

pub struct D3D12Loader {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl D3D12Loader {
    pub fn is_available(&self) -> bool {
        unsafe { DXGI_CREATE_FN.get().is_ok() && DEVICE_CREATE_FN.get().is_ok() }
    }

    pub fn make_context(
        &self,
        validation: bool,
        debug: bool,
        _config: &D3D12Config,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                let dxgi_factory = create_dxgi_factory(validation)
                    .map_err(|e| log::error!("Platform Error: {:#?}", e))?;

                let gpu_assisted = true;
                let debug_interface = unsafe { setup_debug_layer(validation, gpu_assisted) };
                let dxgi_debug = unsafe { setup_dxgi_debug_interface(debug) };

                let context = AnyArc::new_cyclic(move |v| Context {
                    this: v.clone(),
                    debug: debug_interface,
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
    unsafe {
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
}

unsafe fn setup_dxgi_debug_interface(debug: bool) -> Option<IDXGIDebug> {
    unsafe {
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
}
