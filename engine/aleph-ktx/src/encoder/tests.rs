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

use std::io::{Cursor, Read, Seek};

use aleph_vk_format::VkFormat;

use crate::{
    DocumentDescription, DocumentType, KTXDocument, KTXReadError, SuperCompressionScheme,
    ENCODER_NAME,
};

#[test]
fn test_round_trip_files_cts() {
    use std::fs::read_dir;

    let dir = read_dir("./test_images/cts/ktx2").unwrap();
    for entry in dir {
        let entry = entry.unwrap();

        let path = entry.path();
        let file_type = entry
            .file_type()
            .expect(&format!("Get FileType for '{}'", path.display()));

        if file_type.is_file() {
            println!("Trying to round trip '{}'", path.display());
            let file = std::fs::read(&path).unwrap();
            let read = Cursor::new(file.as_slice());
            let doc = KTXDocument::from_reader(read).unwrap();
            round_trip_document(file.as_slice(), doc).unwrap();
        }
    }
}

#[test]
fn test_round_trip_files_samples() {
    use std::fs::read_dir;

    let dir = read_dir("./test_images/cts/ktx2_sample").unwrap();
    for entry in dir {
        let entry = entry.unwrap();

        let path = entry.path();
        let file_type = entry
            .file_type()
            .expect(&format!("Get FileType for '{}'", path.display()));

        if file_type.is_file() {
            println!("Trying to round trip '{}'", path.display());
            let file = std::fs::read(&path).unwrap();
            let read = Cursor::new(file.as_slice());
            let doc = KTXDocument::from_reader(read).unwrap();
            round_trip_document(file.as_slice(), doc).unwrap();
        }
    }
}

fn round_trip_document<R: Read + Seek>(
    file: &[u8],
    doc: KTXDocument<R>,
) -> Result<(), KTXReadError> {
    if doc.format() == VkFormat::UNDEFINED {
        // We don't support writing undefined format files, which may include some supercompressed
        // files
        return Ok(());
    }

    if doc.super_compression_scheme() != SuperCompressionScheme::NONE {
        // We don't support writing supercompressed data so bail here and skip the file
        return Ok(());
    }

    let mut desc = DocumentDescription::new();
    desc.format(doc.format());
    if doc.requests_mip_generation() {
        desc.mip_generate();
    } else {
        desc.mip_levels(doc.level_num());
    }

    let mut output = Vec::new();
    match doc.document_type() {
        DocumentType::Image1D => {
            let mut levels = Vec::new();
            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;
                levels.push(&file[level.to_slice_range()]);
            }
            desc.image_1d(doc.width(), &levels);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Image2D => {
            let mut levels = Vec::new();
            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;
                levels.push(&file[level.to_slice_range()]);
            }
            desc.image_2d(doc.width(), doc.height(), &levels);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Image3D => {
            let mut levels = Vec::new();
            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;
                levels.push(&file[level.to_slice_range()]);
            }
            desc.image_3d(doc.width(), doc.height(), doc.depth(), &levels);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Cube => {
            let mut face_levels = [
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ];

            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;

                let face_size = level.size_uncompressed / 6;

                for face in 0..6 {
                    let face_offset = level.offset + (face_size * face);
                    let face_end = face_offset + face_size;
                    let face_range = face_offset as usize..face_end as usize;
                    face_levels[face as usize].push(&file[face_range]);
                }
            }

            let faces = [
                face_levels[0].as_slice(),
                face_levels[1].as_slice(),
                face_levels[2].as_slice(),
                face_levels[3].as_slice(),
                face_levels[4].as_slice(),
                face_levels[5].as_slice(),
            ];

            desc.cube(doc.width(), doc.height(), faces);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Array1D => {
            let layer_num = doc.layer_num() as u64;

            let mut layer_levels = Vec::new();
            for _ in 0..layer_num {
                layer_levels.push(Vec::new());
            }

            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;

                let layer_size = level.size_uncompressed / layer_num;

                for layer in 0..layer_num {
                    let layer_offset = level.offset + (layer_size * layer);
                    let layer_end = layer_offset + layer_size;
                    let layer_range = layer_offset as usize..layer_end as usize;
                    layer_levels[layer as usize].push(&file[layer_range]);
                }
            }

            let layers = Vec::from_iter(layer_levels.iter().map(|v| v.as_slice()));
            desc.image_1d_array(doc.width(), &layers);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Array2D => {
            let layer_num = doc.layer_num() as u64;

            let mut layer_levels = Vec::new();
            for _ in 0..layer_num {
                layer_levels.push(Vec::new());
            }

            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;

                let layer_size = level.size_uncompressed / layer_num;

                for layer in 0..layer_num {
                    let layer_offset = level.offset + (layer_size * layer);
                    let layer_end = layer_offset + layer_size;
                    let layer_range = layer_offset as usize..layer_end as usize;
                    layer_levels[layer as usize].push(&file[layer_range]);
                }
            }

            let layers = Vec::from_iter(layer_levels.iter().map(|v| v.as_slice()));
            desc.image_2d_array(doc.width(), doc.height(), &layers);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::Array3D => {
            let layer_num = doc.layer_num() as u64;

            let mut layer_levels = Vec::new();
            for _ in 0..layer_num {
                layer_levels.push(Vec::new());
            }

            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;

                let layer_size = level.size_uncompressed / layer_num;

                for layer in 0..layer_num {
                    let layer_offset = level.offset + (layer_size * layer);
                    let layer_end = layer_offset + layer_size;
                    let layer_range = layer_offset as usize..layer_end as usize;
                    layer_levels[layer as usize].push(&file[layer_range]);
                }
            }

            let layers = Vec::from_iter(layer_levels.iter().map(|v| v.as_slice()));
            desc.image_3d_array(doc.width(), doc.height(), doc.depth(), &layers);
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
        DocumentType::CubeArray => {
            let layer_num = doc.layer_num() as u64;

            let mut layer_faces: Vec<[Vec<_>; 6]> = Vec::new();
            for _ in 0..layer_num {
                layer_faces.push([
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ]);
            }

            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;

                let layer_size = level.size_uncompressed / layer_num;
                let face_size = layer_size / 6;

                for layer in 0..layer_num {
                    let layer_offset = level.offset + (layer_size * layer);
                    let layer_end = layer_offset + layer_size;
                    let layer_range = layer_offset as usize..layer_end as usize;
                    let layer_bytes = &file[layer_range];
                    for face in 0..6 {
                        let face_offset = face_size * face;
                        let face_end = face_offset + face_size;
                        let face_range = face_offset as usize..face_end as usize;
                        layer_faces[layer as usize][face as usize].push(&layer_bytes[face_range]);
                    }
                }
            }

            let layers = Vec::from_iter(layer_faces.iter().map(|v| {
                [
                    v[0].as_slice(),
                    v[1].as_slice(),
                    v[2].as_slice(),
                    v[3].as_slice(),
                    v[4].as_slice(),
                    v[5].as_slice(),
                ]
            }));
            desc.cube_array(doc.width(), doc.height(), layers.as_slice());
            desc.write(&mut output)?;

            check_round_trip(file, output, doc)?;
        }
    }

    Ok(())
}

fn check_round_trip<R: Read + Seek>(
    file: &[u8],
    output: Vec<u8>,
    original: KTXDocument<R>,
) -> Result<(), KTXReadError> {
    let read = Cursor::new(output.as_slice());
    let doc = KTXDocument::from_reader(read)?;

    assert_eq!(doc.format(), original.format());
    assert_eq!(
        doc.requests_mip_generation(),
        original.requests_mip_generation()
    );
    assert_eq!(doc.document_type(), original.document_type());
    assert_eq!(doc.width(), original.width());
    assert_eq!(doc.height(), original.height());
    assert_eq!(doc.depth(), original.depth());
    assert_eq!(doc.face_num(), original.face_num());
    assert_eq!(doc.level_num(), original.level_num());
    assert_eq!(doc.layer_num(), original.layer_num());

    let mut scratch = [0u8; ENCODER_NAME.len() + 1];
    let writer_name = doc.lookup_writer(&mut scratch).unwrap();
    assert_eq!(ENCODER_NAME, writer_name);

    match doc.document_type() {
        DocumentType::Image1D
        | DocumentType::Image2D
        | DocumentType::Image3D
        | DocumentType::Cube
        | DocumentType::Array1D
        | DocumentType::Array2D
        | DocumentType::Array3D => {
            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;
                let original_level = original.get_level_info(i as usize)?;

                let bytes = &output[level.to_slice_range()];
                let original_bytes = &file[original_level.to_slice_range()];

                assert_eq!(bytes.len(), original_bytes.len());
                assert_eq!(bytes, original_bytes);
            }
        }
        DocumentType::CubeArray => {
            for i in 0..doc.level_num() {
                let level = doc.get_level_info(i as usize)?;
                let original_level = original.get_level_info(i as usize)?;

                let bytes = &output[level.to_slice_range()];
                let original_bytes = &file[original_level.to_slice_range()];

                assert_eq!(bytes.len(), original_bytes.len());
                assert_eq!(bytes, original_bytes);
            }
        }
    }

    Ok(())
}
