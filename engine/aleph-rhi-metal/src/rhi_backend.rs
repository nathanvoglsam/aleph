use std::sync::atomic::AtomicBool;

use aleph_any::AnyArc;
use aleph_rhi_api::{ContextCreateError, IContext};

pub static RHI_BACKEND_OBJECT: MetalLoader = MetalLoader {
    context_made: AtomicBool::new(false),
};

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct MetalConfig {}

pub struct MetalLoader {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl MetalLoader {
    pub fn is_available(&self) -> bool {
        todo!()
    }

    pub fn make_context(
        &self,
        validation: bool,
        debug: bool,
        config: &MetalConfig,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        todo!()
    }
}
