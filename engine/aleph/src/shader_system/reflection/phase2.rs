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

use crate::shader_system::reflection::{Diagnostic, Node};

pub fn find_push_constant_block(
    diagnostics: &mut Vec<Diagnostic>,
    nodes: &mut Vec<Node>,
) -> Option<u64> {
    // First confirm there are no nested push constant blocks inside the parameter blocks. This
    // should not be possible, but we want to be certain
    for node in nodes.iter() {
        match node {
            v @ Node::ParameterBlock { .. } => {
                if contains_push_constant_block(v) {
                    unreachable!("push constant in param block: phase1 should catch this!");
                }
            }
            Node::Parameter { .. } => {}
            Node::PushConstantBlock { .. } => {}
        }
    }

    // Extract all push constant blocks from 'nodes' and build a set of all the push constant blocks
    // that were found at the root level of the binding set.
    //
    // 'nodes' should now only contain top level parameter blocks
    let mut push_constant_blocks = Vec::new();
    push_constant_blocks.extend(nodes.iter().filter_map(|v| match v {
        Node::PushConstantBlock { bytes } => Some(*bytes),
        _ => None,
    }));
    nodes.retain(|v| match v {
        Node::ParameterBlock { .. } => true,
        Node::Parameter { .. } => false,
        Node::PushConstantBlock { .. } => false,
    });

    // If there's more than a single push constant block then we must emit an error.
    if push_constant_blocks.len() > 1 {
        diagnostics.push(Diagnostic::TooManyPushConstantBlocks);
    }

    push_constant_blocks.first().copied()
}

fn contains_push_constant_block(node: &Node) -> bool {
    match node {
        Node::ParameterBlock { children } => {
            for child in children {
                if contains_push_constant_block(child) {
                    return true;
                }
            }
            false
        }
        Node::Parameter { .. } => false,
        Node::PushConstantBlock { .. } => true,
    }
}
