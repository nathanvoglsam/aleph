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

use crate::interfaces::any::AnyArc;
use crate::interfaces::plugin::IQuitHandle;
use std::sync::atomic::{AtomicBool, Ordering};

///
/// The struct that implements `IQuitHandle`
///
pub struct QuitHandleImpl {
    pub wants_quit: AtomicBool,
}

interfaces::any::declare_interfaces!(QuitHandleImpl, [IQuitHandle]);

impl IQuitHandle for QuitHandleImpl {
    fn quit(&self) {
        self.wants_quit.store(true, Ordering::Relaxed)
    }

    fn quit_requested(&self) -> bool {
        self.wants_quit.load(Ordering::Relaxed)
    }
}

impl QuitHandleImpl {
    pub fn new() -> AnyArc<QuitHandleImpl> {
        let out = Self {
            wants_quit: AtomicBool::new(false),
        };
        AnyArc::new(out)
    }
}
