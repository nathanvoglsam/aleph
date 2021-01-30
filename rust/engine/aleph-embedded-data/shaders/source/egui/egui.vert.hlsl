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

#include "egui.inc.hlsl"
#include <common.hlsl>

// 0-1 linear  from  0-255 sRGB
inline float3 LinearFromSRGB(float3 srgb) {
    const float3 vec_10_31475 = float3(10.31475, 10.31475, 10.31475);
    const float3 vec_3294_6 = float3(3294.6, 3294.6, 3294.6);
    const float3 vec_14_025 = float3(14.025, 14.025, 14.025);
    const float3 vec_269_025 = float3(269.025, 269.025, 269.025);
    const float3 vec_2_4 = float3(2.4, 2.4, 2.4);

    const bool3 cutoff = srgb < vec_10_31475;
    const float3 lower = srgb / vec_3294_6;
    const float3 higher = pow((srgb + vec_14_025) / vec_269_025, vec_2_4);
    return lerp(higher, lower, cutoff);
}
inline float4 LinearFromSRGBA(float4 srgba) {
    const float alpha = srgba.a / 255;
    const float3 colour = LinearFromSRGB(srgba.rgb);
    return float4(colour, alpha);
}

struct PushConstantLayout {
    float2 ScreenSize;
};

[[vk::push_constant]]
PushConstantLayout PushConstants;

EguiPixelInput main(in EguiVertexInput input, out float4 Pos : SV_POSITION) {
    EguiPixelInput output;

    // Transform input into final output vertex position
    const float x = 2.0 * input.Pos.x / PushConstants.ScreenSize.x - 1.0;
    const float y = 2.0 * input.Pos.y / PushConstants.ScreenSize.y - 1.0;
    const float z = 0.0;
    const float w = 1.0;
    Pos = float4(x, y, z, w);

    // Package up fragment payload
    output.Color = LinearFromSRGBA(input.Color * 255);
    output.UV = input.UV;

    return output;
}
