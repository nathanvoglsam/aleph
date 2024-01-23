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

// 
// Struct that describes a box for use with the TraceRayAgainstBox function family.
// 
// Not all the members may be used by the tracing function, depending on the settings for the
// function. Check the documentation for the tracing functions so you can avoid calculating them.
// 
// Members:
// 
// - center: The center point of the box in some coordinate space (e.g world space or view space).
// - radius: The half side lengths of the box on each axis.
// - inv_radius: The inverse of 'radius'. Not always used, check the TraceRayAgainstBox docs.
// - rotation: Rotation matrix defining the orientation of the box. Not always used.
// 
struct TracingBox {
    float3 center;
    float3 radius;
    float3 inv_radius;
    float3x3 rotation;
};

// 
// A function that traces a ray against a single box, returning whether an intersection occurred,
// the distance to the intersection and the normal of the intersection.
// 
// Arguments:
// - box: The box to trace against
// - ray: The ray to trace
// - out distance: Filled with the distance to the intersection, if there was one.
// - out normal: Filled with the normal of the intersection, if there was one.
// - can_start_in_box: Whether the ray can start inside the box. Recommended use for this is as a
//                     compile time constant to switch between different implementations. If
//                     this is false then Box.inv_radius won't be used so initialization of it can
//                     be skipped.
// - is_oriented: Whether the box can be rotated or not. Recommended use for this is as a compile
//                time constant to switch between different implementations. If this is false then
//                the function will assume the box is axis aligned and will ignore the Box.rotation
//                member meaning initializing it can be skipped.
// - inv_ray_direction: The inverse of Ray.direction. Needed when is_oriented is false. We leave
//                      creating this to the caller as they could precalculate this to skip doing
//                      the divide in the shader.
// 
// This is based on the implementation described in this paper:
// A Ray-Box Intersection Algorithm and Efficient Dynamic Voxel Rendering by Alexander Majercik,
// Cyril Crassin, Peter Shirley, and Morgan McGuire
// 
bool TraceRayAgainstBox(
    const TracingBox box,
    const Ray ray,
    out float distance,
    out float3 normal,
    const bool can_start_in_box,
    const bool is_oriented,
    const float3 inv_ray_direction
) {
    // I don't know how this works, it's described in the reference paper. I removed all the
    // comments because they aren't really useful to me. If you want to know how this works, look
    // up the paper.

    const float3 ray_origin = mul(box.rotation, (ray.origin - box.center));

    float3 ray_direction;
    if (is_oriented) {
        ray_direction = mul(box.rotation,ray.direction);
    } else {
        ray_direction = ray.direction;
    }

    float winding;
    if (can_start_in_box) {
        winding = (MaxComponent(abs(ray_origin) * box.inv_radius) < 1.0) ? -1.0 : 1.0;
    }
    else {
        winding = 1.0;
    }

    float3 sgn = -sign(ray_direction);

    float3 distance_to_plane = box.radius * winding * sgn - ray_origin;
    if (is_oriented) {
        distance_to_plane /= ray_direction;
    }
    else {
        distance_to_plane *= inv_ray_direction;
    }

#   define TEST(U, VW) (distance_to_plane.U >= 0.0) && all(abs(ray_origin.VW + ray_direction.VW * distance_to_plane.U) < box.radius.VW)
    bool test0 = TEST(x, yz);
    bool test1 = TEST(y, zx);
    bool test2 = TEST(z, xy);

    sgn = test0 ? float3(sgn.x, 0.0, 0.0) : (test1 ? float3(0.0, sgn.y, 0.0) : float3(0.0, 0.0, test2 ? sgn.z : 0.0));
#   undef TEST

    distance = (sgn.x != 0.0) ? distance_to_plane.x : ((sgn.y != 0.0) ? distance_to_plane.y : distance_to_plane.z);

    if (is_oriented) {
        normal = mul(sgn, box.rotation);
    }
    else {
        normal = sgn;
    }

    return (sgn.x != 0) || (sgn.y != 0) || (sgn.z != 0);
}

// 
// A wrapper around TraceRayAgainstBox. This function can be used for tracing against axis-aligned
// boxes where the ray will always originate from outside the box
// 
bool TraceOutsideRayAgainstAABox(
    const TracingBox box,
    const Ray ray,
    out float distance,
    out float3 normal,
    const float3 inv_ray_direction,
) {
    return TraceRayAgainstBox(box, ray, distance, normal, false, false, inv_ray_direction);
}

// 
// A wrapper around TraceRayAgainstBox. This function can be used for tracing against axis-aligned
// boxes where the ray may originate from inside the box
// 
bool TraceRayAgainstAABox(
        const TracingBox box,
        const Ray ray,
        out float distance,
        out float3 normal,
        const float3 inv_ray_direction,
) {
    return TraceRayAgainstBox(box, ray, distance, normal, true, false, inv_ray_direction);
}

// 
// A wrapper around TraceRayAgainstBox. This function can be used for tracing against oriented
// boxes where the ray will always originate from outside the box
// 
bool TraceOutsideRayAgainstOrientedBox(
        const TracingBox box,
        const Ray ray,
        out float distance,
        out float3 normal,
        const float3 inv_ray_direction,
) {
    return TraceRayAgainstBox(box, ray, distance, normal, false, true, inv_ray_direction);
}

// 
// A wrapper around TraceRayAgainstBox. This function can be used for tracing against oriented
// boxes where the ray may originate from inside the box
// 
bool TraceRayAgainstOrientedBox(
        const TracingBox box,
        const Ray ray,
        out float distance,
        out float3 normal,
        const float3 inv_ray_direction,
) {
    return TraceRayAgainstBox(box, ray, distance, normal, true, true, inv_ray_direction);
}