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

use crate::gpu::{CommandListCreateError, IGeneralCommandList, INamedObject};
use any::{AnyArc, IAny};
use std::any::Any;
use thiserror::Error;

pub trait ICommandPool: INamedObject + Send + Sync + IAny + Any + 'static {
    any_arc_trait_utils_decl!(ICommandPool);

    fn create_general_command_list(
        &self,
    ) -> Result<Box<dyn IGeneralCommandList>, CommandListCreateError>;
}

#[derive(Error, Debug)]
pub enum CommandPoolCreateError {
    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
