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

[shader("vertex")]
func main(
    in StaticMeshVertexInput input,
    out float4 sv_position : SV_Position
) -> StaticMeshPixelInput {
    let in_pos = float4(input.position, 1.0);
    let normal_matrix = (float3x3)g_model.data.normal_matrix;

    var position = mul(g_model.data.model_matrix, in_pos);
    let normal = normalize(mul(normal_matrix, input.normal));
    let tangent = normalize(mul(normal_matrix, input.tangent.xyz));

    StaticMeshPixelInput output;
    output.position = position.xyz;
    output.normal = normal;
    output.tangent = float4(tangent, input.tangent.w);
    output.uv = input.uv;
    output.colour = input.colour;

    position = mul(g_view.camera.view_matrix, position);
    position = mul(g_view.camera.proj_matrix, position);
    sv_position = position;

    return output;
}
