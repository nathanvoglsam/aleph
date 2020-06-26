//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

/*
 * Standard ImGui fragment payload for passing interpolated colour and UV to frag shader
 */
struct ImGuiPixelInput {
    float4 Color : COLOR0;
    float2 UV    : TEXCOORD0;
};

/*
 * The standard vertex input layout that the ImGui shaders use
 */
struct ImGuiVertexInput {
    [[vk::location(0)]] float2 Pos   : SV_POSITION;
    [[vk::location(1)]] float2 UV    : TEXCOORD0;
    [[vk::location(2)]] float4 Color : COLOR0;
};
