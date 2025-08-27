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

use aleph_rhi_api::*;

/// Utility iterator/visitor pattern that handles the parameter block layout iteration that all
/// implementations of [`IDevice::update_parameter_block`] would need to implement themselves.
///
/// This implements the common parts shared by all backends on once place.
pub struct ParameterBlockLayoutVisitor<'a> {
    desc: &'a ParameterBlockDesc<'a>,
    writes: &'a [ParameterWrite<'a>],
    index: u64,
    current_binding: u64,
    current_element: u64,
}

impl<'a> ParameterBlockLayoutVisitor<'a> {
    /// Constructs a new visitor over the given [`ParameterBlockDesc`], starting at the 'base'th
    /// parameter in the flattened list of parameters as described by the given desc.
    #[inline]
    pub fn new(
        desc: &'a ParameterBlockDesc<'a>,
        mut base: u64,
        writes: &'a [ParameterWrite<'a>],
    ) -> Option<Self> {
        let mut total_parameters = 0;
        for binding in desc.params {
            total_parameters += binding.array_size.count();
        }

        // If base is trying to index outside of the parameter block layout
        if base > total_parameters {
            return None;
        }

        // This is the number of parameters that will be visited by this utility.
        let remaining_parameters = total_parameters - base;

        // If the caller has asked to visit more parameters than are remaining after 'base' then
        // we also return 'None' as there are not enough parameters to serve the request.
        if writes.len() as u64 > remaining_parameters {
            return None;
        }

        // Find the binding and array element where the user has asked to start writing
        // parameters from.
        //
        // Once this loop is complete we should now have a binding/element derived from 'base'.
        let mut base_binding = 0;
        let mut base_element = 0;
        for binding in desc.params {
            if base > base_binding {
                base -= binding.array_size.count();
            } else {
                base_element = base;
                break;
            }
            base_binding += 1;
        }

        Some(Self {
            desc,
            writes,
            index: base,
            current_binding: base_binding,
            current_element: base_element,
        })
    }
}

impl<'a> Iterator for ParameterBlockLayoutVisitor<'a> {
    type Item = ParameterBlockLayoutVisitorElement<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Once we've consumed all the parameters
        if self.writes.is_empty() {
            return None;
        }

        let current_param = &self.desc.params[self.current_binding as usize];
        let parameter_type = current_param.ty;

        // We want to consume exactly the number of array elements remaining in the current binding
        // after 'current_element', clamped to 'remaining_parameters'. This deduces that count
        let consume_count = current_param.array_size.count() - self.current_element;
        let consume_count = consume_count.min(self.writes.len() as u64);

        let (consume_writes, remaining_writes) = self.writes.split_at(consume_count as usize);

        let out = ParameterBlockLayoutVisitorElement {
            ty: parameter_type,
            binding: self.current_binding,
            element: self.current_element,
            index: self.index,
            writes: consume_writes,
        };

        // Consume the calculated number of parameters and move to the next binding ready for the
        // next call to 'next'.
        self.writes = remaining_writes;
        self.index += consume_count;
        self.current_binding += 1;
        self.current_element = 0;

        Some(out)
    }
}

pub struct ParameterBlockLayoutVisitorElement<'a> {
    pub ty: ParameterType,
    pub binding: u64,
    pub element: u64,
    pub index: u64,
    pub writes: &'a [ParameterWrite<'a>],
}
