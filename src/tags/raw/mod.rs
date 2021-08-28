/*
 * BSD 3-Clause License
 *
 * Copyright (c) 2020, InterlockLedger Network
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright notice, this
 *   list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 *
 * * Neither the name of the copyright holder nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
//=============================================================================
//! This module contains helper struct and functions that allows the parsing of
//! a serialized sequence of tags. It allows the extraction tag information
//! from any sequence of bytes without the need to convert it into an
//! [`crate::tags::ILTag`] instance.
//!
//! It is expected that
//!
//! New since 1.4.0.
#[cfg(test)]
mod tests;
use crate::io::Reader;
use crate::tags::serialization::*;
use crate::tags::standard::constants::*;
use crate::tags::{is_implicit_tag, ErrorKind, Result};

//=============================================================================
// RawTagOffset
//-----------------------------------------------------------------------------
/// This struct implements the raw tag offset information. It stores the
/// information about a tag, such as its id and the location of its parts
/// inside a sequence of bytes.
///
/// It does not store references to the original byte stream, just the
/// information about a tags inside it.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RawTagOffset {
    id: u64,
    offset: u64,
    header_size: u64,
    value_size: u64,
}

impl RawTagOffset {
    /// Creates a new [`RawTagOffset`].
    pub fn new(id: u64, offset: u64, header_size: u64, value_size: u64) -> Self {
        Self {
            id,
            offset,
            header_size,
            value_size,
        }
    }

    /// Returns the ID of the tag.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Retuns the offset of the tag.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the total tag size.
    pub fn size(&self) -> u64 {
        self.header_size + self.value_size
    }

    /// Returns the offset of the tag's value.
    pub fn value_offset(&self) -> u64 {
        self.offset + self.header_size
    }

    /// Returns the size of the tag's value.
    pub fn value_size(&self) -> u64 {
        self.value_size
    }

    /// Returns the offset of the next tag.
    pub fn next_tag_offset(&self) -> u64 {
        self.offset + self.size()
    }

    /// Returns the initial offset of this tag's value. It is usefull to
    /// create slices on the raw data.
    #[inline]
    pub fn value_start(&self) -> usize {
        self.value_offset() as usize
    }

    /// Returns the end offset of the value. It is usefull to
    /// create slices on the raw data.
    #[inline]
    pub fn value_end(&self) -> usize {
        self.next_tag_offset() as usize
    }

    /// Returns the start offset of the tag. It is usefull to
    /// create slices on the raw data.
    #[inline]
    pub fn tag_start(&self) -> usize {
        self.offset() as usize
    }

    /// Returns the end offset of the tag.
    #[inline]
    pub fn tag_end(&self) -> usize {
        self.value_end()
    }

    /// Returns a slice within `raw` that points to the value of the
    /// tag pointed by this instance.
    ///
    /// Arguments:
    /// - `raw`: The array that contains the tag indexe by this instance;
    ///
    /// Returns a immutable slice within the raw that contains the value of the tag.
    pub fn tag_slice<'a>(&self, raw: &'a [u8]) -> &'a [u8] {
        &raw[self.tag_start()..self.tag_end()]
    }

    /// Returns a slice within `raw` that points to the value of the
    /// tag pointed by this instance.
    ///
    /// Arguments:
    /// - `raw`: The array that contains the tag indexe by this instance;
    ///
    /// Returns a immutable slice within the raw that contains the value of the tag.
    pub fn value_slice<'a>(&self, raw: &'a [u8]) -> &'a [u8] {
        &raw[self.value_start()..self.value_end()]
    }

    /// Creates a new instance of this struct that maps the offsets to the
    /// same space of the parent. It assumes that this instance is contained
    /// by the parent (is within its value) and assumes 0 as the start of
    /// the parent's value.
    ///
    /// For example:
    /// - Parent's value offset = 10;
    /// - This offset = 5;
    ///
    /// The result will be a new instance that maps the beginning of this
    /// tag to 15. This allows the extraction of the tag directly from the
    /// byte sequence where the parent resides.
    ///
    /// This method will perform only basic validations to ensure that
    /// this instace an indeed fit inside the parent but if those two
    /// instances are unrelated, the behavior of this method is undefined.
    ///
    /// Arguments:
    /// - `parent`: The parent of this instance;
    ///
    /// Returns a new instance of [`Self`] that points to the same tag
    /// value but relative to the offset space of the parent.
    pub fn map_to_parent_space(&self, parent: &Self) -> Result<Self> {
        if self.next_tag_offset() > parent.value_size() {
            Err(ErrorKind::CorruptedData)
        } else {
            Ok(Self {
                id: self.id,
                offset: parent.value_offset() + self.offset,
                header_size: self.header_size,
                value_size: self.value_size,
            })
        }
    }
}

impl Default for RawTagOffset {
    fn default() -> Self {
        Self {
            id: 0,
            offset: 0,
            header_size: 0,
            value_size: 0,
        }
    }
}

//=============================================================================
// RawTagScanner
//-----------------------------------------------------------------------------
/// This struct implements a Raw tag scanner that can scan a reader and find
/// the information about the tags inside it.
///
/// Only the top level tags will be considered as it does not parse the
/// value of the tags.
pub struct RawTagScanner<'a, T: Reader> {
    offset: u64,
    reader: &'a mut T,
}

impl<'a, T: Reader> RawTagScanner<'a, T> {
    /// Creates a new instance of [`RawTagScanner`]. It assumes the current
    /// offset of the provided reader as being 0.
    pub fn new(reader: &'a mut T) -> Self {
        Self { offset: 0, reader }
    }

    /// Returns the information about the next tag.
    pub fn next_tag(&mut self) -> Result<Option<RawTagOffset>> {
        let id: u64 = self.reader.deserialize_ilint()?;
        let size_info = self.extract_tag_size(id)?;
        let header_size: u64 = crate::ilint::encoded_size(id) as u64 + size_info.0;
        let value_size: u64 = size_info.1;
        let bytes_to_skip: u64 = size_info.2;
        let offset = self.offset;
        self.offset += header_size + value_size;
        self.reader.skip_u64(bytes_to_skip)?;
        Ok(Some(RawTagOffset {
            id,
            offset,
            header_size,
            value_size,
        }))
    }

    /// Returns the information about the next tag. It works just like
    /// [`Self::next_tag()`] but returns an error if the tag id does not match the
    /// specified id.
    pub fn next_tag_if_id(&mut self, expected_id: u64) -> Result<Option<RawTagOffset>> {
        match self.next_tag()? {
            Some(t) => {
                if t.id == expected_id {
                    Ok(Some(t))
                } else {
                    Err(ErrorKind::UnexpectedTagType)
                }
            }
            None => Ok(None),
        }
    }

    /// Extracts the size of the tag from the reader.
    ///
    /// Returns (u64, u64, u64) where:
    /// - 0: Size of size field;
    /// - 1: Value size;
    /// - 2: Number of bytes to skip to reach the end of the tag;
    fn extract_tag_size(&mut self, id: u64) -> Result<(u64, u64, u64)> {
        Ok(if is_implicit_tag(id) {
            match id {
                IL_ILINT_TAG_ID | IL_SIGNED_ILINT_TAG_ID => {
                    self.extract_implicit_ilint_tag_size()?
                }
                15 => return Err(ErrorKind::CorruptedData),
                _ => {
                    let value_size = crate::tags::standard::implicit::implicit_tag_size(id);
                    (0, value_size, value_size)
                }
            }
        } else {
            let value_size = self.reader.deserialize_ilint()?;
            (
                crate::ilint::encoded_size(value_size) as u64,
                value_size,
                value_size,
            )
        })
    }

    /// Extracts the size of an implicit ILInt tag from the reader.
    ///
    /// Returns (u64, u64, u64) where:
    /// - 0: Size of size field;
    /// - 1: Value size;
    /// - 2: Number of bytes to skip to reach the end of the tag;
    fn extract_implicit_ilint_tag_size(&mut self) -> Result<(u64, u64, u64)> {
        let header = self.reader.read()?;
        let total_size = crate::ilint::decoded_size(header);
        Ok((0, total_size as u64, (total_size - 1) as u64))
    }
}
