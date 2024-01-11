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

use crate::internal::{
    FrameGraphBufferDesc, FrameGraphTextureDesc, IIRNode, IRNode, PassOrderBundle, RenderPass,
    ResourceRoot, ResourceVersion,
};
use crate::{
    FrameGraphBuilder, ImportBundle, ResourceRef, ResourceVariant, TransientResourceBundle,
};
use aleph_arena_drop_list::DropLink;
use aleph_rhi_api::*;
use bumpalo::Bump;
use std::collections::HashMap;
use std::ptr::NonNull;

pub struct FrameGraph {
    /// The bump allocation arena that provides the backing memory for the render passes and any
    /// other memory that's needed for them.
    ///
    /// This typically includes the IRenderPass objects themselves, their name strings and the
    /// payload objects for callback passes.
    ///
    /// This won't be directly used by a constructed graph but must be stored inside the graph in
    /// order to keep the allocations for all the render passes alive.
    pub(crate) _arena: Bump,

    /// Our final pass + barrier execution order that is the final output of our graph building
    /// operations. The passes and barriers are executed by iterating this list and executing the
    /// barriers and passes referenced within in order.
    pub(crate) execution_bundles: Vec<PassOrderBundle>,

    /// Backing storage for our IR nodes.
    ///
    /// This array is indexed by the [PassOrderBundle] objects in the
    /// [FrameGraph::execution_bundles] array. Each bundle contains a set of barriers and passes to
    /// execute in order, with each barrier and pass identified as an index into this array.
    pub(crate) ir_nodes: Vec<IRNode>,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    pub(crate) render_passes: Vec<RenderPass>,

    /// The backing storage used for all of the root resourcs objects. A root resource represents
    /// a concrete [ITexture] or [IBuffer] as created by the graph. This includes both the created
    /// transient resources as well as imported resources. Imported resources are identified by
    /// having their index in the 'imported_resources' set.
    pub(crate) root_resources: Vec<ResourceRoot>,

    /// The backing storage used for all the resource version objects. A resource version is an
    /// indexed set that is used to identify a particular version of a root resource.
    ///
    /// A 'ResourceVersion' contains the accumulated usages of the resource as well as a link to
    /// the previous version to form a linked-list of resource versions back to the first version
    /// when the resource was created or imported.
    ///
    /// These entries are critical as resource versions are what form the core of the graph. They
    /// are what allows the graph to construct a stable program order via an SSA form graph
    /// construction.
    pub(crate) resource_versions: Vec<ResourceVersion>,

    /// The set of resources within the graph that were imported, stored as indices into the
    /// root_resources array.
    pub(crate) imported_resources: Vec<u16>,

    /// The head of the dropper linked-list that contains all the drop functions for objects
    /// allocated from the graph arena
    pub(crate) drop_head: Option<NonNull<DropLink>>,
}

impl FrameGraph {
    pub fn builder() -> FrameGraphBuilder {
        FrameGraphBuilder::new()
    }

    pub unsafe fn execute(
        &mut self,
        transient_bundle: &TransientResourceBundle,
        import_bundle: &ImportBundle,
        encoder: &mut dyn IGeneralEncoder,
    ) {
        // TODO: parallel encode
        //
        // We could record in parallel by allocating command buffers ourselves and doing a parallel
        // iteration over the execution bundles list.

        self.execute_pre_assertions(transient_bundle, import_bundle);

        let resources = FrameGraphResources {
            import_bundle,
            transient_bundle,
        };

        for bundle in self.execution_bundles.iter() {
            let mut has_memory_barrier = false;
            let mut memory_barrier = GlobalBarrier {
                before_sync: BarrierSync::NONE,
                after_sync: BarrierSync::NONE,
                before_access: BarrierAccess::NONE,
                after_access: BarrierAccess::NONE,
            };

            let mut texture_barriers = Vec::new();
            let barriers = unsafe { bundle.barriers.as_ref() };
            for &barrier in barriers {
                let node = &self.ir_nodes[barrier];
                match node {
                    IRNode::RenderPass(_) => unreachable!(),
                    IRNode::Barrier(v) => {
                        memory_barrier.before_sync |= v.before_sync;
                        memory_barrier.before_access |= v.before_access;
                        memory_barrier.after_sync |= v.after_sync;
                        memory_barrier.after_access |= v.after_access;
                        has_memory_barrier = true;
                    }
                    IRNode::LayoutChange(v) => {
                        let root_id = v.resource_id.root;
                        let texture = transient_bundle
                            .get_resource(root_id)
                            .or_else(|| import_bundle.get_resource(root_id))
                            .map(|v| v.unwrap_texture())
                            .unwrap();
                        texture_barriers.push(TextureBarrier {
                            texture,
                            subresource_range: v.subresource_range.clone(),
                            before_sync: v.before_sync,
                            after_sync: v.after_sync,
                            before_access: v.before_access,
                            after_access: v.after_access,
                            before_layout: v.before_layout,
                            after_layout: v.after_layout,
                            queue_transition: None,
                        });
                    }
                }
            }

            if !barriers.is_empty() {
                let memory_barrier = if has_memory_barrier {
                    std::slice::from_ref(&memory_barrier)
                } else {
                    &[]
                };
                encoder.resource_barrier(&memory_barrier, &[], &texture_barriers);
            }

            let passes = unsafe { bundle.passes.as_ref() };
            for &pass in passes {
                let node = &self.ir_nodes[pass];
                debug_assert!(node.is_render_pass());

                let render_pass = node.render_pass();
                let render_pass = &mut self.render_passes[render_pass];
                unsafe { render_pass.pass.as_mut().execute(encoder, &resources) }
            }
        }
    }

    pub fn allocate_transient_resource_bundle(
        &self,
        device: &dyn IDevice,
    ) -> TransientResourceBundle {
        let num_transients = self.root_resources.len() - self.imported_resources.len();
        let mut bundle = TransientResourceBundle {
            transients: HashMap::with_capacity(num_transients),
        };
        for (i, transient) in self
            .root_resources
            .iter()
            .enumerate()
            .filter(|(_, v)| !v.resource_type.is_import())
        {
            let i = u16::try_from(i).unwrap();
            match &transient.resource_type {
                crate::internal::ResourceType::Buffer(v) => {
                    let desc = BufferDesc {
                        size: v.desc.size,
                        cpu_access: v.desc.cpu_access,
                        usage: transient.total_access_flags,
                        name: v.desc.name.map(|v| unsafe { v.as_ref() }),
                    };
                    let buffer = device.create_buffer(&desc).unwrap();
                    bundle.add_resource(i, buffer);
                }
                crate::internal::ResourceType::Texture(v) => {
                    let desc = TextureDesc {
                        width: v.desc.width,
                        height: v.desc.height,
                        depth: v.desc.depth,
                        format: v.desc.format,
                        dimension: v.desc.dimension,
                        clear_value: v.desc.clear_value.clone(),
                        array_size: v.desc.array_size,
                        mip_levels: v.desc.mip_levels,
                        sample_count: v.desc.sample_count,
                        sample_quality: v.desc.sample_quality,
                        usage: transient.total_access_flags,
                        name: v.desc.name.map(|v| unsafe { v.as_ref() }),
                    };
                    let texture = device.create_texture(&desc).unwrap();
                    bundle.add_resource(i, texture);
                }
            }
        }

        bundle
    }

    #[rustfmt::skip]
    pub fn graph_viz_for_pass_order(
        &self,
        graph_name: &str,
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writeln!(writer, "digraph {graph_name} {{")?;
        writeln!(writer, "    compound=true;")?;

        for (i, bundle) in self.execution_bundles.iter().enumerate() {
            writeln!(writer, "    subgraph cluster{i} {{")?;
            writeln!(writer, "        label=\"Bundle {i}\";")?;
            writeln!(writer, "        fontsize=30;")?;
            writeln!(writer, "        cluster{i}entrynode [label=\"\",style=invis,width=0.01,height=0.01];")?;
            writeln!(writer, "        subgraph cluster{i}barriers {{")?;
            writeln!(writer, "            label=\"Barriers\";")?;
            writeln!(writer, "            fontsize=20;")?;
            writeln!(writer, "            style=filled;")?;
            let barriers = unsafe { bundle.barriers.as_ref() };
            for &barrier_index in barriers {
                let node = &self.ir_nodes[barrier_index];
                let target_id = node.resource_id();
                let target_root = self.resource_versions[target_id.version as usize].root_resource;
                let target_root = &self.root_resources[target_root as usize];
                let target_name = unsafe {
                    target_root
                        .resource_type
                        .name()
                        .unwrap_or("Unnamed Resource")
                };
                write!(writer, "            ")?;
                node.write_graph_viz(writer, target_name, barrier_index)?;
            }
            writeln!(writer, "            cluster{i}barriernode [label=\"\",style=invis,width=0.01,height=0.01];")?;
            writeln!(writer, "        }}")?;
            writeln!(writer, "        subgraph cluster{i}passes {{")?;
            writeln!(writer, "            label=\"Passes\";")?;
            writeln!(writer, "            fontsize=20;")?;
            writeln!(writer, "            style=filled;")?;
            let passes = unsafe { bundle.passes.as_ref() };
            for &pass_index in passes {
                let node = &self.ir_nodes[pass_index];
                let pass = &self.render_passes[node.render_pass()];
                let pass_name = unsafe { pass.name.as_ref() };
                write!(writer, "            ")?;
                node.write_graph_viz(writer, pass_name, pass_index)?;

                let prevs = unsafe { node.prev().as_ref() };
                for &prev in prevs {
                    if barriers.contains(&prev) {
                        writeln!(writer, "            node{prev} -> node{pass_index};")?;
                    }
                }
            }
            writeln!(writer, "            cluster{i}passesnode [label=\"\",style=invis,width=0.01,height=0.01];")?;
            writeln!(writer, "        }}")?;
            writeln!(writer, "        cluster{i}barriernode -> cluster{i}passesnode [style=invis];")?;
            writeln!(writer, "        cluster{i}exitnode [label=\"\",style=invis,width=0.01,height=0.01];")?;
            writeln!(writer, "        cluster{i}entrynode -> cluster{i}barriernode [style=invis];")?;
            writeln!(writer, "        cluster{i}passesnode -> cluster{i}exitnode [style=invis];")?;
            writeln!(writer, "    }}")?;

            if i > 0 {
                let prev = i - 1;
                writeln!(writer, "    cluster{prev}exitnode -> cluster{i}entrynode [ltail=cluster{prev},lhead=cluster{i},minlen=2];")?;
            }
        }

        writeln!(writer, "}}")?;

        Ok(())
    }
}

impl FrameGraph {
    /// Internal function that implements the debug assertions that run prior to the main execute
    /// pass in the frame graph.
    unsafe fn execute_pre_assertions(
        &mut self,
        transient_bundle: &TransientResourceBundle,
        import_bundle: &ImportBundle,
    ) {
        if cfg!(debug_assertions) {
            for v in self.imported_resources.iter() {
                let root_resource = &self.root_resources[*v as usize];
                let imported_resource = import_bundle.imports.get(v);
                let transient_resource = transient_bundle.transients.get(v);

                let name = root_resource
                    .resource_type
                    .name()
                    .unwrap_or("Unnamed resource");

                let is_import = root_resource.resource_type.is_import();
                assert!(
                    is_import,
                    "INTERNAL ERROR: Resource '{}' import status internal mismatch",
                    name
                );

                let has_import = imported_resource.is_some();
                assert!(
                    has_import,
                    "The ImportBundle does not contain handle for imported graph resource '{}'",
                    name
                );

                let no_transient_for_import = transient_resource.is_none();
                assert!(
                    no_transient_for_import,
                    "The TransientResourceBundle contains a resource for imported resource '{}'",
                    name
                );

                let imported_resource = imported_resource.unwrap();
                self.validate_import_or_transient_desc(
                    root_resource,
                    imported_resource,
                    "Imported",
                    name,
                );
            }

            for (v, root_resource) in self.root_resources.iter().enumerate() {
                let is_import = root_resource.resource_type.is_import();
                if is_import {
                    // We've already processed the imported resources, skip any imported resources
                    // we find.
                    continue;
                }

                let v = u16::try_from(v).unwrap();
                let imported_resource = import_bundle.imports.get(&v);
                let transient_resource = transient_bundle.transients.get(&v);

                let name = root_resource
                    .resource_type
                    .name()
                    .unwrap_or("Unnamed resource");

                let has_transient = transient_resource.is_some();
                assert!(
                    has_transient,
                    "The TransientResourceBundle does not contain handle for graph resource '{}'",
                    name
                );

                let no_import_for_transient = imported_resource.is_none();
                assert!(
                    no_import_for_transient,
                    "The ImportBundle contains a resource for transient resource '{}'",
                    name
                );

                let transient_resource = transient_resource.unwrap();
                self.validate_import_or_transient_desc(
                    root_resource,
                    transient_resource,
                    "Transient",
                    name,
                );
            }
        }
    }

    fn validate_import_or_transient_desc(
        &self,
        root: &ResourceRoot,
        given: &ResourceVariant,
        resource_type: &str,
        name: &str,
    ) {
        match given {
            crate::ResourceVariant::Buffer(i) => match &root.resource_type {
                crate::internal::ResourceType::Buffer(r) => {
                    let i_desc = i.desc();
                    self.assert_matching_buffer_desc(&r.desc, &i_desc, name);
                }
                crate::internal::ResourceType::Texture(_r) => {
                    panic!("{} buffer '{}' was provided a texture", resource_type, name)
                }
            },
            crate::ResourceVariant::Texture(i) => match &root.resource_type {
                crate::internal::ResourceType::Buffer(_r) => {
                    panic!("{} texture '{}' was provided a buffer", resource_type, name)
                }
                crate::internal::ResourceType::Texture(r) => {
                    let i_desc = i.desc();
                    self.assert_matching_texture_desc(&r.desc, &i_desc, name);
                }
            },
        }
    }

    fn assert_matching_buffer_desc(&self, l: &FrameGraphBufferDesc, r: &BufferDesc, name: &str) {
        assert_eq!(l.size, r.size, "Buffer '{}' not expected size", name);
        assert_eq!(
            l.cpu_access, r.cpu_access,
            "Buffer '{}' not expected cpu_access",
            name
        );
    }

    fn assert_matching_texture_desc(&self, l: &FrameGraphTextureDesc, r: &TextureDesc, name: &str) {
        assert_eq!(l.width, r.width, "Texture '{}' not expected width", name);
        assert_eq!(l.height, r.height, "Texture '{}' not expected height", name);
        assert_eq!(l.depth, r.depth, "Texture '{}' not expected depth", name);
        assert_eq!(l.format, r.format, "Texture '{}' not expected format", name);
        assert_eq!(
            l.dimension, r.dimension,
            "Texture '{}' not expected dimension",
            name
        );
        assert_eq!(
            l.clear_value, r.clear_value,
            "Texture '{}' not expected clear_value",
            name
        );
        assert_eq!(
            l.array_size, r.array_size,
            "Texture '{}' not expected array_size",
            name
        );
        assert_eq!(
            l.mip_levels, r.mip_levels,
            "Texture '{}' not expected mip_levels",
            name
        );
        assert_eq!(
            l.sample_count, r.sample_count,
            "Texture '{}' not expected sample_count",
            name
        );
        assert_eq!(
            l.sample_quality, r.sample_quality,
            "Texture '{}' not expected sample_quality",
            name
        );
    }
}

impl Drop for FrameGraph {
    fn drop(&mut self) {
        // Safety: implementation and API guarantees that dropper only gets called once per
        //         object, and always on the correct type.
        unsafe {
            DropLink::drop_and_null(&mut self.drop_head);
        }
    }
}

pub struct FrameGraphResources<'a> {
    import_bundle: &'a ImportBundle,
    transient_bundle: &'a TransientResourceBundle,
}

impl<'a> FrameGraphResources<'a> {
    pub fn get<T: Into<ResourceRef>>(&self, r: T) -> Option<&ResourceVariant> {
        let r: ResourceRef = r.into();
        let i = r.0.root_id();

        let resource = self
            .transient_bundle
            .get_resource(i)
            .or_else(|| self.import_bundle.get_resource(i));

        resource
    }

    pub fn get_buffer<T: Into<ResourceRef>>(&self, r: T) -> Option<&dyn IBuffer> {
        let r = self.get(r);
        r.map(|v| v.unwrap_buffer())
    }

    pub fn get_texture<T: Into<ResourceRef>>(&self, r: T) -> Option<&dyn ITexture> {
        let r = self.get(r);
        r.map(|v| v.unwrap_texture())
    }
}
