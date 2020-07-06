//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#include "constants.hlsl"
#include "saturate.hlsl"



// float3 energyCompensation = 1.0 + f0 * (1.0 / dfg.y - 1.0);
// // Scale the specular lobe to account for multiscattering
// Fr *= pixel.energyCompensation;



/*
 * Calculates the attenuation factor for a point light that should be applied to 
 */
inline float PointLightAttenuation(const float distance_squared) {
    return 1 / (4 * PI * distance_squared);
}

/*
 * Given the output of a brdf, and a set of point light parameters, calculate the final light
 * contribution.
 */
inline float3 EvaluatePointLight(
    const float3 brdf,
    const float lumens,
    const float distance_squared,
    const float NoL
) {
    const float attenuation = PointLightAttenuation(distance_squared);
    return brdf * (lumens * attenuation * NoL);
}

/*
 * Performs a remapping of the roughness parameter. It makes the roughness parameter seem more
 * linear when being tweaked so it's a bit more intuitive to work with.
 */
inline float RemapRoughness(const float perceptual_roughness) {
    return SaturateFP32(perceptual_roughness * perceptual_roughness);
}

/*
 * Calculates the 'at' term for use in the anisotropic material model
 */
inline float CalculateAT(const float roughness, const float anisotropy) {
    return max(roughness * (1.0 + anisotropy), 0.001);
}

/*
 * Calculates the 'ab' term for use in the anisotropic material model
 */
inline float CalculateAB(const float roughness, const float anisotropy) {
    return max(roughness * (1.0 - anisotropy), 0.001);
}

/*
 * Calculates the F0 value from the base colour, metallic and reflectance
 *
 * Arguments:
 *
 * - base_colour: The base colour parameter of the material
 * - metallic: The metallic parameter of the material
 * - reflectance: The reflectance parameter of the material
 */
inline float3 CalculateF0(const float3 base_colour, const float metallic, const float reflectance) {
    return 0.16 * reflectance * reflectance * (1.0 - metallic) + base_colour * metallic;
}

/*
 * The GGX D term
 */
inline float D_GGX(const float NoH, const float roughness) {
    const float a = NoH * roughness;
    const float k = roughness / (1.0 - NoH * NoH + a * a);
    return k * k * (1.0 / PI);
}

/*
 * The GGX D term with support for anisotropic materials
 */
inline float D_GGX_Anisotropic(
    const float NoH,
    const float3 h,
    const float3 t,
    const float3 b,
    const float at,
    const float ab
) {
    const float ToH = dot(t, h);
    const float BoH = dot(b, h);
    const float a2 = at * ab;
    const float3 v = float3(ab * ToH, at * BoH, a2 * NoH);
    const float v2 = dot(v, v);
    const float w2 = a2 / v2;
    return a2 * w2 * w2 * (1.0 / PI);
}

/*
 * The SmithGGXCorrelated V term
 */
inline float V_SmithGGXCorrelated(const float NoV, const float NoL, const float roughness) {
    const float a2 = roughness * roughness;
    const float GGXV = NoL * sqrt(NoV * NoV * (1.0 - a2) + a2);
    const float GGXL = NoV * sqrt(NoL * NoL * (1.0 - a2) + a2);
    return 0.5 / (GGXV + GGXL);
}

/*
 * The SmithGGXCorrelated V term with support for anisotropic materials
 */
inline float V_SmithGGXCorrelated_Anisotropic(
    const float at,
    const float ab,
    const float ToV,
    const float BoV,
    const float ToL,
    const float BoL,
    const float NoV, 
    const float NoL
) {
    const float lambdaV = NoL * length(float3(at * ToV, ab * BoV, NoV));
    const float lambdaL = NoV * length(float3(at * ToL, ab * BoL, NoL));
    const float v = 0.5 / (lambdaV + lambdaL);
    return SaturateFP32(v);
}

/*
 * The SmithGGXCorrelated V term. Optimized for speed by trading accuracy after noting that all
 * terms under the square roots are in the 0..1 range. This approximation is wrong but faster so
 * pick your poison.
 */
inline float V_SmithGGXCorrelatedFast(const float NoV, const float NoL, const float roughness) {
    const float a = roughness;
    const float GGXV = NoL * (NoV * (1.0 - a) + a);
    const float GGXL = NoV * (NoL * (1.0 - a) + a);
    return 0.5 / (GGXV + GGXL);
}

/*
 * The Schlick F term, using a float3 f0 arg
 */
inline float3 F_SchlickVec(const float u, const float3 f0, const float f90) {
    return f0 + (float3(f90,f90,f90) - f0) * pow(1.0 - u, 5.0);
}

/*
 * The Schlick F term, using a scalar f0 arg
 */
inline float F_Schlick(const float u, const float f0, const float f90) {
    return f0 + (f90 - f0) * pow(1.0 - u, 5.0);
}

/*
 * Lambertian diffuse Fd term. Faster and easier but not as accurate
 */
inline float Fd_Lambert() {
    return 1.0 / PI;
}

/*
 * Disney's Burley diffuse Fd term. Looks good but slower and harder to work with
 */
inline float Fd_Burley(const float NoV, const float NoL, const float LoH, const float roughness) {
    const float f90 = 0.5 + 2.0 * roughness * LoH * LoH;
    const float light_scatter = F_Schlick(NoL, 1.0, f90);
    const float view_scatter = F_Schlick(NoV, 1.0, f90);
    return light_scatter * view_scatter * (1.0 / PI);
}

/*
 * A standard hard surface PBR BRDF. This does not account for light attenuation and intensity.
 *
 * Returns the combined output of the diffuse and specular term prior to lighting
 *
 * Arguments:
 *
 * - v: The view unit vector
 * - l: The incident light unit vector
 * - n: The surface normal vector
 * - base_colour: The base colour of the material
 * - metallic: The metallic parameter of the material
 * - roughness: The roughness value after being mapped from a perceptual roughness value
 * - f0: Reflectance at normal incidence
 */
inline float3 StandardBRDF(
    const float3 v,
    const float3 l,
    const float3 n,
    const float3 base_colour,
    const float metallic,
    const float roughness,
    const float3 f0
) {
    // Half unit vector between l and v
    const float3 h = normalize(v + l);

    const float NoV = abs(dot(n, v)) + 0.00005;
    const float NoL = clamp(dot(n, l), 0.0, 1.0);
    const float NoH = clamp(dot(n, h), 0.0, 1.0);
    const float LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    const float  D = D_GGX(NoH, roughness);
    const float  V = V_SmithGGXCorrelatedFast(NoV, NoL, roughness);
    const float3 F = F_SchlickVec(LoH, f0, 1.0);

    // Specular BRDF
    const float3 Fr_nominator = (D * V) * F;
    const float3 Fr_denominator = max(4 * NoV * NoL, 0.001);
    const float3 Fr = Fr_nominator / Fr_denominator;

    // Diffuse BRDF
    const float3 Fd = base_colour * Fd_Burley(NoV, NoL, LoH, roughness);

    // Diffuse contribution
    const float3 kD = (float3(1,1,1) - F) * (1 - metallic);

    const float3 colour = Fr + (Fd * kD);

    return colour;
}

inline float V_Kelemen(float LoH) {
    return 0.25 / (LoH * LoH);
}

/*
 * A standard hard surface PBR BRDF with an extra clear coat term.
 *
 * Returns the combined output of the diffuse and specular term prior to lighting
 *
 * Arguments:
 *
 * - v: The view unit vector
 * - l: The incident light unit vector
 * - n: The surface normal vector
 * - base_colour: The base colour of the material
 * - metallic: The metallic parameter of the material
 * - roughness: The roughness value after being mapped
 * - f0: Reflectance at normal incidence
 * - clear_coat: The strength of the clear coat effect
 * - clear_coat_roughness: The roughness value for the clearcoat after being mapped
 */
inline float3 ClearCoatBRDF(
    const float3 v,
    const float3 l,
    const float3 n,
    const float3 base_colour,
    const float metallic,
    const float roughness,
    const float3 f0,
    const float clear_coat,
    const float clear_coat_roughness
) {
    // Half unit vector between l and v
    const float3 h = normalize(v + l);

    const float NoV = abs(dot(n, v)) + 0.00005;
    const float NoL = clamp(dot(n, l), 0.0, 1.0);
    const float NoH = clamp(dot(n, h), 0.0, 1.0);
    const float LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    const float  D = D_GGX(NoH, roughness);
    const float  V = V_SmithGGXCorrelatedFast(NoV, NoL, roughness);
    const float3 F = F_SchlickVec(LoH, f0, 1.0);

    // Specular BRDF
    const float3 Fr_nominator = (D * V) * F;
    const float3 Fr_denominator = max(4 * NoV * NoL, 0.001);
    const float3 Fr = Fr_nominator / Fr_denominator;

    // Diffuse BRDF
    const float3 Fd = base_colour * Fd_Burley(NoV, NoL, LoH, roughness);

    // Diffuse contribution
    const float3 kD = (float3(1,1,1) - F) * (1 - metallic);

    const float Dc = D_GGX(NoH, clear_coat_roughness);
    const float Vc = V_SmithGGXCorrelatedFast(NoV, NoL, clear_coat_roughness);
    const float Fc = F_Schlick(LoH, 0.04, 1.0) * clear_coat;

    const float Frc_nominator = (Dc * Vc) * Fc;
    const float Frc_denominator = max(4 * NoV * NoL, 0.001);
    const float Frc = Frc_nominator / Frc_denominator;

    const float cc_energy_loss = 1 - Fc;
    const float3 Or = Fr * cc_energy_loss;
    const float3 Od = Fd * kD;
    const float3 colour = (Od + Or) * cc_energy_loss + Frc;

    return colour;
}

inline float D_Ashikhmin(const float roughness, const float NoH) {
    // Ashikhmin 2007, "Distribution-based BRDFs"
	const float a2 = roughness * roughness;
	const float cos2h = NoH * NoH;
	const float sin2h = max(1.0 - cos2h, 0.0078125); // 2^(-14/2), so sin2h^2 > 0 in fp16
	const float sin4h = sin2h * sin2h;
	const float cot2 = -cos2h / (a2 * sin2h);
	return 1.0 / (PI * (4.0 * a2 + 1.0) * sin4h) * (4.0 * exp(cot2) + sin4h);
}

inline float D_Charlie(const float roughness, const float NoH) {
    // Estevez and Kulla 2017, "Production Friendly Microfacet Sheen BRDF"
    const float invAlpha  = 1.0 / roughness;
    const float cos2h = NoH * NoH;
    const float sin2h = max(1.0 - cos2h, 0.0078125); // 2^(-14/2), so sin2h^2 > 0 in fp16
    return (2.0 + invAlpha) * pow(sin2h, invAlpha * 0.5) / (2.0 * PI);
}

// // specular BRDF
// float D = distributionCloth(roughness, NoH);
// float V = visibilityCloth(NoV, NoL);
// vec3  F = sheenColor;
// vec3 Fr = (D * V) * F;
// 
// // diffuse BRDF
// float diffuse = diffuse(roughness, NoV, NoL, LoH);
// #if defined(MATERIAL_HAS_SUBSURFACE_COLOR)
// // energy conservative wrap diffuse
// diffuse *= saturate((dot(n, light.l) + 0.5) / 2.25);
// #endif
// vec3 Fd = diffuse * pixel.diffuseColor;
// 
// #if defined(MATERIAL_HAS_SUBSURFACE_COLOR)
// // cheap subsurface scatter
// Fd *= saturate(subsurfaceColor + NoL);
// vec3 color = Fd + Fr * NoL;
// color *= (lightIntensity * lightAttenuation) * lightColor;
// #else
// vec3 color = Fd + Fr;
// color *= (lightIntensity * lightAttenuation * NoL) * lightColor;
// #endif

// float3 l = normalize(-lightDirection);
// float NoL = clamp(dot(n, l), 0.0, 1.0);
// 
// // lightIntensity is the illuminance
// // at perpendicular incidence in lux
// float illuminance = lightIntensity * NoL;
// float3 luminance = BSDF(v, l) * illuminance;
