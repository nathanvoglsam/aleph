//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

struct VertexInput
{
    [[vk::location(0)]] float2 Pos   : SV_POSITION;
    [[vk::location(1)]] float2 UV    : TEXCOORD0;
    [[vk::location(2)]] float4 Color : COLOR0;
};

struct PixelInput
{
    float4 Color : COLOR0;
    float2 UV    : TEXCOORD0;
};

struct PushConstantLayout
{
    float2 Scale;
    float2 Translate;
};

[[vk::push_constant]]
PushConstantLayout PushConstants;

[[vk::location(0)]]
PixelInput main(in VertexInput input, out float4 Pos : SV_POSITION)
{

    PixelInput output;

    Pos = float4((input.Pos * PushConstants.Scale) + PushConstants.Translate,0,1.0);
    output.Color = input.Color;
    output.UV = input.UV;

    return output;
}