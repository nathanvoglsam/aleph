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

use std::sync::Arc;

use crossbeam::atomic::AtomicCell;
use interfaces::world::{IWorldCell, IWorldProvider, World, WorldOptions, WorldScope};

pub struct WorldProvider {
    world: Arc<dyn IWorldCell>,
}

impl WorldProvider {
    pub fn new() -> Self {
        let world = World::new(WorldOptions::default()).unwrap();
        let world = Box::new(world);
        let world = WorldCell {
            cell: AtomicCell::new(Some(world)),
        };
        Self {
            world: Arc::new(world),
        }
    }
}

impl IWorldProvider for WorldProvider {
    fn get(&self) -> Arc<dyn IWorldCell> {
        self.world.clone()
    }
}

interfaces::any::declare_interfaces!(WorldProvider, [IWorldProvider]);

/// Internal implementation of IWorldCell
struct WorldCell {
    cell: AtomicCell<Option<Box<World>>>,
}

impl IWorldCell for WorldCell {
    fn take(&self) -> Box<World> {
        self.cell.take().unwrap()
    }

    fn store(&self, world: Box<World>) {
        assert!(self.cell.swap(Some(world)).is_none());
    }

    fn get(&self) -> WorldScope {
        WorldScope::take_from(self)
    }
}
