//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::reflect::structure::StructResolutionError;
use crate::reflect::Struct;
use spirv_reflect::types::ReflectBlockVariable;
use std::ops::Deref;

///
/// Represents a push constant layout reflected from a shader module
///
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct PushConstantReflection {
    inner: Struct,
}

impl PushConstantReflection {
    ///
    /// Reflect a push constant layout from the given block variable
    ///
    pub fn reflect(block: ReflectBlockVariable) -> Result<Self, StructResolutionError> {
        let inner = super::structure::resolve_struct_block(block)?;
        Ok(Self { inner })
    }
}

impl Deref for PushConstantReflection {
    type Target = Struct;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
