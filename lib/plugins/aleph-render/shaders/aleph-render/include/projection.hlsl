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

//
// Unprojects the given point, assuming that the point was projected with a reversed Z matrix with
// an infinite far plane.
//
// ## Parameters
//
// let t = tan(vertical_fov / 2.0);
// let a = 1.0 / t;
// let b = a / aspect_ratio;
//
// These could be pulled directly from the projection matrix as m._m00 = a and m._m11 = b and
// m._32 = near. Again, assuming a reverse Z infinite projeciton matrix for a right-hand-y-up source
// coordinate system.
//
inline func UnprojectPoint<T : __BuiltinFloatingPointType>(
    in T a,
    in T b,
    in T near,
    in vector<T, 3> point
) -> vector<T, 3> {
    let uZ = -near / point.z;
    let uX = (-uZ * point.x) / a;
    let uY = (-uZ * point.y) / b;
    return vector<T, 3>(uX, uY, uZ);
}

//
// Unprojects the given point, assuming that the point was projected with the given pure projection
// matrix. This makes no assumptions about the input and derives 'a', 'b' and 'near' directly from
// the projection parameters. Obviously this requires having them available at the call site.
//
// ## Info
//
// a, b, and near can be trivially extracted from a compatible matrix. If you have a pure projection
// matrix available use [UnprojectPointWithMatrix] instead as it avoids a 'tan' call and two fp
// divisions.
//
inline func UnprojectPointWithParameters<T : __BuiltinFloatingPointType>(
    in T verticalFov,
    in T aspectRatio,
    in T near,
    in vector<T, 3> point
) -> vector<T, 3> {
    let t = tan<T>(verticalFov / T(2));
    let a = T(1) / t;
    let b = a / aspectRatio;
    return UnprojectPoint<T>(a, b, near, point);
}

//
// Unprojects the given point, assuming that the point was projected with the given pure projection
// matrix. This follows the same assumption as [UnprojectPoint]. That is: We assume the given matrix
// contains a reverse Z infinite projection matrix for a right-hand-y-up source coordinate system.
//
// ## Warning
//
// 'proj' MUST be a pure projection matrix for it to get the correct parameters to unproject the
// point. If you don't have a pure matrix on hand you can use [UnprojectPoint] and derive the inputs
// from the provided equations.
//
inline func UnprojectPointWithMatrix<T : __BuiltinFloatingPointType>(
    in matrix<T, 4, 4> proj,
    in vector<T, 3> point
) -> vector<T, 3> {
    let a = proj._m00;
    let b = proj._m11;
    let near = proj._m32;
    return UnprojectPoint(a, b, near, point);
}
