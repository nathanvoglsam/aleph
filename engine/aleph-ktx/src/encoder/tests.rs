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

use aleph_vk_format::VkFormat;

use crate::{DocumentDescription, DocumentType, KTXDocument, ENCODER_NAME};

#[test]
pub fn test_image_1d_round_trip() {
    #[rustfmt::skip]
    static DATA: [u8; 1] = [
        1
    ];
    let levels = [DATA.as_slice()];

    let mut desc = DocumentDescription::new();
    desc.format(VkFormat::R8_UNORM);
    desc.mip_levels(1);
    desc.image_1d(1, &levels);

    let mut data = Vec::new();
    desc.write(&mut data).unwrap();

    let document = KTXDocument::from_slice(&data).unwrap();
    assert_eq!(document.format(), VkFormat::R8_UNORM);
    assert_eq!(document.requests_mip_generation(), false);
    assert_eq!(document.document_type(), DocumentType::Image1D);
    assert_eq!(document.width(), 1);
    assert_eq!(document.height(), 1);
    assert_eq!(document.depth(), 1);
    assert_eq!(document.face_num(), 1);
    assert_eq!(document.level_num(), 1);
    assert_eq!(document.layer_num(), 1);

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = document.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    let info = document.get_level_info(0).unwrap();
    let level = &data[info.to_slice_range()];
    assert_eq!(level.len(), levels[0].len());
    assert_eq!(level, levels[0]);
}

#[test]
pub fn test_image_2d_round_trip() {
    #[rustfmt::skip]
    static DATA: [u8; 4] = [
        1, 2,
        3, 4
    ];
    let levels = [DATA.as_slice()];

    let mut desc = DocumentDescription::new();
    desc.format(VkFormat::R8_UNORM);
    desc.mip_levels(1);
    desc.image_2d(2, 2, &levels);

    let mut data = Vec::new();
    desc.write(&mut data).unwrap();

    let document = KTXDocument::from_slice(&data).unwrap();
    assert_eq!(document.format(), VkFormat::R8_UNORM);
    assert_eq!(document.requests_mip_generation(), false);
    assert_eq!(document.document_type(), DocumentType::Image2D);
    assert_eq!(document.width(), 2);
    assert_eq!(document.height(), 2);
    assert_eq!(document.depth(), 1);
    assert_eq!(document.face_num(), 1);
    assert_eq!(document.level_num(), 1);
    assert_eq!(document.layer_num(), 1);

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = document.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    let info = document.get_level_info(0).unwrap();
    let level = &data[info.to_slice_range()];
    assert_eq!(level.len(), levels[0].len());
    assert_eq!(level, levels[0]);

    std::fs::write("./thing.ktx2", data).unwrap();
}

#[test]
pub fn test_image_2d_mips_round_trip() {
    #[rustfmt::skip]
    static DATA_0: [u8; 4] = [
        1, 2,
        3, 4
    ];
    #[rustfmt::skip]
    static DATA_1: [u8; 1] = [
        5
    ];
    let levels = [DATA_0.as_slice(), DATA_1.as_slice()];

    let mut desc = DocumentDescription::new();
    desc.format(VkFormat::R8_UNORM);
    desc.mip_levels(2);
    desc.image_2d(2, 2, &levels);

    let mut data = Vec::new();
    desc.write(&mut data).unwrap();

    let document = KTXDocument::from_slice(&data).unwrap();
    assert_eq!(document.format(), VkFormat::R8_UNORM);
    assert_eq!(document.requests_mip_generation(), false);
    assert_eq!(document.document_type(), DocumentType::Image2D);
    assert_eq!(document.width(), 2);
    assert_eq!(document.height(), 2);
    assert_eq!(document.depth(), 1);
    assert_eq!(document.face_num(), 1);
    assert_eq!(document.level_num(), 2);
    assert_eq!(document.layer_num(), 1);

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = document.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    let info = document.get_level_info(0).unwrap();
    let level = &data[info.to_slice_range()];
    assert_eq!(level.len(), levels[0].len());
    assert_eq!(level, levels[0]);

    let info = document.get_level_info(1).unwrap();
    let level = &data[info.to_slice_range()];
    assert_eq!(level.len(), levels[1].len());
    assert_eq!(level, levels[1]);

    std::fs::write("./thing.ktx2", data).unwrap();
}

#[test]
pub fn test_image_2d_array_round_trip() {
    #[rustfmt::skip]
    static DATA_0: [u8; 4] = [
        1, 2,
        3, 4
    ];
    #[rustfmt::skip]
    static DATA_1: [u8; 4] = [
        5, 6,
        7, 8
    ];
    #[rustfmt::skip]
    static DATA_2: [u8; 4] = [
        9, 10,
        11, 12
    ];
    let layer_0 = [DATA_0.as_slice()];
    let layer_1 = [DATA_1.as_slice()];
    let layer_2 = [DATA_2.as_slice()];
    let layers = [layer_0.as_slice(), layer_1.as_slice(), layer_2.as_slice()];

    let mut desc = DocumentDescription::new();
    desc.format(VkFormat::R8_UNORM);
    desc.mip_levels(1);
    desc.image_2d_array(2, 2, &layers);

    let mut data = Vec::new();
    desc.write(&mut data).unwrap();

    let document = KTXDocument::from_slice(&data).unwrap();
    assert_eq!(document.format(), VkFormat::R8_UNORM);
    assert_eq!(document.requests_mip_generation(), false);
    assert_eq!(document.document_type(), DocumentType::Array2D);
    assert_eq!(document.width(), 2);
    assert_eq!(document.height(), 2);
    assert_eq!(document.depth(), 1);
    assert_eq!(document.face_num(), 1);
    assert_eq!(document.level_num(), 1);
    assert_eq!(document.layer_num(), 3);

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = document.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    let info = document.get_level_info(0).unwrap();
    let level = &data[info.to_slice_range()];
    let la0 = &level[0..4];
    let la1 = &level[4..8];
    let la2 = &level[8..12];
    assert_eq!(la0.len(), layer_0[0].len());
    assert_eq!(la0, layer_0[0]);
    assert_eq!(la1.len(), layer_1[0].len());
    assert_eq!(la1, layer_1[0]);
    assert_eq!(la2.len(), layer_2[0].len());
    assert_eq!(la2, layer_2[0]);

    std::fs::write("./thing.ktx2", data).unwrap();
}

#[test]
pub fn test_image_3d_round_trip() {
    #[rustfmt::skip]
    static DATA: [u8; 8] = [
        1, 2,
        3, 4,
        5, 6,
        7, 8
    ];
    let levels = [DATA.as_slice()];

    let mut desc = DocumentDescription::new();
    desc.format(VkFormat::R8_UNORM);
    desc.mip_levels(1);
    desc.image_3d(2, 2, 2, &levels);

    let mut data = Vec::new();
    desc.write(&mut data).unwrap();

    let document = KTXDocument::from_slice(&data).unwrap();
    assert_eq!(document.format(), VkFormat::R8_UNORM);
    assert_eq!(document.requests_mip_generation(), false);
    assert_eq!(document.document_type(), DocumentType::Image3D);
    assert_eq!(document.width(), 2);
    assert_eq!(document.height(), 2);
    assert_eq!(document.depth(), 2);
    assert_eq!(document.face_num(), 1);
    assert_eq!(document.level_num(), 1);
    assert_eq!(document.layer_num(), 1);

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = document.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    let info = document.get_level_info(0).unwrap();
    let level = &data[info.to_slice_range()];
    assert_eq!(level.len(), levels[0].len());
    assert_eq!(level, levels[0]);
}
