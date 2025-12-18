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

/// This is a tag we associate with push constant blocks so that we can identify them in the shader
/// output. This is consumed by our shader pipeline to identify a push constant block in the shader
/// for platforms that don't have an explicit concept of push constants like Metal.
///
/// We also use this attribute to identify any push-constant data that has crept in through means
/// other than our RHI sanctioned method.
[__AttributeUsage(_AttributeTargets.Var)]
struct aleph_PushConstantBlockAttribute {};

/// Default to 'TARGET_SPIRV' if no target is defined. This will be the case in the language server
/// which is not aware of our target macros. By defaulting to a target the language server wont
/// die.
#if !defined(TARGET_SPIRV) && !defined(TARGET_METAL) && !defined(TARGET_DXIL)
#define TARGET_SPIRV
#endif

/// PUSH_CONSTANT impl for SPIR-V
#define PUSH_CONSTANT_SPV(T, name) [aleph::PushConstantBlock] [[vk::push_constant]] ConstantBuffer<T> name

/// PUSH_CONSTANT impl for D3D
#define PUSH_CONSTANT_D3D(T, name) [aleph::PushConstantBlock] ConstantBuffer<T> name : register(b0, space1024)

/// PUSH_CONSTANT impl for Metal
#define PUSH_CONSTANT_MTL(T, name) [aleph::PushConstantBlock] ConstantBuffer<T> name : register(b9)

/// Map the impl to the platform as a separate layer so we can document API directly on
/// 'PUSH_CONSTANT'.
///
/// Fallback to spirv to satisfy the language server if no macro is defined.
#if defined(TARGET_SPIRV)
#define PUSH_CONSTANT_IMPL(T, name) PUSH_CONSTANT_SPV(T, name)
#elif defined(TARGET_DXIL)
#define PUSH_CONSTANT_IMPL(T, name) PUSH_CONSTANT_D3D(T, name)
#elif defined(TARGET_METAL)
#define PUSH_CONSTANT_IMPL(T, name) PUSH_CONSTANT_MTL(T, name)
#endif

/// Macro that will declare a push constant block that is compatible with the target API in the
/// format that aleph-rhi expects push constants to be declared with.
///
/// - 'T' is the struct type of the push constant block.
/// - 'name' is the name the value will take.
///
/// Push constants are quite special in each API and they're different enough between APIs that we
/// need some shader-side glue to make them work. We'd like to have avoided needing macros to do it
/// but there doesn't seem to be any other way to do it. SPIR-V is easy, but DX needs a magic ID
/// tagged to a constant buffer, and Metal just treats them like a regular buffer binding so we need
/// to identify which one is the push-constant block somehow.
///
/// In the future we _could_ use user attributes, but the reflection JSON output for those is
/// broken at the moment so instead for non SPIR-V platforms we just use a magic number as the
/// binding slot and hope that works fine.
#define PUSH_CONSTANT(T, name) PUSH_CONSTANT_IMPL(T, name)
