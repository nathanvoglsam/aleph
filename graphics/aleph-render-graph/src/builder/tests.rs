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

    let pass = ClosurePass::new(
        |accesses| {
            accesses.write("A1");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("A1");
            accesses.write("A2");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("A2");
            accesses.write("B1");
            accesses.write("C1");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("B1");
            accesses.write("D1");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("C1");
            accesses.write("E1");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("E1");
            accesses.write("E2");
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("D1");
            accesses.read("E2");
            accesses.write("F1")
        },
        |_| {},
    );
    builder.pass(pass);

    let pass = ClosurePass::new(
        |accesses| {
            accesses.read("F1");
            accesses.write("F2");
        },
        |_| {},
    );
    builder.pass(pass);

    let graph = builder.build().unwrap();
}
