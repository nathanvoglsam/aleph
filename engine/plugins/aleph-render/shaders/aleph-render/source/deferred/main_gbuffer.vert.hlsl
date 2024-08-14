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

#include "main_gbuffer.inc.hlsl"

struct Params {
    ConstantBuffer<CameraLayout> camera;
    ConstantBuffer<ModelLayout> model;
};

ParameterBlock<Params> g_params;

struct VSResult<T> {
    float4 sv_position : SV_Position;
    T payload;
};

func main(in StaticMeshVertexInput input, out float4 sv_position : SV_Position) -> StaticMeshPixelInput {
    let in_pos = float4(input.position, 1.0);
    let normal_matrix = (float3x3)g_params.model.normal_matrix;

    var position = mul(in_pos, g_params.model.model_matrix);
    let normal = normalize(mul(input.normal, normal_matrix));
    let tangent = normalize(mul(input.tangent.xyz, normal_matrix));

    StaticMeshPixelInput output;
    output.position = position.xyz;
    output.normal = normal;
    output.tangent = tangent;
    output.uv = input.uv;

    position = mul(position, g_params.camera.view_matrix);
    position = mul(position, g_params.camera.proj_matrix);
    sv_position = position;

    return output;
}
