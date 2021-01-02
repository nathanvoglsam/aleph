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

use crate::checkers::{FunctionChecker, OpChecker};
use crate::error::TypeCheckResult;
use eon::function::Function;
use eon::indexes::BasicBlockIndex;
use eon::module::Module;

pub struct BasicBlockChecker<'module, 'module_checker, 'function_checker> {
    pub function_checker: &'function_checker FunctionChecker<'module, 'module_checker>,
    pub basic_block: &'module eon::function::BasicBlock,
    pub basic_block_index: BasicBlockIndex,
}

impl<'module, 'module_checker, 'function_checker>
    BasicBlockChecker<'module, 'module_checker, 'function_checker>
{
    pub fn new(
        function_checker: &'module_checker FunctionChecker<'module, 'module_checker>,
        basic_block: &'module eon::function::BasicBlock,
        basic_block_index: BasicBlockIndex,
    ) -> Self {
        Self {
            function_checker,
            basic_block,
            basic_block_index,
        }
    }

    pub fn check(&self) -> TypeCheckResult<()> {
        for (i, op) in self.basic_block.ops.iter().enumerate() {
            OpChecker::new(self, op, i.into()).check()?;
        }
        Ok(())
    }

    pub fn module(&self) -> &'module Module {
        self.function_checker.module()
    }

    pub fn function(&self) -> &'module Function {
        self.function_checker.function
    }
}
