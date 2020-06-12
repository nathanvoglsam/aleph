//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "fragment_payloads.hlsl"

[[vk::binding(0,0)]]
SamplerState Sampler;

[[vk::binding(0,0)]]
Texture2D Tex;

float4 main(in ImGuiPixelInput input) : SV_Target0
{
    return input.Color * Tex.Sample(Sampler, input.UV);
}
