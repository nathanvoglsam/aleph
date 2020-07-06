//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "standard.inc.hlsl"
#include "pbr.hlsl"

[[vk::binding(0,0)]]
ConstantBuffer<CameraLayout> camera_buffer;

float4 main(in StaticMeshPixelInput input) : SV_Target0 {
    // Load buffers so auto complete works properly
    const CameraLayout camera = camera_buffer;
	
    // Light parameters
    const float3 light_position = float3(5.0,0.5,1.0);
    const float lumens = 1000;

    // Camera and light vectors
    const float3 camera_to_frag = camera.position - input.position;
    const float3 light_to_frag = input.position - light_position;

    // Material parameters
    const float3 base_colour = float3(0.2,1,1);
    const float metallic = 0.0;
    const float roughness = RemapRoughness(0.01);
    const float reflectance = 0.5;

    // Derived material parameters
    const float3 v = normalize(camera_to_frag);
    const float3 l = normalize(light_to_frag);
    const float3 n = normalize(input.normal);
    const float3 f0 = CalculateF0(base_colour, metallic, reflectance);

    // Calculate the result of our BRDF
    const float3 brdf = ClearCoatBRDF(v, l, n, base_colour, metallic, roughness, f0, 1.0, 0.1);

    // Apply a single point light
    const float NoL = clamp(dot(n, l), 0.0, 1.0);
    const float distance_squared = dot(light_to_frag, light_to_frag);
    const float3 final = EvaluatePointLight(brdf, lumens, distance_squared, NoL);

    return float4(final, 1);
}
