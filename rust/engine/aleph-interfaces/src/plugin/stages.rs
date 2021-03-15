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

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(u32)]
pub enum InitStage {
    /// Represents the first init stage
    Core = 0,

    /// Represents the second init stage that occurs after `InitStage::Core`
    Main = 1,
}

impl InitStage {
    /// The number of stages
    pub const STAGE_COUNT: usize = 2;
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
#[repr(u32)]
pub enum UpdateStage {
    /// The first update stage. Semantically should be used by platform implementations for
    /// collecting input from the host.
    InputCollection = 0,

    /// The second update stage. Semantically should be used to run code before the bulk of gameplay
    /// code will be run.
    PreUpdate = 1,

    /// The third update stage. Semantically should be used for implementing gameplay logic, like
    /// player controllers, AI, etc.
    Update = 2,

    /// The fourth update stage. Semantically should be used for implementing logic that needs to
    /// run immediately after gameplay logic, but before the rendering stage.
    PostUpdate = 3,

    /// The fifth update stage. Semantically should be used for implementing rendering logic.
    Render = 4,
}

impl UpdateStage {
    pub const STAGE_COUNT: usize = 5;
}
