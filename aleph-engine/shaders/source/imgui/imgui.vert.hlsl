//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "imgui.inc.hlsl"

struct PushConstantLayout {
    float2 Scale;
    float2 Translate;
};

[[vk::push_constant]]
PushConstantLayout PushConstants;

ImGuiPixelInput main(in ImGuiVertexInput input, out float4 Pos : SV_POSITION) {
    ImGuiPixelInput output;

    Pos = float4((input.Pos * PushConstants.Scale) + PushConstants.Translate,0,1.0);
    output.Color = input.Color;
    output.UV = input.UV;

    return output;
}