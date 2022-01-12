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

pub struct CommandBuffer<T: CommandBufferState> {
    state: T,
}

impl CommandBuffer<Closed> {
    #[inline]
    pub fn open(self) -> CommandBuffer<Open> {
        todo!()
    }
}

impl CommandBuffer<Open> {
    #[inline]
    pub fn close(self) -> CommandBuffer<Closed> {
        todo!()
    }
}

impl CommandBuffer<Any> {
    #[inline]
    pub fn to_open(self) -> Result<CommandBuffer<Open>, Self> {
        match self.state.0 {
            CommandBufferStates::Open(v) => Ok(v),
            CommandBufferStates::Closed(_) => Err(self),
        }
    }

    #[inline]
    pub fn to_closed(self) -> Result<CommandBuffer<Closed>, Self> {
        match self.state.0 {
            CommandBufferStates::Open(_) => Err(self),
            CommandBufferStates::Closed(v) => Ok(v),
        }
    }

    #[inline]
    pub fn to_state(self) -> CommandBufferStates {
        self.state.0
    }

    #[inline]
    pub fn is_open(&self) -> bool {
        match self.state.0 {
            CommandBufferStates::Open(_) => true,
            CommandBufferStates::Closed(_) => false,
        }
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }
}

/// Trait used for restricting set of markers used for command buffer state
pub trait CommandBufferState {}

/// Marker type for an open command buffer
pub struct Open();

/// Marker type for a closed command buffer
pub struct Closed();

/// Marker type and state for a command buffer with dynamic state tracking
pub struct Any(CommandBufferStates);

impl CommandBufferState for Open {}
impl CommandBufferState for Closed {}
impl CommandBufferState for Any {}

/// Enumeration of all possible command buffer states
pub enum CommandBufferStates {
    Open(CommandBuffer<Open>),
    Closed(CommandBuffer<Closed>),
}
