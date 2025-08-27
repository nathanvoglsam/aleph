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

use crate::*;

/// An enumeration of the supported set of shader input types.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderBinary<'a> {
    /// This variant encloses a SPIR-V binary. Only supported by the `Vulkan` backend.
    Spirv(&'a [u8]),

    /// This variant encloses a DXIL binary. Only supported by the `D3D12` backend.
    Dxil(&'a [u8]),
}

/// An enumeration of all individual shader types
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ShaderType {
    Compute,
    Vertex,
    Hull,
    Domain,
    Geometry,
    Fragment,
    Amplification,
    Mesh,
}

impl std::fmt::Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderType::Compute => f.write_str("Compute"),
            ShaderType::Vertex => f.write_str("Vertex"),
            ShaderType::Hull => f.write_str("Hull"),
            ShaderType::Domain => f.write_str("Domain"),
            ShaderType::Geometry => f.write_str("Geometry"),
            ShaderType::Fragment => f.write_str("Fragment"),
            ShaderType::Amplification => f.write_str("Amplification"),
            ShaderType::Mesh => f.write_str("Mesh"),
        }
    }
}

impl Default for ShaderType {
    #[inline(always)]
    fn default() -> Self {
        Self::Compute
    }
}

/// This trait provides an opaque interface for an RHI implementation to query shader data from
/// an outside source. An implementation of this trait is expected to provide the shader code for
/// each backend on request from the relevant API entry point.
///
/// This is expected to be implemented by an external type and passed into the RHI.
///
/// # Safety
///
/// This interface is marked as 'unsafe' because the shader code is unverified, and the RHI is not
/// required to do any verification on its own. The shader code could be malformed, or the shader
/// type provided could be incorrect for the provided shader code, or any number of other problems.
///
/// The RHI is not expected to check anything. It is up to the implementer to provide the guarantee
/// that the shader is well formed and all information provided matches.
///
/// Any reflection shader information provided from this interface is also entirely unverified. The
/// RHI is not required to make any effort to enforce that the reflection is correct w.r.t. the
/// provided shader code.
///
/// It is entirely the implementer's responsibility to ensure that the reflection information
/// matches the shader code it is being associated with.
///
/// # Safety 2
///
/// You will almost certainly not be able to make using this interface truly safe, unless you run
/// the entire shader pipeline from source code to bytecode + reflection in-process (and assume
/// there are no bugs in the compiler). The only backend that it's viable to perform a full,
/// in-depth input validation on is Vulkan because SPIR-V is well specified and easy to read.
///
/// D3D12 is _theoretically_ possible to verify as the bytecode is "just" LLVM bitcode. In practice
/// it's a fool's errand as you need to pull in half of LLVM to read it, and DXIL uses a mountain of
/// adhoc bolt-on semantics to pure LLVM that make the validation implementation very bloated.
/// There's also no public spec to validate against.
///
/// Metal is _not_ possible unless you run the slang compiler in-process as you must either
/// implement an MSL (read: C++) parser or reverse engineer the Metal shader bytecode format to
/// extract the information needed to validate. Good luck with either of them.
///
/// You'll just have to trust your shader inputs. D3D12 might get better when they adopt SPIR-V.
pub unsafe trait IShaderCodeSource {
    /// Returns what kind of shader module the shader code represents.
    fn shader_type(&self) -> ShaderType;

    /// Returns the name of the shader module. This is used for integration with debug utilities.
    fn shader_name(&self) -> &str;

    /// Returns a reference to shader code appropriate for consumption by `Vulkan`.
    fn get_vulkan(&self) -> &dyn IShaderPlatformData;

    /// Returns a reference to shader code appropriate for consumption by `D3D12`.
    fn get_d3d12(&self) -> &dyn IShaderPlatformData;

    /// Returns a reference to shader code appropriate for consumption by `Metal`.
    fn get_metal(&self) -> &dyn IShaderPlatformData;
}

/// Accessor interface for an individual target platform's shader data.
///
/// # Safety
///
/// See [`IShaderCodeSource`] documentation.
pub unsafe trait IShaderPlatformData {
    /// Returns the number of parameter blocks the associated shader uses.
    fn get_parameter_block_count(&self) -> usize;

    /// Returns the number of parameters the associated shader uses in the requested block.
    ///
    /// May panic if 'block' is > the result of
    /// [`IShaderPlatformData::get_parameter_block_count`].
    fn get_parameter_count_for_block(&self, block: usize) -> usize;

    /// Writes out a full set of [`ParameterDesc`] structs into the `dst` array that fully describes
    /// the requested parameter block.
    ///
    /// The required length of `dst` can be queried by using
    /// [`IShaderPlatformData::get_parameter_count_for_block`].
    fn get_parameters_for_block(&self, block: usize, dst: &mut [ParameterDesc]);

    /// Returns the reflected push constant block description. If the shader does not use a push
    /// constant block then this will return `None`.
    fn get_push_constant_block(&self) -> Option<PushConstantBlock>;

    /// Returns a reference to the shader code appropriate for consumption by the target API.
    fn get_code(&self) -> &[u8];
}
