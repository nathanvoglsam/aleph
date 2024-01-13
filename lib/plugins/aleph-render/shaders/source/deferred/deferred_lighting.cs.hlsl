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

struct Params {
    int2 size;
};

ConstantBuffer<Params> g_params : register(b0, space0);
Texture2D<float4> g_gbuffer0 : register(t1, space0);
Texture2D<float4> g_gbuffer1 : register(t2, space0);
Texture2D<float4> g_gbuffer2 : register(t3, space0);
RWTexture2D<float4> g_output : register(u4, space0);

[numthreads(8, 8, 1)]
void main(uint3 dispatch_thread_id: SV_DispatchThreadID)
{
    if (dispatch_thread_id.x < g_params.size.x && dispatch_thread_id.y < g_params.size.y) {
        float4 v = g_gbuffer0.Load(int3(0, 0, 0));
        g_output[dispatch_thread_id.xy] = v;
    }
}
