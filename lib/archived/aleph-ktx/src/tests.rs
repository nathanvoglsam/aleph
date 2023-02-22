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

use crate::KTXDocument;
use crate::KTXReadError;
use std::io::Cursor;

static CUBEMAP_YOKOHAMA_ASTC_8X8_SRGB: &[u8] =
    include_bytes!("../test_images/cubemap_yokohama_astc_8x8_srgb.ktx2");
static ETC1: &[u8] = include_bytes!("../test_images/etc1.ktx2");
static ETC2_RGB: &[u8] = include_bytes!("../test_images/etc2-rgb.ktx2");
static ETC2_RGBA1: &[u8] = include_bytes!("../test_images/etc2-rgba1.ktx2");
static ETC2_RGBA8: &[u8] = include_bytes!("../test_images/etc2-rgba8.ktx2");
static ETC2_S_RGB: &[u8] = include_bytes!("../test_images/etc2-sRGB.ktx2");
static ETC2_S_RGBA1: &[u8] = include_bytes!("../test_images/etc2-sRGBa1.ktx2");
static ETC2_S_RGBA8: &[u8] = include_bytes!("../test_images/etc2-sRGBa8.ktx2");
static HI_MARK: &[u8] = include_bytes!("../test_images/hi_mark.ktx2");
static KTX_APP_U: &[u8] = include_bytes!("../test_images/ktx_app-u.ktx2");
static PATTERN_02_BC2: &[u8] = include_bytes!("../test_images/pattern_02_bc2.ktx2");
static RGB_MIPMAP_REFERENCE_U: &[u8] = include_bytes!("../test_images/rgb-mipmap-reference-u.ktx2");
static TEXTUREARRAY_ASTC_8X8_UNORM: &[u8] =
    include_bytes!("../test_images/texturearray_astc_8x8_unorm.ktx2");
static TEXTUREARRAY_BC3_UNORM: &[u8] = include_bytes!("../test_images/texturearray_bc3_unorm.ktx2");
static TEXTUREARRAY_ETC2_UNORM: &[u8] =
    include_bytes!("../test_images/texturearray_etc2_unorm.ktx2");

//static INCORRECT_MIP_LAYOUT_AND_PADDING: &'static [u8] =
//    include_bytes!("../test_images/incorrect_mip_layout_and_padding.ktx2");
static INVALID_FACE_COUNT_AND_PADDING: &[u8] =
    include_bytes!("../test_images/invalid_face_count_and_padding.ktx2");

#[test]
fn test_validates_files() {
    let file_list = [
        CUBEMAP_YOKOHAMA_ASTC_8X8_SRGB,
        ETC1,
        ETC2_RGB,
        ETC2_RGBA1,
        ETC2_RGBA8,
        ETC2_S_RGB,
        ETC2_S_RGBA1,
        ETC2_S_RGBA8,
        HI_MARK,
        KTX_APP_U,
        PATTERN_02_BC2,
        RGB_MIPMAP_REFERENCE_U,
        TEXTUREARRAY_ASTC_8X8_UNORM,
        TEXTUREARRAY_BC3_UNORM,
        TEXTUREARRAY_ETC2_UNORM,
    ];

    file_list.iter().for_each(|file| {
        let _ktx = KTXDocument::from_slice(file).unwrap();
    });
}

#[test]
fn test_invalid_files() {
    //let error = KTXDocument::from_slice(INCORRECT_MIP_LAYOUT_AND_PADDING).err().unwrap();
    //if !matches!(error, KTXReadError::InvalidFaceCount(4)) {
    //    panic!("Loading failed for the wrong reason");
    //}

    let error = KTXDocument::from_slice(INVALID_FACE_COUNT_AND_PADDING)
        .err()
        .unwrap();
    if !matches!(error, KTXReadError::InvalidFaceCount(4)) {
        panic!("Loading failed for the wrong reason");
    }
}

#[test]
fn test_lookup_key() {
    let doc = KTXDocument::from_slice(CUBEMAP_YOKOHAMA_ASTC_8X8_SRGB).unwrap();
    let _value = doc.lookup_key("KTXorientation").unwrap().unwrap();
    let _value = doc.lookup_key("KTXwriter").unwrap().unwrap();

    assert!(doc.lookup_key("AKeyThatDoesntExist").unwrap().is_none());
}

#[test]
fn test_read_image_data() {
    let ktx = KTXDocument::from_slice(RGB_MIPMAP_REFERENCE_U).unwrap();

    let level_vals: [[u8; 3]; 7] = [
        [255, 0, 0],
        [255, 116, 0],
        [255, 255, 0],
        [0, 255, 0],
        [0, 0, 255],
        [0, 255, 255],
        [255, 0, 255],
    ];

    for (level, expected_vals) in level_vals.iter().enumerate() {
        let image_bytes = ktx.image_bytes(level).unwrap();
        let mut cursor = Cursor::new(vec![0u8; image_bytes]);

        ktx.read_image(0, 0, level, &mut cursor).unwrap();

        let data = cursor.into_inner();

        data.chunks(3).for_each(|pixel_vals| {
            assert_eq!(pixel_vals, expected_vals);
        });
    }
}
