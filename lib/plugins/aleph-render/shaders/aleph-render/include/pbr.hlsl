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

#include "common.hlsl"

// float3 energyCompensation = 1.0 + f0 * (1.0 / dfg.y - 1.0);
// // Scale the specular lobe to account for multiscattering
// Fr *= pixel.energyCompensation;

//
// Calculates the attenuation factor for a point light that should be applied to
//
inline func PointLightAttenuation<T: __BuiltinFloatingPointType>(in T distance_squared) -> T {
    return T(1) / (T(4) * T(PI) * distance_squared);
}

//
// Given the output of a brdf, and a set of point light parameters, calculate the final light
// contribution.
//
inline func EvaluatePointLight<T: __BuiltinFloatingPointType>(
    in vector<T, 3> brdf,
    in T lumens,
    in T distance_squared,
    in T NoL
) -> vector<T, 3> {
    let attenuation = PointLightAttenuation<T>(distance_squared);
    return brdf * (lumens * attenuation * NoL);
}

// 
// Performs a remapping of the roughness parameter. It makes the roughness parameter seem more
// linear when being tweaked so it's a bit more intuitive to work with.
// 
inline func RemapRoughness<T: __BuiltinFloatingPointType>(in T perceptual_roughness) -> T {
    return Saturate<T>(perceptual_roughness * perceptual_roughness);
}

//
// Calculates the 'at' term for use in the anisotropic material model
//
inline func CalculateAT<T: __BuiltinFloatingPointType>(
    in T roughness,
    in T anisotropy
) -> T {
    return max(roughness * (T(1.0) + anisotropy), T(0.001));
}

//
// Calculates the 'ab' term for use in the anisotropic material model
//
inline func CalculateAB<T: __BuiltinFloatingPointType>(
    in T roughness,
    in T anisotropy
) -> T {
    return max(roughness * (T(1.0) - anisotropy), T(0.001));
}

//
// Calculates the F0 value from the base colour, metallic and reflectance
//
// Arguments:
//
// - base_colour: The base colour parameter of the material
// - metallic: The metallic parameter of the material
// - reflectance: The reflectance parameter of the material
//
inline func CalculateF0<T: __BuiltinFloatingPointType>(
    in vector<T, 3> base_colour,
    in T metallic,
    in T reflectance
) -> vector<T, 3> {
    return T(0.16) * reflectance * reflectance * (T(1.0) - metallic) + base_colour * metallic;
}

//
// The GGX D term
//
inline func D_GGX<T: __BuiltinFloatingPointType>(
    in T NoH,
    in T roughness
) -> T {
    let a = NoH * roughness;
    let k = roughness / (T(1.0) - NoH * NoH + a * a);
    return k * k * (T(1.0) / T(PI));
}

// 
// The GGX D term with support for anisotropic materials
// 
inline func D_GGX_Anisotropic(
    in float NoH,
    in float3 h,
    in float3 t,
    in float3 b,
    in float at,
    in float ab
) -> float {
    let ToH = dot(t, h);
    let BoH = dot(b, h);
    let a2 = at * ab;
    let v = float3(ab * ToH, at * BoH, a2 * NoH);
    let v2 = dot(v, v);
    let w2 = a2 / v2;
    return a2 * w2 * w2 * (1.0 / PI);
}

//
// The SmithGGXCorrelated V term
//
inline func V_SmithGGXCorrelated<T: __BuiltinFloatingPointType>(
    in T NoV,
    in T NoL,
    in T roughness
) -> T {
    let a2 = roughness * roughness;
    let GGXV = NoL * sqrt(NoV * NoV * (T(1.0) - a2) + a2);
    let GGXL = NoV * sqrt(NoL * NoL * (T(1.0) - a2) + a2);
    return T(0.5) / (GGXV + GGXL);
}

// 
// The SmithGGXCorrelated V term with support for anisotropic materials
// 
inline func V_SmithGGXCorrelated_Anisotropic<T: __BuiltinFloatingPointType>(
    in T at,
    in T ab,
    in T ToV,
    in T BoV,
    in T ToL,
    in T BoL,
    in T NoV, 
    in T NoL
) -> T {
    let lambdaV = NoL * length(vector<T, 3>(at * ToV, ab * BoV, NoV));
    let lambdaL = NoV * length(vector<T, 3>(at * ToL, ab * BoL, NoL));
    let v = T(0.5) / (lambdaV + lambdaL);
    return Saturate<T>(v);
}

//
// The SmithGGXCorrelated V term. Optimized for speed by trading accuracy after noting that all
// terms under the square roots are in the 0..1 range. This approximation is wrong but faster so
// pick your poison.
//
inline func V_SmithGGXCorrelatedFast<T: __BuiltinFloatingPointType>(
    in T NoV,
    in T NoL,
    in T roughness
) -> T {
    let a = roughness;
    let GGXV = NoL * (NoV * (T(1.0) - a) + a);
    let GGXL = NoV * (NoL * (T(1.0) - a) + a);
    return T(0.5) / (GGXV + GGXL);
}

//
// The Schlick F term, using a float3 f0 arg
//
inline func F_SchlickVec<T: __BuiltinFloatingPointType>(
    in T u,
    in vector<T, 3> f0,
    in T f90
) ->vector<T, 3> {
    return f0 + (vector<T, 3>(f90,f90,f90) - f0) * pow(T(1.0) - u, T(5.0));
}

//
// The Schlick F term, using a scalar f0 arg
//
inline func F_Schlick<T: __BuiltinFloatingPointType>(
    in T u,
    in T f0,
    in T f90
) -> T {
    return f0 + (f90 - f0) * pow(T(1.0) - u, T(5.0));
}

//
// Lambertian diffuse Fd term. Faster and easier but not as accurate
//
inline func Fd_Lambert<T: __BuiltinFloatingPointType>() -> T {
    return T(1.0) / T(PI);
}

//
// Disney's Burley diffuse Fd term. Looks good but slower and harder to work with
//
inline func Fd_Burley<T: __BuiltinFloatingPointType>(
    in T NoV,
    in T NoL,
    in T LoH,
    in T roughness
) -> T {
    let f90 = T(0.5) + T(2.0) * roughness * LoH * LoH;
    let light_scatter = F_Schlick(NoL, T(1.0), f90);
    let view_scatter = F_Schlick(NoV, T(1.0), f90);
    return light_scatter * view_scatter * (T(1.0) / T(PI));
}

// 
// A standard hard surface PBR BRDF. This does not account for light attenuation and intensity.
// 
// Returns the combined output of the diffuse and specular term prior to lighting
// 
// Arguments:
// 
// - v: The view unit vector
// - l: The incident light unit vector
// - n: The surface normal vector
// - base_colour: The base colour of the material
// - metallic: The metallic parameter of the material
// - roughness: The roughness value after being mapped from a perceptual roughness value
// - f0: Reflectance at normal incidence
// 
inline func StandardBRDF(
    in float3 v,
    in float3 l,
    in float3 n,
    in float3 base_colour,
    in float metallic,
    in float roughness,
    in float3 f0
) -> float3 {
    // Half unit vector between l and v
    let h = normalize(v + l);

    let NoV = abs(dot(n, v)) + 0.00005;
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    let NoH = clamp(dot(n, h), 0.0, 1.0);
    let LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    let D = D_GGX(NoH, roughness);
    let V = V_SmithGGXCorrelatedFast(NoV, NoL, roughness);
    let F = F_SchlickVec(LoH, f0, 1.0);

    // Specular BRDF
    let Fr_nominator = (D * V) * F;
    let Fr_denominator = max(4 * NoV * NoL, 0.001);
    let Fr = Fr_nominator / Fr_denominator;

    // Diffuse BRDF
    let Fd = base_colour * Fd_Burley(NoV, NoL, LoH, roughness);

    // Diffuse contribution
    let kD = (float3(1,1,1) - F) * (1 - metallic);

    let colour = Fr + (Fd * kD);

    return colour;
}

inline func V_Kelemen<T: __BuiltinFloatingPointType>(T LoH) -> T {
    return T(0.25) / (LoH * LoH);
}

// 
// A standard hard surface PBR BRDF with an extra clear coat term.
// 
// Returns the combined output of the diffuse and specular term prior to lighting
// 
// Arguments:
// 
// - v: The view unit vector
// - l: The incident light unit vector
// - n: The surface normal vector
// - base_colour: The base colour of the material
// - metallic: The metallic parameter of the material
// - roughness: The roughness value after being mapped
// - f0: Reflectance at normal incidence
// - clear_coat: The strength of the clear coat effect
// - clear_coat_roughness: The roughness value for the clearcoat after being mapped
//
inline func ClearCoatBRDF(
    in float3 v,
    in float3 l,
    in float3 n,
    in float3 base_colour,
    in float metallic,
    in float roughness,
    in float3 f0,
    in float clear_coat,
    in float clear_coat_roughness
) -> float3 {
    // Half unit vector between l and v
    let h = normalize(v + l);

    let NoV = abs(dot(n, v)) + 0.00005;
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    let NoH = clamp(dot(n, h), 0.0, 1.0);
    let LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    let D = D_GGX(NoH, roughness);
    let V = V_SmithGGXCorrelatedFast(NoV, NoL, roughness);
    let F = F_SchlickVec(LoH, f0, 1.0);

    // Specular BRDF
    let Fr_nominator = (D * V) * F;
    let Fr_denominator = max(4 * NoV * NoL, 0.001);
    let Fr = Fr_nominator / Fr_denominator;

    // Diffuse BRDF
    let Fd = base_colour * Fd_Burley(NoV, NoL, LoH, roughness);

    // Diffuse contribution
    let kD = (float3(1,1,1) - F) * (1 - metallic);

    let Dc = D_GGX(NoH, clear_coat_roughness);
    let Vc = V_SmithGGXCorrelatedFast(NoV, NoL, clear_coat_roughness);
    let Fc = F_Schlick(LoH, 0.04, 1.0) * clear_coat;

    let Frc_nominator = (Dc * Vc) * Fc;
    let Frc_denominator = max(4 * NoV * NoL, 0.001);
    let Frc = Frc_nominator / Frc_denominator;

    let cc_energy_loss = 1 - Fc;
    let Or = Fr * cc_energy_loss;
    let Od = Fd * kD;
    let colour = (Od + Or) * cc_energy_loss + Frc;

    return colour;
}

inline func D_Ashikhmin(in float roughness, in float NoH) -> float {
    // Ashikhmin 2007, "Distribution-based BRDFs"
	let a2 = roughness * roughness;
	let cos2h = NoH * NoH;
	let sin2h = max(1.0 - cos2h, 0.0078125); // 2^(-14/2), so sin2h^2 > 0 in fp16
	let sin4h = sin2h * sin2h;
	let cot2 = -cos2h / (a2 * sin2h);
	return 1.0 / (PI * (4.0 * a2 + 1.0) * sin4h) * (4.0 * exp(cot2) + sin4h);
}

inline func D_Charlie(in float roughness, in float NoH) -> float {
    // Estevez and Kulla 2017, "Production Friendly Microfacet Sheen BRDF"
    let invAlpha  = 1.0 / roughness;
    let cos2h = NoH * NoH;
    let sin2h = max(1.0 - cos2h, 0.0078125); // 2^(-14/2), so sin2h^2 > 0 in fp16
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
