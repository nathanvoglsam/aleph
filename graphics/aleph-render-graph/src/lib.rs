//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

//!
//! A render graph implementation with an SSA form based resource access model. By using an SSA
//! graph, resource lifetimes become immensely simpler to deduce while also allowing the library
//! user to specify explicit execution orderings rather than implicitly deriving them from pass
//! submission order.
//!
//! Using a more traditional resource model where everything has read/write access is problematic
//! as much effort needs to be put into reducing the list of passes into a dependency graph to
//! properly synchronize access. Under such a model, the resource access declarations would look
//! like so:
//!
//! `PassA; reads: ; writes: A`
//! `PassB; reads: A; writes: A`
//! `PassC; reads: A; writes: A, B`
//! `PassD; reads: A; writes: C`
//! `PassE; reads: B; writes: D`
//! `PassF; reads: C, D; writes: E`
//! `Submit Order: PassA, PassB, PassC, PassD, PassE, PassF`
//!
//! Under this model all resources are always R/W and dependencies must be reconstructed based on
//! submission order. This means work must be done to deduce dependencies, and the only real way to
//! actually resolve them is to base the graph on submission order as there is no other information
//! available to find which pass depends on another in the above form.
//!
//! By specifying graph passes in SSA form (static single assignment), a technique devised primarily
//! for use in low level compiler internals for machine code generation (LLVM IR is in SSA form),
//! the problems of the above representation disappear.
//!
//! SSA form requires that each variable is written to exactly once, and then can be read from any
//! number of times after creation from it's initial write. This immensely simplifies lifetime
//! analysis as it can be easily resolved by finding the span from the initial write to the last
//! place it was read from.
//!
//! If a resource needs to be written to multiple times, the resource must be written with a new
//! identifier. This means the above list of passes would now be defined as follows:
//!
//! `PassA; reads: ; writes: A1`
//! `PassB; reads: A1; writes: A2`
//! `PassC; reads: A2; writes: A3, B1`
//! `PassD; reads: A3; writes: C1`
//! `PassE; reads: B1; writes: D1`
//! `PassF; reads: C1, D1; writes: E1`
//! `Submit Order: PassA, PassB, PassC, PassD, PassE, PassF`
//!
//! With the new representation it can be seen that each time A is written a new name is given for
//! the result and any passes that depend on the new value of A just specify they access the new
//! name. This simplifies building the dependency graph. In fact, SSA form directly represents
//! the graph itself. This also means that the graph's layout can be described regardless of pass
//! submission order.
//!
//! This is not without compromise unfortunately. Consuming graph resources is a little more
//! cumbersome to work with due to needing to use a new name for each resource transformation
//! (write). This can make injecting new passes into existing parts of the graph more difficult as
//! the identifiers need to be updated in the directly dependent nodes.
//!
//! This could be solved by providing a "compiler" of sorts that consumes the more traditional
//! style of resource access pattern and resolves the SSA graph. This just gets us back to square
//! one (though square one is more convenient for some cases) but at the very least the cost of
//! building the SSA form is made explicit and can be skipped.
//!
//! Whether an interface for building an SSA form is provided instead of just having the user
//! provide SSA form directly is provided is un-decided at this point but the possibility is there
//! regardless.
//!

mod builder;
mod graph;
mod resource;
mod utils;

pub use builder::BufferExport;
pub use builder::BufferImport;
pub use builder::BufferReadDescription;
pub use builder::BufferWriteDescription;
pub use builder::GraphBuildError;
pub use builder::ImageExport;
pub use builder::ImageImport;
pub use builder::ImageReadDescription;
pub use builder::ImageWriteDescription;
pub use builder::RenderGraphBuilder;
pub use builder::ResourceAccess;
pub use builder::ResourceExport;
pub use builder::ResourceImport;

pub use graph::ClosurePass;
pub use graph::RenderGraph;
pub use graph::RenderGraphPass;
