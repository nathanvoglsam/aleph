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

// Material parameters
static float3 base_colour = float3(0.2,1,1);
static float metallic = 0.0;
static float roughness = RemapRoughness(0.01);
static float reflectance = 0.5;

struct PixelOutput {
    float4 gbuffer_0: SV_Target0;
    float4 gbuffer_1: SV_Target1;
    float2 gbuffer_2: SV_Target2;
};

PixelOutput main(in StaticMeshPixelInput input) {
    const float3 n = normalize(input.normal);

    PixelOutput output;
    output.gbuffer_0.xyz = base_colour;
    output.gbuffer_0.w = 1;
    output.gbuffer_1.xyz = n;
    output.gbuffer_2.x = metallic;
    output.gbuffer_2.y = roughness;

    return output;
}
