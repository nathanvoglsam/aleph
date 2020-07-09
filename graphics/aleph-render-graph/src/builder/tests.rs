//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{ClosurePass, RenderGraphBuilder};

#[test]
fn graph_construct_1() {
    let mut builder = RenderGraphBuilder::new();

    let pass = ClosurePass::new(|accesses| {}, || {}, || {});
    builder.pass(pass);

    let graph = builder.build().unwrap();
}
