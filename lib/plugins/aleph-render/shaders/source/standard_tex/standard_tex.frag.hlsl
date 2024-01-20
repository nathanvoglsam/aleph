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

#include "normal_map.hlsl"
#include "pbr.hlsl"
#include "standard_tex.inc.hlsl"

[[vk::binding(0, 0)]]
ConstantBuffer<CameraLayout> camera_buffer: register(b0);

[[vk::binding(1, 0)]]
Texture2D<float3> BaseColourTex : register(t1);
[[vk::binding(2, 0)]]
SamplerState BaseColourSampler : register(s2);

[[vk::binding(3, 0)]]
Texture2D<float3> NormalTex : register(t3);
[[vk::binding(4, 0)]]
SamplerState NormalSampler : register(s4);

float4 main(in StaticMeshPixelInput input) : SV_Target0 {
	// Load buffers so auto complete works properly
    const CameraLayout camera = camera_buffer;
	
    // Light parameters
    const float3 light_position = float3(0,0,0);
    const float lumens = 5000;

    // Camera and light vectors
    const float3 camera_to_frag = camera.position - input.position;
    const float3 light_to_frag = input.position - light_position;

    // Material parameters
    const float3 base_colour = BaseColourTex.Sample(BaseColourSampler, input.uv);
    const float metallic = 0.0;
    const float roughness = RemapRoughness(0.8);
    const float reflectance = 0.5;
    
    // Derive TBN and sample the normal map
	float3 n;
	float3 t;
	float3 b;
	const float3 normal_sample = NormalTex.Sample(NormalSampler, input.uv);
	const float3 map_normal = normalize((normal_sample * 2.0) - 1.0);
	TBNNormalMapSample(map_normal, input.normal, input.tangent, n, t, b);
    
    // Derived material parameters
	const float3 v = normalize(camera_to_frag);
	const float3 l = normalize(light_to_frag);
	const float3 f0 = CalculateF0(base_colour, metallic, reflectance);

    // Calculate the result of our BRDF
    const float3 brdf = StandardBRDF(v, l, n, base_colour, metallic, roughness, f0);

    // Apply a single point light
    const float NoL = clamp(dot(n, normalize(light_to_frag)), 0.0, 1.0);
    const float distance_squared = dot(light_to_frag, light_to_frag);
    const float3 final = EvaluatePointLight(brdf, lumens, distance_squared, NoL);

    return float4(final, 1);
}
