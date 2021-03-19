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

#pragma once

/*
 * Calculate a world space normal vector from a normal map and the vertex normal + vertex tangent.
 *
 * This calculates a TBN matrix and transforms the tangent space normal with it.
 *
 * Arguments:
 * 
 * - map_normal: A sample from a tangent space normal map
 * - mesh_normal: The raw interpolated vertex normal
 * - mesh_tangent: The raw interpolated vertex tangent
 * - OUT n: The output normal mapped normal vector
 * - OUT t: The normalized tangent vector
 * - OUT b: The normalized bitangent vector
 */
void TBNNormalMapSample(const float3 map_normal, const float3 mesh_normal, const float4 mesh_tangent, out float3 n, out float3 t, out float3 b) {
	const float3 tangent = mesh_tangent.xyz;
	const float3 bitangent = cross(mesh_normal, tangent) * sign(mesh_tangent.w);
	const float3x3 tbn = float3x3(tangent, bitangent, mesh_normal);
	n = normalize(mul(map_normal, tbn));
	t = normalize(tangent);
	b = normalize(bitangent);

}

/*
 * Calculate a world space normal vector from a normal map and the vertex normal + vertex tangent.
 *
 * This uses the code listing on http://www.mikktspace.com/ for sampling the normal under the Pixel Shader Transformation section
 *
 * Arguments:
 * 
 * - map_normal: A sample from a tangent space normal map
 * - mesh_normal: The raw interpolated vertex normal
 * - mesh_tangent: The raw interpolated vertex tangent
 * - OUT n: The output normal mapped normal vector
 * - OUT t: The normalized tangent vector
 * - OUT b: The normalized bitangent vector
 */
void MikkTNormalMapSample(const float3 map_normal, const float3 mesh_normal, const float4 mesh_tangent, out float3 n, out float3 t, out float3 b) {
	const float3 tangent = mesh_tangent.xyz;
	const float3 bitangent = cross(mesh_normal, tangent) * sign(mesh_tangent.w);
	n = normalize(map_normal.x * tangent + map_normal.y * bitangent + map_normal.z * mesh_normal);
	t = normalize(tangent);
	b = normalize(bitangent);
}
