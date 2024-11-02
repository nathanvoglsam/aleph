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

// The SRGB expectations for SMAA are a bit difficult to decipher from the documentation but I think
// I have it down after studying the reference implementation.
//
// There are a few assumptions. Firstly, the input colour we want to anti-alias is assumed to be an
// SRGB texture. That is, the data it contains is SRGB. It is also assumed we want to output SRGB
// encoded colour as the ultimate anti-aliased result.
//
// Given those assumptions then we can extract the following:
//
// - The input colour is SRGB encoded.
// - The first pass (edge detection) operates in linear colour, so we must use an SRGB view to
//   decode from SRGB into linear colour.
// - The output from the edge detection pass is linear, and so should be stored as UNORM and only
//   ever viewed as UNORM.
// - The weight calculate pass also operates in linear colour, so we must again use an SRGB view to
//   read from input colour. Just like the edge detection pass.
// - And again, for weight calculate, our output 'weight tex' is also linear, and so is stored and
//   viewed as UNORM.
// - The final pass, blending, is different. The blend pass assumes an SRGB input but the reference
//   implementation uses a UNORM view which means the texture samples will get the encoded SRGB
//   values, and not linear colour. As far as I can tell this is intentional and I make the
//   assumption the reference implementation is correct.
// - The pre-calculated textures are plain UNORM and contain linear data.
//
// The take-away is that when SMAA.hlsl writes 'All texture reads and buffer writes must be
// non-sRGB, with the exception of the input read and the output write in "SMAANeighborhoodBlending"
// (and only in this pass!)' I believe they intend that all inputs must be in linear colour except
// for the final blending pass. Hence why an SRGB view is used for the input colour in the first
// two passes, as the input colour is SRGB encoded and must be decoded. This also makes the
// reference implementation consistent with the documentation for the blending pass, which uses
// plain UNORM views for both the colour read and render target. It follows then that input colour
// is still in SRGB space, and the output is stored as SRGB.

[[vk::binding(0, 0)]]
Texture2D blendTex : register(t0);

[[vk::binding(1, 0)]]
Texture2D colorTex : register(t1);

[[vk::binding(2, 0)]]
SamplerState LinearSampler : register(s2);

[[vk::binding(3, 0)]]
SamplerState PointSampler : register(s3);

[[vk::push_constant]]
ConstantBuffer<SmaaMetrics> g_constants : register(b0, space1024);

#define SMAA_RT_METRICS g_constants.metrics
#define SMAA_INCLUDE_VS 0

#include "smaa/SMAA.hlsl"

float4 main(PixelInput input) : SV_Target0
{
    return SMAANeighborhoodBlendingPS(input.uv, input.offset, colorTex, blendTex);
}
