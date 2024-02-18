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

#pragma once

/*
 * Standard fragment payload for passing interpolated params to fragment shader for static mesh
 */
struct StaticMeshPixelInput {
    float3 position  : A0;
    float2 uv        : A1;
    float3 normal    : A2;
    float3 tangent   : A3;
};

/*
 * The standard vertex layout that a static mesh shader will use
 */
struct StaticMeshVertexInput {
    float3 position : A0;
    float2 uv       : A1;
    float3 normal   : A2;
    float3 tangent  : A3;
};

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float3 position;
};

struct ModelLayout {
    float4x4 model_matrix;
    float4x4 normal_matrix;
};
