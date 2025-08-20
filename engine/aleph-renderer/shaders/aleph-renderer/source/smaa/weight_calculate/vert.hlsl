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

#include "rhi.hlsl"

#include "smaa/metrics.hlsl"
#include "payload.hlsl"

PUSH_CONSTANT(SmaaMetrics, g_constants);

#define SMAA_RT_METRICS g_constants.metrics
#define SMAA_INCLUDE_PS 0

#include "smaa/SMAA.hlsl"

[shader("vertex")]
PixelInput main(uint id : SV_VertexID) {
	// Fullscreen triangle generation
	PixelInput output;
	output.uv = float2((id << 1) & 2, id & 2);
	output.sv_position = float4(output.uv * float2(2, -2) + float2(-1, 1), 0, 1);

	// Writes into output.offset and output.pixcoord
    SMAABlendingWeightCalculationVS(output.uv, output.pixcoord, output.offset);

	return output;
}
