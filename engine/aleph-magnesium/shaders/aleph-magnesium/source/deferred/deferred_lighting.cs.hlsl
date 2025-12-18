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
#include "sampling.hlsl"

struct CameraLayout {
    float4x4 view_matrix;
    float4x4 proj_matrix;
    float4 position;
};

struct LightingParams {
    Texture2D<float> depth;
    Texture2D<float4> gbuffer0;
    Texture2D<float2> gbuffer1;
    Texture2D<float4> gbuffer2;
    RWTexture2D<float4> output;
    ConstantBuffer<CameraLayout> camera;
};

// Light parameters
static float lumens = 200;

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
    let diffuse_colour = CalculateDiffuseColour(base_colour, metallic);

    // Calculate the result of our BRDF
    let brdf = StandardBRDF(v, l, n, diffuse_colour, metallic, roughness, f0);

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
    let diffuse_colour = CalculateDiffuseColour(base_colour, metallic);

    // Calculate the result of our BRDF
    let brdf = StandardBRDF(v, l, n, diffuse_colour, metallic, roughness, f0);

    // Apply a single point light
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    return brdf * NoL;
}

[shader("compute")]
[numthreads(8, 8, 1)]
void main(uint3 dispatch_thread_id: SV_DispatchThreadID, ParameterBlock<LightingParams> params)
{
    uint width;
    uint height;
    params.depth.GetDimensions(width, height);

    let texCoord = int3(dispatch_thread_id.x, dispatch_thread_id.y, 0);

    let view_rotation_matrix = float3x3(params.camera.view_matrix);

    if (texCoord.x < width && texCoord.y < height) {
        let viewportX = ((float(texCoord.x) / float(width)) - 0.5) * 2;
        let viewportY = ((float(texCoord.y) / float(height)) - 0.5) * 2;
        let viewportZ = params.depth.Load(texCoord);
        let viewportPoint = float3(viewportX, -viewportY, viewportZ);
        let viewspacePoint = UnprojectPointWithMatrix(params.camera.proj_matrix, viewportPoint);

        // If we still have the clear value of the depth buffer then don't do any lighting as this
        // isn't an object
        if (viewportZ == 0) {
            params.output[dispatch_thread_id.xy] = float4(0, 0, 0, 1);
            return;
        }

        let base_colour = params.gbuffer0.Load(texCoord).rgb;
        let ws_oct_normal = params.gbuffer1.Load(texCoord).xy;
        let ws_normal = OctahedralDecode(ws_oct_normal);
        let gbuffer_2 = params.gbuffer2.Load(texCoord);
        let metallic = gbuffer_2.r;
        let roughness = RemapRoughness(gbuffer_2.g);

        // Transform the normal into viewspace, where we do all our lighting
        let vs_normal = mul(view_rotation_matrix, ws_normal);

        float3 light = float3(0, 0, 0);

        let point_pos = mul(params.camera.view_matrix, float4(float3(1.5, 0.2, 0), 1), );
        light += PointLight(point_pos.xyz, viewspacePoint, base_colour, vs_normal, metallic, roughness);

        let dir_vector = mul(view_rotation_matrix, normalize(float3(1, 1, 1)), );
        light += DirectionLight(dir_vector, viewspacePoint, base_colour, vs_normal, metallic, roughness) * 2.0;
        light += DirectionLight(-dir_vector, viewspacePoint, base_colour, vs_normal, metallic, roughness) * float3(0.25, 0.1, 0.1);

        params.output[dispatch_thread_id.xy] = float4(light, 1);
    }
}
