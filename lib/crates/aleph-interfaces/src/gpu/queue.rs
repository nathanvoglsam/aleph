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

use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum QueueType {
    General,
    Compute,
    Transfer,
}

impl Display for QueueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueType::General => f.write_str("QueueType::General"),
            QueueType::Compute => f.write_str("QueueType::Compute"),
            QueueType::Transfer => f.write_str("QueueType::Transfer"),
        }
    }
}

#[derive(Error, Debug)]
pub enum QueueSubmitError {
    #[error("The queue '{0}' is not available")]
    QueueNotAvailable(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum QueuePresentError {
    #[error("The queue '{0}' does not support presentation to the requested swap chain")]
    QueuePresentationNotSupported(QueueType),

    #[error("The queue '{0}' is not available")]
    QueueNotAvailable(QueueType),

    #[error("An internal backend error has occurred '{0}'")]
    Platform(#[from] anyhow::Error),
}
