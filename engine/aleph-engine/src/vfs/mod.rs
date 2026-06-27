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

use crate::vpath::VPath;

pub struct Router {

}

/// Describes the complete set of layers to mount together into a VFS.
pub struct MountDesc<'a> {
    /// A loosely sequenced set of layers that will be mounted in sorted order based on the
    /// priority declared in the layer.
    ///
    /// We guarantee to sort with a stable algorithm, so given a set of layers sharing the same
    /// priority we will preserve their order as found in the input set.
    pub layers: &'a [LayerDesc<'a>],
}

/// Describes how to mount a single layer within a VFS.
pub struct LayerDesc<'a> {
    /// A sort-key that is used to order this layer against other layers in the set when
    /// initializing a VFS.
    pub priority: u64,

    /// What path within the vfs that the given layer should be mounted at.
    pub mount_path: &'a VPath,

    /// The fs layer to mount.
    pub layer: Option<Box<dyn ILayer>>,
}

/// Interface expected of a 'layer' interface. A 'layer' is expected to be mounted into a vfs as a
/// mount point and should be able to translate paths into 'files' within itself.
///
/// 'Layers' are generally expected to abstract over their underlying representation.
pub trait ILayer {
    /// Will be called when installing the layer into a VFS exactly once.
    ///
    /// This is intended to be used to initialize any internal state before we 'freeze' the objects
    /// to allow sharing the VFS across threads.
    ///
    /// It is incorrect to query a layer before calling this function to install it. Using any of
    /// the query functions without calling 'install' first will produce inconsistent results.
    fn install(&mut self, mount_point: &VPath);

    /// Query for an entity at the given 'path'.
    ///
    /// 'path' must be a local path scoped to just this _layer_. The mount point should not be
    /// included.
    ///
    /// If 'path' is relative then it will be assumed to be relative to the root of this layer. If
    /// 'path' is absolute then the 'root' will be defined as the root of this layer.
    fn query_entity(&self, path: &VPath);
}