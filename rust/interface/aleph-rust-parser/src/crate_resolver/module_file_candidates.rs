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
/// Internal enum for representing the `ModuleFileCandidates` state machine
///
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum ModuleFileCandidatesState {
    SubFolder,
    Name,
    End,
}

impl ModuleFileCandidatesState {
    ///
    /// Get the next value in the sequence
    ///
    const fn next(self) -> ModuleFileCandidatesState {
        match self {
            ModuleFileCandidatesState::SubFolder => ModuleFileCandidatesState::Name,
            ModuleFileCandidatesState::Name => ModuleFileCandidatesState::End,
            ModuleFileCandidatesState::End => ModuleFileCandidatesState::End,
        }
    }
}

///
/// An iterator type that yields a series of strings that represent *relative* paths that are
/// potential candidates for a rust module's file to be found
///
pub struct ModuleFileCandidates<'a> {
    name: &'a str,
    state: ModuleFileCandidatesState,
}

impl<'a> ModuleFileCandidates<'a> {
    ///
    /// Creates an iterator that will yield possible
    ///
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            state: ModuleFileCandidatesState::SubFolder,
        }
    }
}

impl<'a> Iterator for ModuleFileCandidates<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.state {
            ModuleFileCandidatesState::SubFolder => Some(format!("{}/mod.rs", self.name)),
            ModuleFileCandidatesState::Name => Some(format!("{}.rs", self.name)),
            ModuleFileCandidatesState::End => None,
        };
        self.state = self.state.next();
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (2, Some(2))
    }
}
