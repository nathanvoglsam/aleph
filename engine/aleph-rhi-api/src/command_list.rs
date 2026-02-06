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

use aleph_any::IAny;
use thiserror::Error;

use crate::*;

pub trait ICommandList: IAny + IGetPlatformInterface + Send {
    fn begin_general(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError>;

    fn begin_compute(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError>;

    fn begin_transfer(&mut self) -> Result<CommandEncoder<'_>, CommandListBeginError>;
}

pub struct CommandListDesc<'a> {
    /// The type of queue this command list will be compatible with. This affects what kinds of
    /// commands can be recorded and what kind of queues the list can be submitted to.
    pub queue_type: QueueType,

    /// The name of the object
    pub name: Option<&'a str>,
}

#[derive(Error, Debug)]
pub enum CommandListBeginError {
    #[error("The command list is not ready to begin recording commands")]
    InvalidCommandListState,

    #[error("The command list does not support encoding commands for a '{0}' queue")]
    InvalidEncoderType(QueueType),

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}

#[derive(Error, Debug)]
pub enum CommandListCloseError {
    #[error("The command list was already closed but has been closed again.")]
    AlreadyClosed,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
