//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{RenderGraphPass, ResourceAccess};

///
/// A simple pass that wraps some closures in an object that implements the `RenderGraphPass` trait
///
pub struct ClosurePass<RegAcc: FnMut(&mut ResourceAccess), Comp: FnMut(), Rec: FnMut()> {
    register_accesses: RegAcc,
    compile: Comp,
    record: Rec,
}

impl<RegAcc, Comp, Rec> ClosurePass<RegAcc, Comp, Rec>
where
    RegAcc: FnMut(&mut ResourceAccess),
    Comp: FnMut(),
    Rec: FnMut(),
{
    ///
    /// Builds a new ClosurePass from the provided closure objects
    ///
    pub fn new(register_accesses: RegAcc, compile: Comp, record: Rec) -> Self {
        Self {
            register_accesses,
            compile,
            record,
        }
    }
}

impl<RegAcc, Comp, Rec> RenderGraphPass for ClosurePass<RegAcc, Comp, Rec>
where
    RegAcc: FnMut(&mut ResourceAccess),
    Comp: FnMut(),
    Rec: FnMut(),
{
    fn register_access(&mut self, accesses: &mut ResourceAccess) {
        (self.register_accesses)(accesses)
    }

    fn compile(&mut self) {
        (self.compile)()
    }

    fn record(&mut self) {
        (self.record)()
    }
}
