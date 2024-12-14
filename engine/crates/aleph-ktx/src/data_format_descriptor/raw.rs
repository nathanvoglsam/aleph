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

use std::io::{Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

///
/// Raw layout of a DFD
///
pub struct RawDataFormatDescriptor {
    pub dfd_total_size: u32,
    pub vendor_id: u32,
    pub descriptor_type: u16,
    pub version_number: u16,
    pub descriptor_block_size: u16,
    pub color_model: u8,
    pub color_primaries: u8,
    pub transfer_function: u8,
    pub flags: u8,
    pub block_dimensions: [u8; 4],
    pub byte_planes: [u8; 8],
}

impl RawDataFormatDescriptor {
    ///
    /// Reads the next 7 words out of the reader and unpacks them to a raw DFD struct
    ///
    #[allow(clippy::identity_op)]
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, std::io::Error> {
        //
        // Read off the next 7 words of the DFD and unpack them into the different parts
        //

        // Read in the next 7 words.
        let mut words = [0; 7];
        reader.read_u32_into::<LittleEndian>(&mut words)?;

        // Unpack the next word
        let dfd_total_size = words[0];

        // Unpack the next word
        let vendor_id = words[1] & 0x1FFFF;
        let descriptor_type = (words[1] >> 17) as u16;

        // Unpack the next word
        let version_number = (words[2] & 0xFFFF) as u16;
        let descriptor_block_size = (words[2] >> 16) as u16;

        // Unpack the next word
        let color_model = ((words[3] >> 0) & 0xFF) as u8;
        let color_primaries = ((words[3] >> 8) & 0xFF) as u8;
        let transfer_function = ((words[3] >> 16) & 0xFF) as u8;
        let flags = ((words[3] >> 24) & 0xFF) as u8;

        // Unpack the next word
        let texel_block_dimensions_0 = (words[4] >> 0) & 0xFF;
        let texel_block_dimensions_1 = (words[4] >> 8) & 0xFF;
        let texel_block_dimensions_2 = (words[4] >> 16) & 0xFF;
        let texel_block_dimensions_3 = (words[4] >> 24) & 0xFF;
        let block_dimensions = [
            texel_block_dimensions_0 as u8,
            texel_block_dimensions_1 as u8,
            texel_block_dimensions_2 as u8,
            texel_block_dimensions_3 as u8,
        ];

        // Unpack byte plane sizes
        let byte_planes_0 = ((words[5] >> 0) & 0xFF) as u8;
        let byte_planes_1 = ((words[5] >> 8) & 0xFF) as u8;
        let byte_planes_2 = ((words[5] >> 16) & 0xFF) as u8;
        let byte_planes_3 = ((words[5] >> 24) & 0xFF) as u8;
        let byte_planes_4 = ((words[6] >> 0) & 0xFF) as u8;
        let byte_planes_5 = ((words[6] >> 8) & 0xFF) as u8;
        let byte_planes_6 = ((words[6] >> 16) & 0xFF) as u8;
        let byte_planes_7 = ((words[6] >> 24) & 0xFF) as u8;
        let byte_planes = [
            byte_planes_0,
            byte_planes_1,
            byte_planes_2,
            byte_planes_3,
            byte_planes_4,
            byte_planes_5,
            byte_planes_6,
            byte_planes_7,
        ];

        Ok(Self {
            dfd_total_size,
            vendor_id,
            descriptor_type,
            version_number,
            descriptor_block_size,
            color_model,
            color_primaries,
            transfer_function,
            flags,
            block_dimensions,
            byte_planes,
        })
    }
}
