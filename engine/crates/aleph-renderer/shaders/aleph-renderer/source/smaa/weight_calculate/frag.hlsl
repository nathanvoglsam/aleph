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

#include "smaa/metrics.hlsl"
#include "payload.hlsl"

[[vk::binding(0, 0)]]
Texture2D edgesTex : register(t0);

[[vk::binding(1, 0)]]
Texture2D areaTex : register(t1);

[[vk::binding(2, 0)]]
Texture2D searchTex : register(t2);

[[vk::binding(3, 0)]]
SamplerState LinearSampler : register(s3);

[[vk::binding(4, 0)]]
SamplerState PointSampler : register(s4);

[[vk::push_constant]]
ConstantBuffer<SmaaMetrics> g_constants : register(b0, space1024);

#define SMAA_RT_METRICS g_constants.metrics
#define SMAA_INCLUDE_VS 0

#include "smaa/SMAA.hlsl"

float4 main(PixelInput input) : SV_Target0
{
    // Unused, needed for temporal SMAA which we aren't using
    float4 subsampleIndices = float4(0, 0, 0, 0);
    return SMAABlendingWeightCalculationPS(input.uv, input.pixcoord, input.offset, edgesTex, areaTex, searchTex, subsampleIndices);
}
