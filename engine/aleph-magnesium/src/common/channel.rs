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

/// An error returned from the 'recv' method.
///
/// A message could not be received because the channel is empty and disconnected.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RecvError;

/// An error returned from when attempting to receive a message over a channel.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TryRecvError {
    /// A message could not be received because the channel is empty.
    ///
    /// If this is a zero-capacity channel, then the error indicates that there was no sender
    /// available to send a message at the time.
    Empty,

    /// The message could not be received because the channel is empty and disconnected.
    Disconnected,
}

impl From<std::sync::mpsc::TryRecvError> for TryRecvError {
    #[inline]
    fn from(value: std::sync::mpsc::TryRecvError) -> Self {
        match value {
            std::sync::mpsc::TryRecvError::Empty => Self::Empty,
            std::sync::mpsc::TryRecvError::Disconnected => Self::Disconnected,
        }
    }
}

impl From<crossbeam::channel::TryRecvError> for TryRecvError {
    #[inline]
    fn from(value: crossbeam::channel::TryRecvError) -> Self {
        match value {
            crossbeam::channel::TryRecvError::Empty => Self::Empty,
            crossbeam::channel::TryRecvError::Disconnected => Self::Disconnected,
        }
    }
}
