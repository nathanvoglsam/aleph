//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

/*
 * Saturate a FP16 (half precision) float to be in the 0-1 range but never actually 0
 */
inline float SaturateFP16(float val) {
    return clamp(val, 0.089, 1.0);
}

/*
 * Saturate a FP32 (full precision) float to be in the 0-1 range but never actually 0 
 */
inline float SaturateFP32(float val) {
    return clamp(val, 0.045, 1.0);
}
