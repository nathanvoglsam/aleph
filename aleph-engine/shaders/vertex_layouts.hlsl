//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

/*
 * The standard vertex input layout that the ImGui shaders use
 */
struct ImGuiVertexInput
{
    [[vk::location(0)]] float2 Pos   : SV_POSITION;
    [[vk::location(1)]] float2 UV    : TEXCOORD0;
    [[vk::location(2)]] float4 Color : COLOR0;
};

/*
 * The standard vertex layout that a static mesh shader will use
 */
struct StaticMeshVertexInput {
    [[vk::location(0)]] float3 position : SV_POSITION;
    [[vk::location(1)]] float3 normal   : NORMAL;
    [[vk::location(2)]] float3 tangent  : TANGENT;
    [[vk::location(3)]] float2 uv       : TEXCOORD0;
};
