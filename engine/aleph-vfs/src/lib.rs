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

pub mod async_io;
pub mod directory_layer;
pub mod file;
pub mod path;

#[cfg(test)]
mod tests;

use std::io;
use std::sync::Arc;

use aleph_alloc::instrumentation::{IAllocationCategory, system};
use aleph_alloc::{BBox, BHashMap};

use crate::file::{IAsyncVFile, VFile};
use crate::path::{Component, Components, VPath};

/// The top level interface to our 'vfs' implementation.
///
/// # VFS - Virtual File System
///
/// This implements a heavily simplified file-system like interface that allows symbolically
/// addressing binary files in a virtual directory tree. The main three concepts are:
///
/// - Layers
/// - Mounts
/// - VFiles
///
/// ## Layers
///
/// Layers are an abstract interface [`ILayer`]. They provide access to a subtree that can be
/// queried to access files within it. The underlying source of the files is opaque. The expectation
/// is that different layer implementations project different sources of data into the virtual
/// file system.
///
/// You could source data from:
/// - A zip archive
/// - A folder on disk
/// - A remote server over the network
///
/// Layers are _mounted_ into the vfs at mount points at the root of the virtual directory tree.
///
/// ## Mounts
///
/// A mount takes an [`ILayer`] implementation and makes it available under a named directory at
/// the root of the virtual file tree. They are configured with [`LayerDesc`]. Multiple layers can
/// be mounted into the same tree, and each layer can use a different [`ILayer`] implementation.
///
/// Once a router has been constructed from a list of mounts, the physical source (network, disk,
/// archive) is not important to the downstream users. The intention is to allow downstream data
/// loading code to abstract over exactly where the data comes from.
///
/// In a game engine we need to be able to load assets from numerous sources, including fully baked
/// asset bundles as well as unpackaged data on the OS filesystem during development. We also want
/// to abstract the physical organization of the data. We don't care where the packages are in
/// relation to the game executable, only that we can access files with the expected symbolic paths.
///
/// ## VFiles
///
/// [`VFile`] is our vfs analogue of a file handle. They are `!Send` and expose a heavily simplified
/// interface compared. All we care about is being able to read a stream of bytes.
///
/// There are also extended interfaces for working with accelerated aysnc io tools.
///
/// # Limitations
///
/// This is not a general purpose file system, and is heavily simplified down to the bare essentials
/// for storing game assets. All layers and files are read-only. You can't open directories to
/// enumerate the files and folders they contain. There is no file metadata like mtime. There are no
/// symlinks.
///
/// You get 3 fundamental operations:
/// - open
/// - read
/// - close
///
/// We only allow mounting layers at the root. Two layers 'A' and 'B' can be made available as '/A'
/// and '/B', but never '/A/B' or '/C/A' to prevent mount points from overlapping. This is an
/// intentional restriction to simplify the implementation. Overlapping mounts would complicate
/// mapping a virtual file to the source and reduce runtime efficiency. This prevents using overlays
/// to 'patch' files with replacements. However, we see this is a potential footgun that can be
/// resolved at higher layers of the engine.
///
/// All mounted files are immutable. We don't allow mutating the data through this interface, but
/// outside processes or other code could modify the mounted files. This will cause problems, and we
/// make no guarantees about expected behavior other than the interface must remain safe. Do not
/// mutate mounted data.
pub struct Router {
    layers: BHashMap<String, BBox<dyn ILayer, VfsSystem>, VfsSystem>,
}

impl Router {
    /// Construct a new [`Router`], mounting the given set of layers at the described mount points.
    ///
    /// This will collect all the layers in the given source iterator. They will be 'mounted' into
    /// the internal tree. Then [`ILayer::install`] will be called so te layers can finalize their
    /// setup.
    ///
    /// # Failure
    ///
    /// This will fail if you attempt to mount to layers onto the same mount point.
    ///
    /// Any io errors from [`ILayer::install`] will be propagated to the caller.
    ///
    /// Upon failure all the layers will be dropped.
    pub fn new<'l, L: IntoIterator<Item = LayerDesc<'l>>>(in_layers: L) -> io::Result<Self> {
        Self::__new(in_layers.into_iter())
    }

    fn __new<'l, L: Iterator<Item = LayerDesc<'l>>>(in_layers: L) -> io::Result<Self> {
        let mut layers = BHashMap::default();

        for layer in in_layers {
            let name = Vfs::with(|| String::from(layer.mount_name));

            assert!(
                layers.insert(name, layer.layer).is_none(),
                "You can't mount multiple layers at the same mount name"
            );
        }

        for (mount_name, layer) in layers.iter_mut() {
            layer.install(mount_name)?;
        }

        Ok(Self { layers })
    }

    /// The core implementation of `open` with the generic args stripped away so we don't monomorph
    /// the whole method body for every type that implements `AsRef<VPath>`.
    fn ___open(&self, path: &VPath) -> io::Result<VFile<'_>> {
        let mut components = path.components();

        let layer_name = Self::parse_target_layer(&mut components)?;

        // Try and find the layer mounted at the given name
        if let Some(layer) = self.layers.get(layer_name) {
            // Take the remaining path in 'components' as the path we send into the layer to find
            // the true asset.
            layer.query_entity(components.as_path())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No such file."))
        }
    }

    /// The core implementation of `open_async` with the generic args stripped away so we don't
    /// monomorph the whole method body for every type that implements `AsRef<VPath>`.
    fn ___open_async(&self, path: &VPath) -> io::Result<Arc<dyn IAsyncVFile>> {
        let mut components = path.components();

        let layer_name = Self::parse_target_layer(&mut components)?;

        // Try and find the layer mounted at the given name
        if let Some(layer) = self.layers.get(layer_name) {
            // Take the remaining path in 'components' as the path we send into the layer to find
            // the true asset.
            layer.query_entity_async_io(components.as_path())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No such file."))
        }
    }

    fn parse_target_layer<'a>(components: &'a mut Components) -> io::Result<&'a str> {
        let layer_name = match components.next() {
            // The empty path categorically doesn't refer to any elements, so bail
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidFilename,
                    "Empty path is not a file.",
                ));
            }

            // If we have a leading root segment we skip it to find the first named segment
            Some(Component::Root) => match components.next() {
                // There is no readable entry at the root, and the path points there. Bail.
                None => return Err(io::Error::new(io::ErrorKind::IsADirectory, "Not a file.")),

                // This should be impossible, Components can only yield 'Root' once.
                Some(Component::Root) => unreachable!("How did 'Components' yield 'Root' twice?"),

                // This is the first named segment, which is the layer name we're looking for.
                Some(Component::Segment(seg)) => seg,
            },

            // There is no CWD concept, so a relative path is simply defined as relative to the
            // root. The first segment in this case is the name of the layer.
            Some(Component::Segment(seg)) => seg,
        };

        // We intentionally skip ".." components. No good can come from trying to
        // handle '..'. It's not a sane name for a file, so they only real time
        // you'll see this is either a bug or someone intentionally trying to break
        // things.
        //
        // We skip "." components too because they are likely to cause problems too.
        if matches!(layer_name, ".." | ".") {
            Err(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "Path contains '..' | '.' segments.",
            ))
        } else {
            Ok(layer_name)
        }
    }
}

impl IRouter for Router {
    fn __open(&self, path: &VPath) -> io::Result<VFile<'_>> {
        self.___open(path)
    }

    fn __open_async(&self, path: &VPath) -> io::Result<Arc<dyn IAsyncVFile>> {
        self.___open_async(path)
    }
}

/// 'ABI' level trait that exposes the interface for [`Router`] as a trait object. See
/// [`IRouterExt`] for cleaner interfaces.
pub trait IRouter: Send + Sync + 'static {
    /// The core implementation of [`Router::open`] with the generic args stripped away so we don't
    /// monomorph the whole method body for every type that implements `AsRef<VPath>`.
    ///
    /// Use [`Router::open`] or [`IRouter::open`] instead.
    fn __open(&self, path: &VPath) -> io::Result<VFile<'_>>;

    /// The core implementation of [`Router::open`] with the generic args stripped away so we don't
    /// monomorph the whole method body for every type that implements `AsRef<VPath>`.
    ///
    /// Use [`Router::open_async`] or [`IRouter::open_async`] instead.
    fn __open_async(&self, path: &VPath) -> io::Result<Arc<dyn IAsyncVFile>>;
}

/// An extension over [`IRouter`] that providers neater interfaces. We need this layer because we
/// can't use generic functions on traits that can be trait objets.
pub trait IRouterExt: IRouter + Send + Sync + 'static {
    /// Attempts to open a [`VFile`] by searching for a file at the given path.
    fn open<P: AsRef<VPath>>(&self, path: P) -> io::Result<VFile<'_>> {
        self.__open(path.as_ref())
    }

    /// Attempts to open a [`IAsyncVFile`] by searching for a file at the given path.
    fn open_async<P: AsRef<VPath>>(&self, path: P) -> io::Result<Arc<dyn IAsyncVFile>> {
        self.__open_async(path.as_ref())
    }
}

impl<T: IRouter + ?Sized> IRouterExt for T {}

/// Describes how to mount a single layer within a VFS.
pub struct LayerDesc<'a> {
    /// What name the layer should be given/mounted at within the router.
    pub mount_name: &'a str,

    /// The fs layer to mount.
    pub layer: BBox<dyn ILayer, VfsSystem>,
}

/// Interface expected of a 'layer'. A 'layer' will be mounted into a vfs as a mount point and
/// should be able to translate paths into 'files' within itself.
///
/// 'Layers' are generally expected to abstract over their underlying representation.
pub trait ILayer: Send + Sync + 'static {
    /// Will be called when installing the layer into a VFS exactly once.
    ///
    /// This is intended to be used to initialize any internal state before we 'freeze' the objects
    /// to allow sharing the VFS across threads.
    ///
    /// It is incorrect to query a layer before calling this function to install it. Using any of
    /// the query functions without calling 'install' first will produce inconsistent results.
    fn install(&mut self, mount_name: &str) -> io::Result<()>;

    /// Query for an entity at the given 'path'.
    ///
    /// 'path' must be a local path scoped to just this _layer_. The mount point should not be
    /// included.
    ///
    /// If 'path' is relative then it will be assumed to be relative to the root of this layer. If
    /// 'path' is absolute then the 'root' will be defined as the root of this layer.
    fn query_entity(&self, path: &VPath) -> io::Result<VFile<'_>>;

    fn query_entity_async_io(&self, path: &VPath) -> io::Result<Arc<dyn IAsyncVFile>>;
}

/// Utility for boxing a layer implementation into the tagged box types we use for allocation
/// tracking.
pub fn box_layer<T: ILayer>(layer: T) -> BBox<dyn ILayer, VfsSystem> {
    let (ptr, allocator) = BBox::into_raw_with_allocator(BBox::new_in(layer, system()));
    let ptr: *mut _ = ptr;
    unsafe { BBox::from_raw_in(ptr, allocator) }
}

pub struct Vfs;
aleph_alloc::new_alloc_category!(Vfs, "019f06a8-fabd-7761-b89c-e5d13c09926c");

pub type VfsSystem = aleph_alloc::instrumentation::Instrumented<Vfs>;
