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

mod frame_graph;
mod pin_board;
mod render_pass;
mod resource;
mod resource_registry;
mod utils;

pub use frame_graph::{FrameGraph, FrameGraphBuilder};
pub use pin_board::PinBoard;
pub use render_pass::IRenderPass;
pub use resource::{ResourceMut, ResourceRef};
pub use resource_registry::ResourceRegistry;

/// Two types of resources as far as the graph is concerned
/// - Imported
/// - Transient
///
/// Exported resources are simply a special case of imported resources where the resource is used
/// after the graph is completed, with whatever state it's found in as the 'exported' result.
///
/// These can be considered as the base or root for the resources presented to frame graph users.
/// A resource handle in the frame graph corresponds to a combination of the root resource, and
/// whatever state the resource is trying to be read in. What this means is that a handle is both
/// a resource and 'version'.
///
/// What this effectively means is that the resource dependencies encode an SSA graph. Whenever a
/// write is declared a new handle is created that renames the root resource by updating the
/// version.
///
/// The key problems we need to solve are:
///
/// - When do we generate new root resources?
/// - How do we encode graph edges?
/// - How do we then weave a command stream from the passes?
/// - How do we handle read-after-read barriers that require different resource states? The barrier
///   effectively becomes a write as the layout change requires exclusive access. What does this
///   mean for `ResourceRef`? Do read edges get promoted to write edges?
///
/// # Ideas
///
/// Resource Creation Rules:
/// - Any 'import resource' or 'create resource' creates a new root resource.
/// - Any 'read resource' simply records a read.
///     - We have no choice but to patch up read-to-read layout transitions at the end
/// - Any 'write resource' records the write, creates a new version of the resource
///
/// Graph Creation:
/// - There are types of nodes beyond just the render passes themselves. Resource transitions are
///   similar to write operations, resource export may require resource transitions, potentially
///   aliasing transitions.
/// - The graph should not have passes as nodes, render passes should just be one type of node.
///   Resource transitions should themselves be nodes in the graph, allowing them to be optimized
///   with other graph transformations.
/// - How do we deal with ordering reads with regard to read-to-read layout transitions? I think the
///   only tractable solution is to just order based on pass submission order. In practice almost
///   every images is going to be in one of five layouts: sampled image, render target, unordered
///   access, copy dest and copy src. Almost any sane transition in this set is already a read
///   -> write or write -> read transition anyway. CopySrc -> Sampled Image is possible, but I would
///   expect to be rare. This problem is likely not that important, submission order will do.
///
/// # Further
///
/// Clearly we need a multi-phased graph construction algorithm. We get a program order from the
/// SSA edges formed by versioning resources through write access, but we have to resolve read->read
/// barriers and insert graph edges where they're needed. We won't worry about alias analysis in
/// the initial implementation as that is it's own can of worms.
///
/// This seems relatively straight forward. Setup phase has passes declare read/write/creates and
/// we just record them. Each read/write declares their required synchronization. Importantly
/// resource creates only declares immediate usage. We need to take the union of all uses within a
/// graph for transient resources as we need to know all the ways a resource is used in order to
/// create them. We absolutely _must_ allow linting these as some combinations of resource usage
/// flags have performance implications (mobile will wring your neck with de-optimizations if you
/// try to do anything other than render target for render targets). These can just be user
/// callbacks given during the build phase.
///
/// Once we have all our passes declared we should then build an explicit graph from the declared
/// edges. Our first step is to build a graph of the edges naively, as declared. This is to give us
/// an easy to walk graph instead of a flat list of nodes with implicit edges. Next we need to split
/// read -> read barriers into their dependency chains.
///
/// read -> read barriers are easy to identify, simply find all usages of a resource and find when
/// some are used with a different image layout. We should try and bundle access under the same
/// layout together and minimize the number of layout transitions. Write -> other edges will always
/// require barriers.
///
/// There may yet be more graph passes we need to get a fully valid graph.
///
/// Once we have a valid graph we need to flatten the graph into command lists, which is easier said
/// than done. The immediate idea is to walk the graph from root to leaves and try and schedule
/// passes to maximize the amount of work issued to the GPU between pipeline barriers to keep the
/// GPU fed.
///
/// Once we have a flattened pass order we have the option to try and split it up and record
/// commands in parallel. How to do this is not obvious to me currently.
///
/// Another open question: Vulkan render passes. Our RHI lacks a representation of subpasses. Do we
/// want them, and do we want them in the frame graph. My experience is _yes_ as we absolutely want
/// to be able to merge passes, but this will only really help on mobile (a platform I'm not really
/// going to ship on). I would say _no_ for now. I just don't have the time to do it all.
///
/// And again: resource aliasing. Transient resources have their entire lifetime scoped to the
/// frame graph, every state they will ever be in is known. We have all the information to do alias
/// resources across time/space. We can allocate resources out of the same memory if their lifetime
/// is disjoint in time. Figuring this out automatically is a lifetime analysis problem. This does
/// introduce a conflict though, the optimal pass ordering and barrier order for compute purposes
/// will keep resources live for longer and keep our memory high water mark high. What do we do when
/// compute speed and memory working set is in conflict? Who should take priority? Do we optimize
/// for maximizing aliasing? I suspect the optimal answer lies somewhere around 'prioritize compute
/// throughput but take easy aliasing wins when they're cheap'.
///
/// This is 100% just a stream of thinking and not a concrete plan. I need some _deep_ thought time
/// on this to work out a concrete implementation.
