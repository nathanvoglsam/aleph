//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

struct PixelInput
{
    float4 Color : COLOR0;
    float2 UV    : TEXCOORD0;
};

[[vk::binding(0,0)]]
SamplerState Sampler;

[[vk::binding(0,0)]]
Texture2D Tex;

float4 main(in PixelInput input) : SV_Target0
{
    return input.Color * Tex.Sample(Sampler, input.UV);
}
