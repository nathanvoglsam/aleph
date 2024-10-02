//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#include "pbr.hlsl"
#include "projection.hlsl"

[[vk::binding(0, 0)]]
Texture2D<float> g_depth : register(t0, space0);
[[vk::binding(1, 0)]]
Texture2D<float4> g_gbuffer0 : register(t1, space0);
[[vk::binding(2, 0)]]
Texture2D<float4> g_gbuffer1 : register(t2, space0);
[[vk::binding(3, 0)]]
Texture2D<float4> g_gbuffer2 : register(t3, space0);
[[vk::binding(4, 0)]]
RWTexture2D<float4> g_output : register(u4, space0);

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float4 position;
};

[[vk::binding(5, 0)]]
ConstantBuffer<CameraLayout> g_camera : register(b5, space0);

// Light parameters
static float lumens = 100;

static float reflectance = 0.5;

func PointLight(
    light_position: float3,
    frag_position: float3,
    base_colour: float3,
    normal: float3,
    metallic: float,
    roughness: float,
) -> float3 {
    // Camera and light vectors
    let frag_to_camera = -frag_position; // We light in view space, 0,0,0 is implicitly the camera
    let frag_to_light = light_position - frag_position;

    // Derived material parameters
    let v = normalize(frag_to_camera);
    let l = normalize(frag_to_light);
    let n = normalize(normal);
    let f0 = CalculateF0(base_colour, metallic, reflectance);

    // Calculate the result of our BRDF
    let brdf = StandardBRDF(v, l, n, base_colour, metallic, roughness, f0);

    // Apply a single point light
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    let distance_squared = dot(frag_to_light, frag_to_light);
    return EvaluatePointLight(brdf, lumens, distance_squared, NoL);
}

func DirectionLight(
    light_direction: float3,
    frag_position: float3,
    base_colour: float3,
    normal: float3,
    metallic: float,
    roughness: float,
) -> float3 {
    // Derived material parameters
    let v = normalize(-frag_position); // We light in view space, 0,0,0 is implicitly the camera
    let l = normalize(light_direction);
    let n = normalize(normal);
    let f0 = CalculateF0(base_colour, metallic, reflectance);

    // Calculate the result of our BRDF
    let brdf = StandardBRDF(v, l, n, base_colour, metallic, roughness, f0);

    // Apply a single point light
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    return brdf * NoL;
}

[numthreads(8, 8, 1)]
void main(uint3 dispatch_thread_id: SV_DispatchThreadID)
{
    uint width;
    uint height;
    g_depth.GetDimensions(width, height);

    let texCoord = int3(dispatch_thread_id.x, dispatch_thread_id.y, 0);

    let view_rotation_matrix = float3x3(g_camera.view_matrix);

    if (texCoord.x < width && texCoord.y < height) {
        let viewportX = ((float(texCoord.x) / float(width)) - 0.5) * 2;
        let viewportY = ((float(texCoord.y) / float(height)) - 0.5) * 2;
        let viewportZ = g_depth.Load(texCoord);
        let viewportPoint = float3(viewportX, -viewportY, viewportZ);
        let viewspacePoint = UnprojectPointWithMatrix(g_camera.proj_matrix, viewportPoint);

        // If we still have the clear value of the depth buffer then don't do any lighting as this
        // isn't an object
        if (viewportZ == 0) {
            g_output[dispatch_thread_id.xy] = float4(0, 0, 0, 1);
            return;
        }

        let base_colour = g_gbuffer0.Load(texCoord).rgb;
        let ws_normal = g_gbuffer1.Load(texCoord).xyz;
        let gbuffer_2 = g_gbuffer2.Load(texCoord);
        let metallic = gbuffer_2.r;
        let roughness = RemapRoughness(gbuffer_2.g);

        // Transform the normal into viewspace, where we do all our lighting
        let vs_normal = mul(ws_normal, view_rotation_matrix);

        float3 light = float3(0, 0, 0);

        let point_pos = mul(float4(float3(1.5, 0, 0), 1), g_camera.view_matrix);
        light += PointLight(point_pos.xyz, viewspacePoint, base_colour, vs_normal, metallic, roughness);

        let dir_vector = mul(normalize(float3(1, 1, 1)), view_rotation_matrix);
        light += DirectionLight(dir_vector, viewspacePoint, base_colour, vs_normal, metallic, roughness) * 0.5;
        light += DirectionLight(-dir_vector, viewspacePoint, base_colour, vs_normal, metallic, roughness) * float3(0.25, 0.1, 0.1);

        g_output[dispatch_thread_id.xy] = float4(light, 1);
    }
}
