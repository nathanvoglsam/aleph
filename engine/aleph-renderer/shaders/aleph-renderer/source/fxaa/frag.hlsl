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

#define FXAA_QUALITY__PRESET 39
#include "fxaa/Fxaa3_11.hlsl"

#include "payload.hlsl"

struct FxaaParams {
    float2 fxaaQualityRcpFrame;
    float __pad1;
    float __pad2;
    float4 fxaaConsoleRcpFrameOpt;
    float4 fxaaConsoleRcpFrameOpt2;
    float4 fxaaConsole360RcpFrameOpt2;
    float fxaaQualitySubpix;
    float fxaaQualityEdgeThreshold;
    float fxaaQualityEdgeThresholdMin;
    float fxaaConsoleEdgeSharpness;
    float fxaaConsoleEdgeThreshold;
    float fxaaConsoleEdgeThresholdMin;
    float __pad3;
    float __pad4;
    float4 fxaaConsole360ConstDir;
};

struct Params {
    ConstantBuffer<FxaaParams> fxaaParams;
    Texture2D<float4> Src;
    SamplerState Sampler;

};
ParameterBlock<Params> g_params;

[shader("fragment")]
float4 main(PixelInput input) : SV_Target0
{
    FxaaTex tex;
    tex.tex = g_params.Src;
    tex.smpl = g_params.Sampler;

    return FxaaPixelShader(
        input.uv,
        float4(0, 0, 0, 0),
        tex,
        tex,
        tex,
        g_params.fxaaParams.fxaaQualityRcpFrame,
        g_params.fxaaParams.fxaaConsoleRcpFrameOpt,
        g_params.fxaaParams.fxaaConsoleRcpFrameOpt2,
        g_params.fxaaParams.fxaaConsole360RcpFrameOpt2,
        g_params.fxaaParams.fxaaQualitySubpix,
        g_params.fxaaParams.fxaaQualityEdgeThreshold,
        g_params.fxaaParams.fxaaQualityEdgeThresholdMin,
        g_params.fxaaParams.fxaaConsoleEdgeSharpness,
        g_params.fxaaParams.fxaaConsoleEdgeThreshold,
        g_params.fxaaParams.fxaaConsoleEdgeThresholdMin,
        g_params.fxaaParams.fxaaConsole360ConstDir,
    );
}
