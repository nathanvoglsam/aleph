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
