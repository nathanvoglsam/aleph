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

use crate::pass::resource_processor::GenerateMips;
use crate::{BufferHandle, BufferUploadDesc, TextureHandle, TextureUploadDesc};

#[derive(Default)]
pub struct ResourceCommandBuffer {
    pub(crate) bundle: CommandBundle,
}

impl ResourceCommandBuffer {
    pub fn new() -> Self {
        Self {
            bundle: CommandBundle::new(),
        }
    }

    pub fn push_command(&mut self, c: ResourceCommand) {
        self.bundle.push_command(c);
    }

    pub fn push_bundle(&mut self, c: ResourceCommandBuffer) {
        self.bundle.push_bundle(c.bundle);
    }

    pub(crate) fn walk<'a>(&'a self, mut f: impl FnMut(usize, &'a ResourceCommand)) {
        let mut i = 0;
        Self::walk_inner(&self.bundle, &mut i, &mut f);
    }

    fn walk_inner<'a>(
        b: &'a CommandBundle,
        i: &mut usize,
        f: &mut impl FnMut(usize, &'a ResourceCommand),
    ) {
        for c in b.bundle.iter() {
            match c {
                InternalCmd::Bundle(b) => {
                    Self::walk_inner(b, i, f);
                }
                InternalCmd::Direct(c) => {
                    f(*i, c);
                    *i += 1;
                }
            }
        }
    }
}

pub enum ResourceCommand {
    BufferUpload(BufferHandle, BufferUploadDesc),
    TextureUpload(TextureHandle, GenerateMips, TextureUploadDesc),
}

#[derive(Default)]
pub(crate) struct CommandBundle {
    /// The actual list of commands in the bundle
    pub(crate) bundle: Vec<InternalCmd>,

    /// The cummulative number of commands contained in this bundle. Includes the totals from all
    /// nested bundles
    pub(crate) len: usize,

    /// Flags whether the bundle contains any nested bundles
    pub(crate) contains_bundle: bool,
}

impl CommandBundle {
    const fn new() -> Self {
        Self {
            bundle: Vec::new(),
            len: 0,
            contains_bundle: false,
        }
    }

    fn push_command(&mut self, c: ResourceCommand) {
        self.len += 1;
        self.bundle.push(InternalCmd::Direct(c));
    }

    fn push_bundle(&mut self, c: CommandBundle) {
        // We add only the number of commands in the bundle. We do _not_ add 1 for the bundle as a
        // bundle is not a command.
        self.len += c.len;
        self.contains_bundle = true;
        self.bundle.push(InternalCmd::Bundle(c))
    }
}

pub(crate) enum InternalCmd {
    Bundle(CommandBundle),
    Direct(ResourceCommand),
}
