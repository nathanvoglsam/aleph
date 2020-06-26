//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "normal_map.hlsl"
#include "pbr.hlsl"
#include "standard_tex.inc.hlsl"

[[vk::binding(0,0)]]
ConstantBuffer<CameraLayout> camera_buffer;

[[vk::binding(0,2)]]
SamplerState BaseColourSampler;

[[vk::binding(1,2)]]
Texture2D<float3> BaseColourTex;

[[vk::binding(2,2)]]
SamplerState NormalSampler;

[[vk::binding(3,2)]]
Texture2D<float3> NormalTex;

float4 main(in StaticMeshPixelInput input) : SV_Target0 {
	// Load buffers so auto complete works properly
    const CameraLayout camera = camera_buffer;
	
    // Light parameters
    const float3 light_position = float3(0,0,0);
    const float lumens = 5000;

    // Camera and light vectors
    const float3 camera_to_frag = camera.position - input.position;
    const float3 light_to_frag = input.position - light_position;
    const float3 frag_to_light = light_position - input.position;

    // Material parameters
    const float3 base_colour = BaseColourTex.Sample(BaseColourSampler, input.uv);
    const float metallic = 0.0;
    const float roughness = RemapRoughness(0.8);
    const float reflectance = 0.5;
    
    // Derive TBN and sample the normal map
	float3 n;
	float3 t;
	float3 b;
	const float3 normal_sample = NormalTex.Sample(NormalSampler, input.uv);
	const float3 map_normal = normalize((normal_sample * 2.0) - 1.0);
	TBNNormalMapSample(map_normal, input.normal, input.tangent, n, t, b);
    
    // Derived material parameters
	const float3 v = normalize(camera_to_frag);
	const float3 l = normalize(light_to_frag);
	const float3 diffuse_colour = DiffuseFromBaseColour(base_colour, roughness);
	const float3 f0 = CalculateF0(base_colour, metallic, reflectance);

    // Calculate the result of our BRDF
    const float3 brdf = StandardBRDF(v, l, n, diffuse_colour, roughness, f0);

    // Apply a single point light
    const float NoL = clamp(dot(n, normalize(frag_to_light)), 0.0, 1.0);
    const float distance_squared = dot(light_to_frag, light_to_frag);
    const float3 final = EvaluatePointLight(brdf, lumens, distance_squared, NoL);

    return float4(final, 1);
}
