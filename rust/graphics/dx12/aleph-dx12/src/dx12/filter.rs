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

use raw::windows::win32::direct3d12::D3D12_FILTER;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Filter {
    MinMagMipPoint,
    MinMagPointMipLinear,
    MinPointMagLinearMipPoint,
    MinPointMagMipLinear,
    MinLinearMagMipPoint,
    MinLinearMagPointMipLinear,
    MinMagLinearMipPoint,
    MinMagMipLinear,
    Anisotropic,
    ComparisonMinMagMipPoint,
    ComparisonMinMagPointMipLinear,
    ComparisonMinPointMagLinearMipPoint,
    ComparisonMinPointMagMipLinear,
    ComparisonMinLinearMagMipPoint,
    ComparisonMinLinearMagPointMipLinear,
    ComparisonMinMagLinearMipPoint,
    ComparisonMinMagMipLinear,
    ComparisonAnisotropic,
    MinimumMinMagMipPoint,
    MinimumMinMagPointMipLinear,
    MinimumMinPointMagLinearMipPoint,
    MinimumMinPointMagMipLinear,
    MinimumMinLinearMagMipPoint,
    MinimumMinLinearMagPointMipLinear,
    MinimumMinMagLinearMipPoint,
    MinimumMinMagMipLinear,
    MinimumAnisotropic,
    MaximumMinMagMipPoint,
    MaximumMinMagPointMipLinear,
    MaximumMinPointMagLinearMipPoint,
    MaximumMinPointMagMipLinear,
    MaximumMinLinearMagMipPoint,
    MaximumMinLinearMagPointMipLinear,
    MaximumMinMagLinearMipPoint,
    MaximumMinMagMipLinear,
    MaximumAnisotropic,
}

impl Into<D3D12_FILTER> for Filter {
    fn into(self) -> D3D12_FILTER {
        match self {
            Filter::MinMagMipPoint => D3D12_FILTER::D3D12_FILTER_MIN_MAG_MIP_POINT,
            Filter::MinMagPointMipLinear => D3D12_FILTER::D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR,
            Filter::MinPointMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT
            }
            Filter::MinPointMagMipLinear => D3D12_FILTER::D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR,
            Filter::MinLinearMagMipPoint => D3D12_FILTER::D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT,
            Filter::MinLinearMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR
            }
            Filter::MinMagLinearMipPoint => D3D12_FILTER::D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT,
            Filter::MinMagMipLinear => D3D12_FILTER::D3D12_FILTER_MIN_MAG_MIP_LINEAR,
            Filter::Anisotropic => D3D12_FILTER::D3D12_FILTER_ANISOTROPIC,
            Filter::ComparisonMinMagMipPoint => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT
            }
            Filter::ComparisonMinMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR
            }
            Filter::ComparisonMinPointMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT
            }
            Filter::ComparisonMinPointMagMipLinear => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR
            }
            Filter::ComparisonMinLinearMagMipPoint => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT
            }
            Filter::ComparisonMinLinearMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR
            }
            Filter::ComparisonMinMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT
            }
            Filter::ComparisonMinMagMipLinear => {
                D3D12_FILTER::D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR
            }
            Filter::ComparisonAnisotropic => D3D12_FILTER::D3D12_FILTER_COMPARISON_ANISOTROPIC,
            Filter::MinimumMinMagMipPoint => D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT,
            Filter::MinimumMinMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR
            }
            Filter::MinimumMinPointMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT
            }
            Filter::MinimumMinPointMagMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR
            }
            Filter::MinimumMinLinearMagMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT
            }
            Filter::MinimumMinLinearMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR
            }
            Filter::MinimumMinMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT
            }
            Filter::MinimumMinMagMipLinear => D3D12_FILTER::D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR,
            Filter::MinimumAnisotropic => D3D12_FILTER::D3D12_FILTER_MINIMUM_ANISOTROPIC,
            Filter::MaximumMinMagMipPoint => D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT,
            Filter::MaximumMinMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR
            }
            Filter::MaximumMinPointMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT
            }
            Filter::MaximumMinPointMagMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR
            }
            Filter::MaximumMinLinearMagMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT
            }
            Filter::MaximumMinLinearMagPointMipLinear => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR
            }
            Filter::MaximumMinMagLinearMipPoint => {
                D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT
            }
            Filter::MaximumMinMagMipLinear => D3D12_FILTER::D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR,
            Filter::MaximumAnisotropic => D3D12_FILTER::D3D12_FILTER_MAXIMUM_ANISOTROPIC,
        }
    }
}
