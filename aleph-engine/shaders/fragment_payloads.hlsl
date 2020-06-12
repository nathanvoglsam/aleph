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
    float3 Pos       : POSITION;
    float3 Normal    : NORMAL;
    float3 Tangent   : TANGENT;
    float3 Bitangent : BINORMAL;
    float2 UV        : TEXCOORD0;
};
