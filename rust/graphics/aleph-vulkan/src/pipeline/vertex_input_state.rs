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

use crate::embedded::data::CubeMesh;
use crate::format::{format_from_gltf_accessor, AccessorFormatError};
use crate::gltf::Semantic;
use aleph_vulkan_core::erupt::vk1_0::{
    Format, PipelineVertexInputStateCreateInfoBuilder, VertexInputAttributeDescriptionBuilder,
    VertexInputBindingDescriptionBuilder, VertexInputRate,
};
use std::mem::size_of;

///
/// An enum to represent the set of errors that can be generated when producing a vertex input
/// state description
///
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum VertexInputStateError {
    ///
    /// An error was encountered getting the data format for an attribute
    ///
    AccessorFormatError(AccessorFormatError),

    ///
    /// A generic error that can be thrown from the location mapping function
    ///
    LocationFunctionError(&'static str),
}

impl From<AccessorFormatError> for VertexInputStateError {
    fn from(other: AccessorFormatError) -> Self {
        VertexInputStateError::AccessorFormatError(other)
    }
}

impl From<&'static str> for VertexInputStateError {
    fn from(other: &'static str) -> Self {
        VertexInputStateError::LocationFunctionError(other)
    }
}

///
/// Namespace struct for creating vertex input descriptions
///
pub struct VertexInputState {}

impl VertexInputState {
    ///
    /// Provides a vertex binding and vertex attribute description for a pipeline that takes a
    /// single fullscreen quad
    ///
    pub fn for_fullscreen_quad<'a>(
        bindings: &'a mut Vec<VertexInputBindingDescriptionBuilder<'static>>,
        attributes: &'a mut Vec<VertexInputAttributeDescriptionBuilder<'static>>,
    ) {
        assert!(bindings.is_empty(), "List of bindings must be empty");
        assert!(attributes.is_empty(), "List of attributes must be empty");

        let binding = VertexInputBindingDescriptionBuilder::new()
            .binding(0)
            .stride(size_of::<f32>() as u32 * 2)
            .input_rate(VertexInputRate::VERTEX);
        bindings.push(binding);

        let attribute = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .format(Format::R32G32_SFLOAT)
            .offset(0)
            .location(0);
        attributes.push(attribute);
    }

    ///
    /// Takes a gltf primitive and builds a vertex input description for the mesh data layout the
    /// primitive uses.
    ///
    /// There is an optional parameter for passing a function that can remap attribute locations
    /// based on the current index in the attribute array, the total number of attributes and the
    /// semantic for the attribute. The function also has the ability to return a generic error
    /// string which will be surfaced back up at the `for_gltf_primitive` call site
    ///
    pub fn for_gltf_primitive<'a>(
        primitive: &crate::gltf::Primitive,
        bindings: &'a mut Vec<VertexInputBindingDescriptionBuilder<'static>>,
        attributes: &'a mut Vec<VertexInputAttributeDescriptionBuilder<'static>>,
        location_mapper: Option<&impl Fn(usize, usize, &Semantic) -> Result<u32, &'static str>>,
    ) -> Result<(), VertexInputStateError> {
        assert!(bindings.is_empty(), "List of bindings must be empty");
        assert!(attributes.is_empty(), "List of attributes must be empty");

        // Preallocate the list of bindings
        let attr_num = primitive.attributes().count();
        bindings.resize(attr_num, VertexInputBindingDescriptionBuilder::new());
        attributes.resize(attr_num, VertexInputAttributeDescriptionBuilder::new());

        for (i, (semantic, accessor)) in primitive.attributes().enumerate() {
            let location = match location_mapper {
                None => i as _,
                Some(func) => (*func)(i, attr_num, &semantic)?,
            };

            let binding = VertexInputBindingDescriptionBuilder::new()
                .input_rate(VertexInputRate::VERTEX)
                .binding(location)
                .stride(accessor.size() as _);
            bindings[location as usize] = binding;

            let attribute = VertexInputAttributeDescriptionBuilder::new()
                .format(format_from_gltf_accessor(&accessor)?)
                .offset(0)
                .location(location)
                .binding(location);

            attributes[location as usize] = attribute;
        }
        Ok(())
    }

    ///
    /// Provides an input description that matches what a standard static mesh GLTF file will
    /// provide.
    ///
    pub fn for_static_mesh<'a>(
        bindings: &'a mut Vec<VertexInputBindingDescriptionBuilder<'static>>,
        attributes: &'a mut Vec<VertexInputAttributeDescriptionBuilder<'static>>,
    ) {
        let cube = CubeMesh::gltf();
        let primitive = cube
            .document
            .meshes()
            .nth(0)
            .unwrap()
            .primitives()
            .nth(0)
            .unwrap();

        // Because this is built from a "known good" gltf file that we embed into the engine binary
        // itself we assume it's in the correct format and panic if an error is produced in the
        // call to the function below
        Self::for_gltf_primitive(
            &primitive,
            bindings,
            attributes,
            Some(&Self::static_mesh_location_mapper),
        )
        .expect("Built in cube mesh data invalid")
    }

    // Tell the inspector to ignore checking match statements are exhaustive in the match statement
    // in the function below. There is a variant of the enum behind a feature gate which be marked
    // as missing, even if the feature isn't enabled
    //noinspection RsMatchCheck
    ///
    /// A standard function provided for remapping attribute locations to match what is expected
    /// for a static mesh
    ///
    pub fn static_mesh_location_mapper(
        _i: usize,
        len: usize,
        semantic: &Semantic,
    ) -> Result<u32, &'static str> {
        if len != 4 {
            return Err("We need exactly four vertex attributes for a standard static mesh");
        }
        match semantic {
            Semantic::Positions => Ok(0),
            Semantic::Normals => Ok(1),
            Semantic::Tangents => Ok(2),
            Semantic::TexCoords(0) => Ok(3),
            Semantic::TexCoords(_) => Err("Static mesh should only have 1 set of UV attributes"),
            Semantic::Colors(_) => Err("Static mesh should not have vertex colour attribute"),
            Semantic::Joints(_) => Err("Static mesh should not have joints attribute"),
            Semantic::Weights(_) => Err("Static mesh should not have weights attribute"),
        }
    }

    ///
    /// Creates a new PipelineVertexInputStateCreateInfo from the given list of bindings and
    /// attributes
    ///
    pub fn new<'a>(
        bindings: &'a [VertexInputBindingDescriptionBuilder<'static>],
        attributes: &'a [VertexInputAttributeDescriptionBuilder<'static>],
    ) -> PipelineVertexInputStateCreateInfoBuilder<'a> {
        PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(bindings)
            .vertex_attribute_descriptions(attributes)
    }
}
