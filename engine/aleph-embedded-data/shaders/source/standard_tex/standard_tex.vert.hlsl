//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "standard_tex.inc.hlsl"

[[vk::binding(0,0)]]
ConstantBuffer<CameraLayout> camera_buffer;

[[vk::binding(0,1)]]
ConstantBuffer<ModelLayout> model_buffer;

StaticMeshPixelInput main(in StaticMeshVertexInput input, out float4 out_position : SV_POSITION) {
    // Load buffers so auto complete works properly
    const CameraLayout camera = camera_buffer;
    const ModelLayout model = model_buffer;
	
    StaticMeshPixelInput output;

    const float4 in_pos = float4(input.position, 1.0);
    const float3x3 normal_matrix = (float3x3)model.normal_matrix;
    
    float4 position = mul(in_pos, model.model_matrix);
    float3 normal = normalize(mul(input.normal, normal_matrix));
    float3 tangent = normalize(mul(input.tangent.xyz, normal_matrix));
    output.position = position.xyz;
    output.normal = normal;
    output.tangent = float4(tangent, input.tangent.w);
    output.uv = input.uv;

    position = mul(position, camera.view_matrix);
    position = mul(position, camera.proj_matrix);
    
    out_position = position;

    return output;
}