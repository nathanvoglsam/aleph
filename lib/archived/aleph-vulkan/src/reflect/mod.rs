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

mod member_resolution;
mod push_constant;
mod set;
mod structure;
mod vertex_layout;

pub use member_resolution::MemberResolutionError;
pub use push_constant::PushConstantReflection;
pub use set::Binding;
pub use set::BindingMapperFn;
pub use set::BindingType;
pub use set::DescriptorSetReflection;
pub use structure::FloatType;
pub use structure::IntegerType;
pub use structure::MatrixInfo;
pub use structure::MatrixLayout;
pub use structure::Member;
pub use structure::MemberType;
pub use structure::ScalarType;
pub use structure::Struct;
pub use structure::StructResolutionError;
pub use structure::VectorInfo;
pub use vertex_layout::VertexLayoutReflection;
pub use vertex_layout::VertexLayoutResolutionError;
