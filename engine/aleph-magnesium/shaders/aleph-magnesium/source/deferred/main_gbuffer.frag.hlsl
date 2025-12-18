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

#include "pbr.hlsl"
#include "normal_map.hlsl"
#include "sampling.hlsl"

#include "main_gbuffer.inc.hlsl"

// Material parameters
static float reflectance = 0.5;

struct PixelOutput {
    float4 gbuffer_0: SV_Target0;
    float2 gbuffer_1: SV_Target1;
    float2 gbuffer_2: SV_Target2;
};

[shader("fragment")]
PixelOutput main(in StaticMeshPixelInput input) {
    float3 n;
	float3 t;
	float3 b;
	const float3 normal_sample = g_material.normal_map.Sample(g_view.sampler, input.uv);
	const float3 map_normal = normalize((normal_sample * 2.0) - 1.0);
	TBNNormalMapSample(
        map_normal,
        normalize(input.normal),
        input.tangent,
        n,
        t,
        b
    );

    let vtx_colour = input.colour;
    let base_colour = g_material.data.colour.xyz;
    let base_colour_tex = g_material.base_colour.Sample(g_view.sampler, input.uv).xyz;

    let metal_roughness = g_material.metal_roughness.Sample(g_view.sampler, input.uv);

    let metallic = g_material.data.metal_roughness_padding.x * metal_roughness.z;
    let roughness = RemapRoughness(g_material.data.metal_roughness_padding.y) * RemapRoughness(metal_roughness.y);

    let mapped_normal = OctahedralEncode(n);

    PixelOutput output;
    output.gbuffer_0.xyz = vtx_colour * base_colour * base_colour_tex;
    output.gbuffer_0.w = 1;
    output.gbuffer_1.xy = mapped_normal;
    output.gbuffer_2.x = metallic;
    output.gbuffer_2.y = roughness;

    return output;
}
