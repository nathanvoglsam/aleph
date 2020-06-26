//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include <aces.hlsl>
#include <fullscreen_quad/fullscreen_quad.inc.hlsl>

[[vk::input_attachment_index(0)]]
[[vk::binding(0, 0)]]
SubpassInput<float4> ColourInput;

float3 LinearTosRGB(in float3 color) {
    float3 x = color * 12.92f;
    float3 y = 1.055f * pow(saturate(color), 1.0f / 2.4f) - 0.055f;

    float3 clr = color;
    clr.r = color.r < 0.0031308f ? x.r : y.r;
    clr.g = color.g < 0.0031308f ? x.g : y.g;
    clr.b = color.b < 0.0031308f ? x.b : y.b;

    return clr;
}

float4 main(in FSQuadPSInput input) : SV_Target0 {
    float3 colour = ColourInput.SubpassLoad().xyz;
    colour = LinearTosRGB(ACESFitted(colour) * 1.8f);
    return float4(colour, 1.0);
}
