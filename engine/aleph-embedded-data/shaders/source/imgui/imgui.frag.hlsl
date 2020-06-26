//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "imgui.inc.hlsl"

[[vk::binding(0,0)]]
Texture2D Tex;

[[vk::binding(1,0)]]
SamplerState Sampler;

float4 main(in ImGuiPixelInput input) : SV_Target0 {
    return input.Color * Tex.Sample(Sampler, input.UV);
}
