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

use crate::checkers::{BasicBlockChecker, ModuleChecker};
use crate::error::TypeCheckResult;
use eon::function::Function;
use eon::indexes::FunctionIndex;
use eon::module::Module;

pub struct FunctionChecker<'module, 'module_checker> {
    pub module_checker: &'module_checker ModuleChecker<'module>,
    pub function: &'module Function,
    pub function_index: FunctionIndex,
}

impl<'module, 'module_checker> FunctionChecker<'module, 'module_checker> {
    pub fn new(
        module_checker: &'module_checker ModuleChecker<'module>,
        function: &'module eon::function::Function,
        function_index: FunctionIndex,
    ) -> Self {
        Self {
            module_checker,
            function,
            function_index,
        }
    }

    pub fn check(&self) -> TypeCheckResult<()> {
        for (i, basic_block) in self.function.basic_blocks.iter().enumerate() {
            BasicBlockChecker::new(self, basic_block, i.into()).check()?;
        }
        Ok(())
    }

    pub fn module(&self) -> &'module Module {
        self.module_checker.module
    }
}
