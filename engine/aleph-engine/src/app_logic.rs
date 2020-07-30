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

///
/// A user's main interface with Aleph. This represents the life cycle of the entire application.
///
pub trait AppLogic: Send {
    ///
    /// This method will be called once on application start up. Anything that needs to be called
    /// once for the entire life cycle of the application goes here.
    ///
    /// This will be called after every engine subsystem has been initialized, right before
    /// `on_update` would have been called for the first time. Trust that the engine is fully
    /// initialized by the time this is called.
    ///
    fn on_init(&mut self);

    ///
    /// This method will be called once per frame. Prior to rendering and before any other `Logic`
    /// instances.
    ///
    fn on_update(&mut self, ui: &aleph_imgui::Ui);

    ///
    /// This method will be called once on application exit. This would usually be used for closing
    /// resources and de-initializing whatever had been done in `on_init`.
    ///
    /// This will be called before any engine subsystems begin shutdown so it can be trusted that
    /// they are still safe to use.
    ///
    fn on_exit(&mut self);
}
