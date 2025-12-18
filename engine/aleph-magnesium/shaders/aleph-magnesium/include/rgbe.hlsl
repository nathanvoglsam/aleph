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

// This code has been taken from Microsoft's MiniEngine reference. The license terms are
//
// The MIT License (MIT)
// 
// Copyright (c) 2013-2015 Microsoft
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

// RGBE, aka R9G9B9E5_SHAREDEXP, is an unsigned float HDR pixel format where red, green,
// and blue all share the same exponent.  The color channels store a 9-bit value ranging
// from [0/512, 511/512] which multiplies by 2^Exp and Exp ranges from [-15, 16].
// Floating point specials are not encoded.
func PackRGBE(float3 rgb) -> uint
{
    // To determine the shared exponent, we must clamp the channels to an expressible range
    const float kMaxVal = asfloat(0x477F8000); // 1.FF x 2^+15
    const float kMinVal = asfloat(0x37800000); // 1.00 x 2^-16

    // Non-negative and <= kMaxVal
    rgb = clamp(rgb, 0, kMaxVal);

    // From the maximum channel we will determine the exponent.  We clamp to a min value
    // so that the exponent is within the valid 5-bit range.
    float MaxChannel = max(max(kMinVal, rgb.r), max(rgb.g, rgb.b));

    // 'Bias' has to have the biggest exponent plus 15 (and nothing in the mantissa).  When
    // added to the three channels, it shifts the explicit '1' and the 8 most significant
    // mantissa bits into the low 9 bits.  IEEE rules of float addition will round rather
    // than truncate the discarded bits.  Channels with smaller natural exponents will be
    // shifted further to the right (discarding more bits).
    float Bias = asfloat((asuint(MaxChannel) + 0x07804000) & 0x7F800000);

    // Shift bits into the right places
    uint3 RGB = asuint(rgb + Bias);
    uint E = (asuint(Bias) << 4) + 0x10000000;
    return E | RGB.b << 18 | RGB.g << 9 | (RGB.r & 0x1FF);
}

func UnpackRGBE(uint p) -> float3
{
    float3 rgb = uint3(p, p >> 9, p >> 18) & 0x1FF;
    return ldexp(rgb, (int)(p >> 27) - 24);
}
