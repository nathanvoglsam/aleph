//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::data_format_descriptor::SampleFlags;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Seek};

pub struct SampleInfo {
    bit_offset: u16,
    bit_length: u8,
    channel_type: u8,
    sample_flags: SampleFlags,
    sample_positions: [u8; 4],
    sample_upper: u32,
    sample_lower: u32,
}

impl SampleInfo {
    ///
    /// Unpacks the sample info from 4 words (should have been read from a file)
    ///
    pub fn unpack_from(words: &[u32; 4]) -> Self {
        let bit_offset = ((words[0] >> 0) & 0xFFFF) as u16;
        let bit_length = ((words[0] >> 16) & 0xFF) as u8;
        let channel_type = ((words[0] >> 24) & 0xF) as u8;
        let sample_flags = ((words[0] >> 28) & 0xF) as u8;
        let sample_flags = SampleFlags::from_bits_truncate(sample_flags);

        let sample_pos_0 = ((words[1] >> 0) & 0xFF) as u8;
        let sample_pos_1 = ((words[1] >> 8) & 0xFF) as u8;
        let sample_pos_2 = ((words[1] >> 16) & 0xFF) as u8;
        let sample_pos_3 = ((words[1] >> 24) & 0xFF) as u8;
        let sample_positions = [sample_pos_0, sample_pos_1, sample_pos_2, sample_pos_3];

        let sample_upper = words[2];
        let sample_lower = words[3];

        Self {
            bit_offset,
            bit_length,
            channel_type,
            sample_flags,
            sample_positions,
            sample_upper,
            sample_lower,
        }
    }

    ///
    /// Gets the bit offset
    ///
    pub fn bit_offset(&self) -> u16 {
        self.bit_offset
    }

    ///
    /// Gets the decoded bit length (the file stores the bit length minus 1)
    ///
    /// # Info
    ///
    /// We cast the length up to u16 so we don't overflow
    ///
    pub fn bit_length(&self) -> u16 {
        self.bit_length as u16 + 1
    }

    ///
    /// Gets sample flags set by the sample info
    ///
    pub fn sample_flags(&self) -> SampleFlags {
        self.sample_flags
    }

    ///
    /// List of sample positions
    ///
    pub fn sample_positions(&self) -> &[u8; 4] {
        &self.sample_positions
    }

    ///
    /// Gets the sample upper value
    ///
    pub fn sample_upper(&self) -> u32 {
        self.sample_upper
    }

    ///
    /// Gets the sample lower value
    ///
    pub fn sample_lower(&self) -> u32 {
        self.sample_lower
    }
}

///
/// Iterates over the sample information blocks in the DFD block
///
pub struct SampleInfoIterator<'a, R: Read + Seek> {
    reader: &'a mut R,
    index: u16,
    count: u16,
}

impl<'a, R: Read + Seek> SampleInfoIterator<'a, R> {
    ///
    /// Will create an iterator that will yield `count` samples from the reader
    ///
    /// # Info
    ///
    /// There can't be more than `u16::max_value` sample infos because of how the number is stored
    /// in the file so we use a u16 for `count` to make this explicit (also makes the struct small)
    ///
    pub fn from_reader_count(reader: &'a mut R, count: u16) -> Self {
        Self {
            reader,
            index: 0,
            count,
        }
    }
}

impl<'a, R: Read + Seek> Iterator for SampleInfoIterator<'a, R> {
    type Item = (usize, SampleInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.count {
            None
        } else {
            let mut words = [0; 4];
            self.reader.read_u32_into::<LittleEndian>(&mut words).ok()?;

            // Build output and iterate the index
            let out = Some((self.index as usize, SampleInfo::unpack_from(&words)));
            self.index += 1;
            out
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}
