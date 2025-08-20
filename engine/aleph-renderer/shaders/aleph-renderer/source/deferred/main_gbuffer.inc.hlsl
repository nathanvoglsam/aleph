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
    [[vk::location(0)]] float3 position  : A0;
    [[vk::location(1)]] float2 uv        : A1;
    [[vk::location(2)]] float3 normal    : A2;
    [[vk::location(3)]] float4 tangent   : A3;
    [[vk::location(4)]] float3 colour    : A4;
};

/*
 * The standard vertex layout that a static mesh shader will use
 */
struct StaticMeshVertexInput {
    [[vk::location(0)]] float3 position : A0;
    [[vk::location(1)]] float2 uv       : A1;
    [[vk::location(2)]] float3 normal   : A2;
    [[vk::location(3)]] float4 tangent  : A3;
    [[vk::location(4)]] float3 colour   : A4;
};

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float4 position;
};

struct MaterialLayout {
    float4 colour;
    float4 metal_roughness_padding;
};

struct ModelLayout {
    float4x4 model_matrix;
    float4x4 normal_matrix;
};

struct ViewParams {
    ConstantBuffer<CameraLayout> camera;
    SamplerState sampler;
};

struct MaterialParams {
    ConstantBuffer<MaterialLayout> data;
    Texture2D<float4> base_colour;
    Texture2D<float4> metal_roughness;
    Texture2D<float3> normal_map;
};

struct ModelParams {
    ConstantBuffer<ModelLayout> data;
};

ParameterBlock<ViewParams> g_view;
ParameterBlock<MaterialParams> g_material;
ParameterBlock<ModelParams> g_model;
