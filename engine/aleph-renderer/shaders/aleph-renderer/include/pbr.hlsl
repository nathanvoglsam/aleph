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
// Calculates the diffuse colour from the base colour and metallic parameters.
//
// We need to use the derived diffuse colour as the input to our BRDFs, not the base colour. This
// function derives that diffuse colour.
//
// Arguments:
//
// - base_colour: The base colour parameter of the material
// - metallic: The metallic parameter of the material
//
inline func CalculateDiffuseColour<T: __BuiltinFloatingPointType>(
    in vector<T, 3> base_colour,
    in T metallic
) -> vector<T, 3> {
    return (T(1.0) - metallic) * base_colour;
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

// Neubelt and Pettineo 2013, "Crafting a Next-gen Material Pipeline for The Order: 1886"
inline func V_Neubelt<T: __BuiltinFloatingPointType>(T NoV, T NoL) -> T {
    return saturate(T(1.0) / (T(4.0) * (NoL + NoV - NoL * NoV)));
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
// Lambertian diffuse Fd term modified for use in a fabric/cloth material model.
//
// This is not a very physically based diffuse BRDF.
//
// - 'w': a term between 0 and 1 defining by how much the diffuse light should wrap around the
//        terminator.
//
// - 'subsurface': artist authored subsurface parameter. is a hack and is not physically based.
//
inline func Fd_LambertCloth<T: __BuiltinFloatingPointType>(in T w, in T NoL, in vector<T, 3> subsurface) -> vector<T, 3> {
    let _0 = NoL + w;
    let _1 = (T(1.0) + w) * (T(1.0) + w);
    let _2 = Fd_Lambert<T>() * saturate(_0 / _1) * saturate(subsurface + NoL);
    return _2;
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
// - diffuse_colour: The diffuse colour of the material
// - metallic: The metallic parameter of the material
// - roughness: The roughness value after being mapped from a perceptual roughness value
// - f0: Reflectance at normal incidence
// 
inline func StandardBRDF(
    in float3 v,
    in float3 l,
    in float3 n,
    in float3 diffuse_colour,
    in float metallic,
    in float roughness,
    in float3 f0
) -> float3 {
    // Half unit vector between l and v
    let h = normalize(v + l);

    let NoV = clamp(abs(dot(n, v)) + 0.00005, 0.0, 1.0);
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    let NoH = clamp(dot(n, h), 0.0, 1.0);
    let LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    let D = D_GGX(NoH, roughness);
    let V = V_SmithGGXCorrelated(NoV, NoL, roughness);
    let F = F_SchlickVec(LoH, f0, 1.0);

    // Specular BRDF
    let Fr = (D * V) * F; // The division by (4 * NoV * NoL) is factored into the V function

    // // Compensation for energy loss due to single-scattering BRDF.
    // // Fr *= float3(1.0) + f0 * (1.0 / dfg.y - 1);

    // Diffuse BRDF
    let Fd = diffuse_colour * Fd_Burley(NoV, NoL, LoH, roughness);

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
// - diffuse_colour: The diffuse colour of the material
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
    in float3 diffuse_colour,
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

    // Recalculate the f0 term, which as given assumes an air-material interface so that it instead
    // assumes an clearcoat-material interface.
    let rootF0 = sqrt(f0);
    let baseF0nominator = (1 - 5 * rootF0) * (1 - 5 * rootF0);
    let baseF0denominator = (5 - rootF0) * (5 - rootF0);
    let baseF0 = baseF0nominator / baseF0denominator;

    // Calculate the different parts of the BRDF
    let D = D_GGX(NoH, roughness);
    let V = V_SmithGGXCorrelated(NoV, NoL, roughness);
    let F = F_SchlickVec(LoH, baseF0, 1.0);

    // Specular BRDF
    let Fr = (D * V) * F; // The division by (4 * NoV * NoL) is factored into the V function

    // // Compensation for energy loss due to single-scattering BRDF.
    // // Fr *= float3(1.0) + f0 * (1.0 / dfg.y - 1);

    // Diffuse BRDF
    let Fd = diffuse_colour * Fd_Burley(NoV, NoL, LoH, roughness);

    // Diffuse contribution
    let kD = (float3(1,1,1) - F) * (1 - metallic);

    // The clear coat layer has no diffuse term
    let Dc = D_GGX(NoH, clear_coat_roughness);
    let Vc = V_SmithGGXCorrelated(NoV, NoL, clear_coat_roughness);
    let Fc = F_Schlick(LoH, 0.04, 1.0) * clear_coat;

    let Frc = (Dc * Vc) * Fc; // The division by (4 * NoV * NoL) is factored into the V function

    // // Compensation for energy loss due to single-scattering BRDF.
    // // Frc *= float3(1.0) + f0 * (1.0 / dfg.y - 1);

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
// - diffuse_colour: The diffuse colour of the material
// - metallic: The metallic parameter of the material
// - roughness: The roughness value after being mapped from a perceptual roughness value
// - f0: Reflectance at normal incidence
// 
inline func ClothBRDF(
    in float3 v,
    in float3 l,
    in float3 n,
    in float3 diffuse_colour,
    in float3 subsurface_colour,
    in float metallic,
    in float roughness,
    in float3 f0
) -> float3 {
    // Half unit vector between l and v
    let h = normalize(v + l);

    let NoV = clamp(abs(dot(n, v)) + 0.00005, 0.0, 1.0);
    let NoL = clamp(dot(n, l), 0.0, 1.0);
    let NoH = clamp(dot(n, h), 0.0, 1.0);
    let LoH = clamp(dot(l, h), 0.0, 1.0);

    // Calculate the different parts of the BRDF
    let D = D_Ashikhmin(roughness, NoH);
    let V = V_Neubelt(NoV, NoL);
    let F = F_SchlickVec(LoH, f0, 1.0); // We could allow artists to define this directly

    // Specular BRDF
    // The division by (4 * NoV * NoL) is not used and an alternative is used, which is 'factored'
    // into the V term (the V term _is_ the divisor in this case 'V_Neubelt')
    let Fr = D * V * F;

    // Diffuse BRDF
    let Fd = diffuse_colour * Fd_LambertCloth(0.5, NoL, subsurface_colour);

    // Diffuse contribution
    let kD = (float3(1,1,1) - F) * (1 - metallic);

    let colour = Fr + (Fd * kD);

    return colour;
}
