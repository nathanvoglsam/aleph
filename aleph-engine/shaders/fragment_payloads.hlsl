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
struct ImGuiPixelInput
{
    float4 Color : COLOR0;
    float2 UV    : TEXCOORD0;
};

/*
 * Standard fragment payload for passing interpolated params to fragment shader for static mesh
 */
struct StaticMeshPixelInput
{
    float3 position  : POSITION;
    float3 normal    : NORMAL;
    float3 tangent   : TANGENT;
    float3 bitangent : BINORMAL;
    float2 uv        : TEXCOORD0;
};
