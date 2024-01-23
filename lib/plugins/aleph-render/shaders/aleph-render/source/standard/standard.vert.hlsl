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

#include "standard.inc.hlsl"

[[vk::binding(0, 0)]]
ConstantBuffer<CameraLayout> camera_buffer : register(b0);

[[vk::binding(1, 0)]]
ConstantBuffer<ModelLayout> model_buffer : register(b1);

StaticMeshPixelInput main(in StaticMeshVertexInput input, out float4 out_position : SV_POSITION) {
    // Load buffers so auto complete works properly
    const CameraLayout camera = camera_buffer;
    const ModelLayout model = model_buffer;
	
    StaticMeshPixelInput output;

    const float4 in_pos = float4(input.position, 1.0);
    const float3x3 normal_matrix = (float3x3)model.normal_matrix;
    
    float4 position = mul(in_pos, model.model_matrix);
    float3 normal = normalize(mul(input.normal, normal_matrix));
    float3 tangent = normalize(mul(input.tangent.xyz, normal_matrix));
    output.position = position.xyz;
    output.normal = normal;
    output.tangent = float4(tangent, input.tangent.w);
    output.uv = input.uv;

    position = mul(position, camera.view_matrix);
    position = mul(position, camera.proj_matrix);
    out_position = position;

    return output;
}