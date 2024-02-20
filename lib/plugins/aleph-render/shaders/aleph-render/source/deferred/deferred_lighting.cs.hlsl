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

#include "projection.hlsl"

[[vk::binding(0, 0)]]
Texture2D<float> g_depth : register(t0, space0);
[[vk::binding(1, 0)]]
Texture2D<float4> g_gbuffer0 : register(t1, space0);
[[vk::binding(2, 0)]]
Texture2D<float4> g_gbuffer1 : register(t2, space0);
[[vk::binding(3, 0)]]
Texture2D<float4> g_gbuffer2 : register(t3, space0);
[[vk::binding(4, 0)]]
RWTexture2D<float4> g_output : register(u4, space0);

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float3 position;
};

[[vk::binding(5, 0)]]
ConstantBuffer<CameraLayout> g_camera : register(b5, space0);

[numthreads(8, 8, 1)]
void main(uint3 dispatch_thread_id: SV_DispatchThreadID)
{
    uint width;
    uint height;
    g_depth.GetDimensions(width, height);

    let texCoord = int3(dispatch_thread_id.x, dispatch_thread_id.y, 0);

    if (texCoord.x < width && texCoord.y < height) {
        let viewportX = ((float(texCoord.x) / float(width)) - 0.5) * 2;
        let viewportY = ((float(texCoord.y) / float(height)) - 0.5) * 2;
        let viewportZ = g_depth.Load(texCoord);
        let viewportPoint = float3(viewportX, viewportY, viewportZ);
        let viewspacePoint = UnprojectPointWithMatrix(g_camera.proj_matrix, viewportPoint);

        // let v = g_gbuffer0.Load(texCoord);
        g_output[dispatch_thread_id.xy] = float4(viewspacePoint, 1);
    }
}
