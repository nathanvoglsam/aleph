//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::KTXDocument;
use std::io::Cursor;

static CUBEMAP_YOKOHAMA_ASTC_8X8_SRGB: &'static [u8] =
    include_bytes!("../test_images/cubemap_yokohama_astc_8x8_srgb.ktx2");
static ETC1: &'static [u8] = include_bytes!("../test_images/etc1.ktx2");
static ETC2_RGB: &'static [u8] = include_bytes!("../test_images/etc2-rgb.ktx2");
static ETC2_RGBA1: &'static [u8] = include_bytes!("../test_images/etc2-rgba1.ktx2");
static ETC2_RGBA8: &'static [u8] = include_bytes!("../test_images/etc2-rgba8.ktx2");
static ETC2_S_RGB: &'static [u8] = include_bytes!("../test_images/etc2-sRGB.ktx2");
static ETC2_S_RGBA1: &'static [u8] = include_bytes!("../test_images/etc2-sRGBa1.ktx2");
static ETC2_S_RGBA8: &'static [u8] = include_bytes!("../test_images/etc2-sRGBa8.ktx2");
static HI_MARK: &'static [u8] = include_bytes!("../test_images/hi_mark.ktx2");
static KTX_APP_U: &'static [u8] = include_bytes!("../test_images/ktx_app-u.ktx2");
static PATTERN_02_BC2: &'static [u8] = include_bytes!("../test_images/pattern_02_bc2.ktx2");
static RGB_MIPMAP_REFERENCE_U: &'static [u8] =
    include_bytes!("../test_images/rgb-mipmap-reference-u.ktx2");
static TEXTUREARRAY_ASTC_8X8_UNORM: &'static [u8] =
    include_bytes!("../test_images/texturearray_astc_8x8_unorm.ktx2");
static TEXTUREARRAY_BC3_UNORM: &'static [u8] =
    include_bytes!("../test_images/texturearray_bc3_unorm.ktx2");
static TEXTUREARRAY_ETC2_UNORM: &'static [u8] =
    include_bytes!("../test_images/texturearray_etc2_unorm.ktx2");

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
fn test_read_image_data() {
    let mut ktx = KTXDocument::from_slice(RGB_MIPMAP_REFERENCE_U).unwrap();

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
