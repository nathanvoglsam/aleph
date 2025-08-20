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

use std::collections::VecDeque;

use crate::shader_system::reflection::Node;

pub fn flatten_parameter_tree(nodes: &[Node]) -> Vec<aleph_shader_db::ParameterBlockDesc> {
    let mut blocks = Vec::new();

    // Fill the queue with the top level parameter blocks first
    let mut queue = VecDeque::new();
    for node in nodes {
        match node {
            v @ Node::ParameterBlock { .. } => queue.push_back(v),
            Node::Parameter { .. } => unreachable!(),
            Node::PushConstantBlock { .. } => {}
        }
    }

    // Breadth-first traversal to flatten the parameter blocks out into the final order. This is
    // what assigns the set indices/register spaces.
    while let Some(node) = queue.pop_front() {
        match node {
            v @ Node::ParameterBlock { children } => {
                blocks.push(v);
                for child in children {
                    queue.push_back(child);
                }
            }
            Node::Parameter { .. } => {}
            Node::PushConstantBlock { .. } => {}
        }
    }

    // Trim our flattened blocks to remove the nested parameter block entries
    let mut final_blocks = Vec::new();

    for block in blocks {
        let mut desc = aleph_shader_db::ParameterBlockDesc { parameters: vec![] };
        walk_node_tree(&mut desc, block);
        final_blocks.push(desc);
    }

    final_blocks
}

fn walk_node_tree(desc: &mut aleph_shader_db::ParameterBlockDesc, node: &Node) {
    match node {
        Node::ParameterBlock { children } => {
            for child in children {
                walk_node_tree(desc, child);
            }
        }
        Node::Parameter {
            parameter_type,
            array_size,
        } => desc.parameters.push(aleph_shader_db::ParameterDesc {
            ty: *parameter_type,
            array_size: *array_size,
        }),
        Node::PushConstantBlock { .. } => unreachable!(),
    }
}
