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

struct ModelLayout {
    float4x4 model_matrix;
    float4x4 normal_matrix;
};

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float3 position;
};

[[vk::binding(0,0)]]
ConstantBuffer<CameraLayout> camera;

[[vk::binding(1,0)]]
ConstantBuffer<ModelLayout> model;

StaticMeshPixelInput main(in StaticMeshVertexInput input, out float4 pos : SV_POSITION) {
    StaticMeshPixelInput output;

    const float4 in_pos = float4(input.position, 1.0);
    const float3x3 normal_matrix = (float3x3)model.normal_matrix;
    
    float4 out_pos = mul(in_pos, model.model_matrix);

    output.position = out_pos.xyz;
    output.normal = normalize(mul(input.normal, normal_matrix));
    output.tangent = normalize(mul(input.tangent, normal_matrix));
    output.bitangent = normalize(mul(cross(input.normal, input.tangent), normal_matrix));
    output.uv = input.uv;

    out_pos = mul(out_pos, camera.view_matrix);
    out_pos = mul(out_pos, camera.proj_matrix);
    pos = out_pos;

    return output;
}