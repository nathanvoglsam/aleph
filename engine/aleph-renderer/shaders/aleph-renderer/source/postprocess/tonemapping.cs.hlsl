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

#include "common.hlsl"
#include "srgb.hlsl"
#include "tonemap.hlsl"

[[vk::binding(0, 0)]]
Texture2D<float4> g_input : register(t0, space0);

[[vk::binding(1, 0)]]
RWTexture2D<float4> g_output : register(u1, space0);

[numthreads(8, 8, 1)]
void main(uint3 dispatch_thread_id: SV_DispatchThreadID)
{
    uint width;
    uint height;
    g_input.GetDimensions(width, height);

    if (dispatch_thread_id.x < width && dispatch_thread_id.y < height) {
        float3 colour = g_input.Load(int3(dispatch_thread_id.x, dispatch_thread_id.y, 0)).rgb;

        // // Get our linear space ACES colour
        // float3 acesColour = ACESFitted(colour);

        // Then while we're still linear we get an estimate for luminance, which will go in the
        // spare alpha. This is intended to be consumed by FXAA
        float luma = LuminanceFromLinearRGB(colour);

        // Finally do the SRGB conversion before output
        float3 srgbColour = LinearToSRGB(colour);

        g_output[dispatch_thread_id.xy] = float4(srgbColour, luma);
    }
}

