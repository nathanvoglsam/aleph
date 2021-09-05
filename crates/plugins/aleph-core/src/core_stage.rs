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

use aleph_label::Label;

/// This enum provides a [`Label`] type that names the core engine execution stages.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum CoreStage {
    /// This stage runs at the very beginning of a frame and should be primarily used for collecting
    /// input from the keyboard/mouse/controllers/etc...
    InputCollection,

    /// This stage runs directly before [CoreStage::Update] and has the primary purpose of allowing
    /// systems to setup engine state before the update stage.
    PreUpdate,

    /// This is the main update stage where the bulk of all gameplay code should be placed. Any
    /// other stage will be used almost exclusively by engine systems to make whatever happens in
    /// the update stage work
    Update,

    /// This stage runs directly after [CoreStage::Update] and is the counterpart
    /// [CoreStage::PreUpdate], used for scheduling core to run after the update stage.
    PostUpdate,

    /// This is the final execution stage that runs after all other stages. This should primarily
    /// be used by the renderer to drive record and submit GPU work at a stage where all gameplay
    /// code has finished.
    Render,
}

impl Label for CoreStage {
    fn dyn_clone(&self) -> Box<dyn Label> {
        Box::new(*self)
    }
}
