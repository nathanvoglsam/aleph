//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::embedded_data::CubeMesh;
use crate::gpu::vk::format::{format_from_gltf_accessor, AccessorFormatError};
use erupt::vk1_0::{
    PipelineVertexInputStateCreateInfoBuilder, VertexInputAttributeDescriptionBuilder,
    VertexInputBindingDescriptionBuilder, VertexInputRate,
};
use gltf::Semantic;

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
    /// Takes a gltf primitive and builds a vertex input description for the mesh data layout the
    /// primitive uses.
    ///
    /// There is an optional parameter for passing a function that can remap attribute locations
    /// based on the current index in the attribute array, the total number of attributes and the
    /// semantic for the attribute.
    ///
    pub fn for_gltf_primitive<'a>(
        primitive: &gltf::Primitive,
        bindings: &'a mut Vec<VertexInputBindingDescriptionBuilder<'static>>,
        attributes: &'a mut Vec<VertexInputAttributeDescriptionBuilder<'static>>,
        location_fn: Option<&impl Fn(usize, usize, &Semantic) -> Result<u32, &'static str>>,
    ) -> Result<(), VertexInputStateError> {
        assert!(bindings.is_empty(), "List of bindings must be empty");
        assert!(attributes.is_empty(), "List of attributes must be empty");

        let attr_num = primitive.attributes().count();

        for (i, (semantic, accessor)) in primitive.attributes().enumerate() {
            let binding = VertexInputBindingDescriptionBuilder::new()
                .input_rate(VertexInputRate::VERTEX)
                .binding(i as _)
                .stride(accessor.size() as _);
            bindings.push(binding);

            let location = match location_fn {
                None => i as _,
                Some(func) => (*func)(i, attr_num, &semantic)?,
            };
            let attribute = VertexInputAttributeDescriptionBuilder::new()
                .format(format_from_gltf_accessor(&accessor)?)
                .offset(0)
                .location(location)
                .binding(i as _);

            attributes.push(attribute)
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
            panic!("We need exactly four vertex attributes for a standard static mesh")
        }
        match semantic {
            Semantic::Positions => Ok(0),
            Semantic::Normals => Ok(1),
            Semantic::Tangents => Ok(2),
            Semantic::TexCoords(_) => Ok(3),
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
