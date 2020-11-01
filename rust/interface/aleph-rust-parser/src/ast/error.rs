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

/// The set of errors that can be thrown while generating an interface
#[derive(Debug)]
pub enum GeneratorError {
    /// Tried to generate an interface for a method with an unsupported signature
    UnsupportedMethodSignature,

    /// Tried to generate an interface for a struct field with an unsupported type
    UnsupportedStructField,

    /// This error occurs when the code being parsed has attached the interface attribute to an
    /// object with generic parameters.
    AttributeOnGenericObject,

    /// An item was declared with `pub(in path)` or `pub(super)` visibility specifier. This is not
    /// supported due to complicating the parser implementation significantly.
    UnsupportedVisibility,

    /// Attempting to use with a path of the form `::something::else` is invalid as it, by
    /// definition, refers to objects outside of the current crate which we can't parse. As such we
    /// can't ever handle these imports in a sane way so just make them a hard error.
    InvalidUsePath,

    /// If we detect invalid rust syntax in our walk of tree from syn
    InvalidUseSyntax,

    /// Occurs when two objects in the same scope are declared with the same name
    MultipleObjectsWithSameName,

    /// Occurs when a `use` statement tries to import an object which doesn't exist in the crate
    ImportedObjectDoesNotExist,

    /// Occurs when a type that doesn't exist is referred to
    ReferencedObjectDoesNotExist,
}
