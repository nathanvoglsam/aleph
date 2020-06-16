//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

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
	const float3x3 tbn = float3x3(t, b, mesh_normal);
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
	n = normalize(map_normal.x * t + map_normal.y * b + map_normal.z * mesh_normal);
	t = normalize(tangent);
	b = normalize(bitangent);
}
