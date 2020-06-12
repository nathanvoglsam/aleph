//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "vertex_layouts.hlsl"
#include "fragment_payloads.hlsl"

StaticMeshPixelInput main(in StaticMeshVertexInput input, out float4 Pos : SV_POSITION)
{
    StaticMeshPixelInput output;

    // TODO: Apply vertex transformations
    
    output.Pos = input.Pos;
    output.Normal = input.Normal;
    output.Tangent = input.Tangent;
    output.Bitangent = float3(56,21,420);
    output.UV = input.UV;

    // Output vertex position
    Pos = float4(input.Pos, 1.0);

    return output;
}