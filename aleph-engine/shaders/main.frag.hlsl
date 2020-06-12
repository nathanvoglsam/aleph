//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "fragment_payloads.hlsl"
#include "pbr.hlsl"

[[vk::binding(0,0)]]
SamplerState BaseColourSampler;

[[vk::binding(0,0)]]
Texture2D BaseColourTex;

[[vk::binding(0,1)]]
SamplerState NormalSampler;

[[vk::binding(0,1)]]
Texture2D NormalTex;

float4 main(in StaticMeshPixelInput input) : SV_Target0
{
    return float4(1,1,1,1);
}
