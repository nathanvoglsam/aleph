//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{RenderGraphPass, ResourceAccess};
use aleph_vulkan_core::erupt::vk1_0::CommandBuffer;

///
/// A simple pass that wraps some closures in an object that implements the `RenderGraphPass` trait
///
pub struct ClosurePass<RegAcc: FnMut(&mut ResourceAccess), Rec: FnMut(CommandBuffer)> {
    register_accesses: RegAcc,
    record: Rec,
}

impl<RegAcc: FnMut(&mut ResourceAccess), Rec: FnMut(CommandBuffer)> ClosurePass<RegAcc, Rec> {
    ///
    /// Builds a new ClosurePass from the provided closure objects
    ///
    pub fn new(register_accesses: RegAcc, record: Rec) -> Self {
        Self {
            register_accesses,
            record,
        }
    }
}

impl<RegAcc, Rec> RenderGraphPass for ClosurePass<RegAcc, Rec>
where
    RegAcc: FnMut(&mut ResourceAccess),
    Rec: FnMut(CommandBuffer),
{
    fn register_access(&mut self, accesses: &mut ResourceAccess) {
        (self.register_accesses)(accesses)
    }

    fn record(&mut self, command_buffer: CommandBuffer) {
        (self.record)(command_buffer)
    }
}
