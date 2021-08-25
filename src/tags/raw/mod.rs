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
//! from any sequence of bytes without the need to convert it into an [`ILTag`]
//! instance.
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
}

//=============================================================================
// RawTagScanner
//-----------------------------------------------------------------------------
/// This struct implements a Raw tag scanner that can scan a reader and find
/// the information about the tags inside it.
///
/// Only the top level tags will be considered as it does not parse the
/// value of the tags.
pub struct RawTagScanner<'a> {
    offset: u64,
    reader: &'a mut dyn Reader,
}

impl<'a> RawTagScanner<'a> {
    /// Creates a new instance of [`RawTagScanner`]. It assumes the current
    /// offset of the provided reader as being 0.
    pub fn new(reader: &'a mut dyn Reader) -> Self {
        Self { offset: 0, reader }
    }

    /// Returns the information about the next tag in the reader and skips to
    /// the beginning of the next tag.
    pub fn find_next(&mut self) -> Result<Option<RawTagOffset>> {
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

//=============================================================================
// RawTagView
//-----------------------------------------------------------------------------
/// This struct maps a memory segment into the tag's components using the
/// specified [`RawTagOffset`].
pub struct RawTagView<'a> {
    data: &'a [u8],
    offset: RawTagOffset,
}

impl<'a> RawTagView<'a> {
    pub fn new(data: &'a [u8], offset: RawTagOffset) -> Result<Self> {
        if offset.next_tag_offset() <= data.len() as u64 {
            Ok(RawTagView { data, offset })
        } else {
            Err(ErrorKind::CorruptedData)
        }
    }

    #[inline]
    pub fn offset(&self) -> &RawTagOffset {
        &self.offset
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.offset().id()
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.offset().size()
    }

    #[inline]
    pub fn value_size(&self) -> u64 {
        self.offset().value_size()
    }

    pub fn tag(&self) -> &[u8] {
        let start = self.offset().offset() as usize;
        let end = self.offset().next_tag_offset() as usize;
        &self.data[start..end]
    }

    pub fn value(&self) -> &[u8] {
        let start = self.offset().value_offset() as usize;
        let end = self.offset().next_tag_offset() as usize;
        &self.data[start..end]
    }
}
