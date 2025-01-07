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
#include "pbr.hlsl"
#include "normal_map.hlsl"

[[vk_binding(0, 0)]] ConstantBuffer<CameraLayout> g_camera : register(b0, space0);

[[vk_binding(0, 1)]] ConstantBuffer<MaterialLayout> g_mat : register(b0, space1);
[[vk_binding(1, 1)]] Texture2D<float4> g_base_colour : register(t1, space1);
[[vk_binding(2, 1)]] Texture2D<float4> g_metal_roughness : register(t2, space1);
[[vk_binding(3, 1)]] Texture2D<float3> g_normal_map : register(t3, space1);
[[vk_binding(4, 1)]] SamplerState g_sampler : register(s4, space1);

[[vk_binding(0, 2)]] ConstantBuffer<ModelLayout> g_model : register(b0, space2);

// Material parameters
static float reflectance = 0.5;

struct PixelOutput {
    float4 gbuffer_0: SV_Target0;
    float4 gbuffer_1: SV_Target1;
    float2 gbuffer_2: SV_Target2;
};

PixelOutput main(in StaticMeshPixelInput input) {
    float3 n;
	float3 t;
	float3 b;
	const float3 normal_sample = g_normal_map.Sample(g_sampler, input.uv);
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
    let base_colour = g_mat.colour.xyz;
    let base_colour_tex = g_base_colour.Sample(g_sampler, input.uv).xyz;

    let metal_roughness = g_metal_roughness.Sample(g_sampler, input.uv);

    let metallic = g_mat.metal_roughness_padding.x * metal_roughness.z;
    let roughness = RemapRoughness(g_mat.metal_roughness_padding.y) * RemapRoughness(metal_roughness.y);

    PixelOutput output;
    output.gbuffer_0.xyz = vtx_colour * base_colour * base_colour_tex;
    output.gbuffer_0.w = 1;
    output.gbuffer_1.xyz = n;
    output.gbuffer_2.x = metallic;
    output.gbuffer_2.y = roughness;

    return output;
}
