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
//! This module contains the implementation of the **IL2 ILTags** standard.
pub mod std;

use crate::ilint::encoded_size;
use crate::io::data::{DataReader, DataWriter};
use ::std::any::Any;

/// Definition of the errors from this package.
pub enum ErrorKind {
    UnknownTag,
    UnsupportedTag,
    CorruptedData,
    IOError(crate::io::ErrorKind),
    Boxed(Box<dyn ::std::error::Error>),
}

/// Alias to errors from this package.
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

/// Maximum tag id value for implicit tags.
pub const IMPLICIT_ID_MAX: u64 = 0x0F;

/// Maximum tag id value for reserved tags.
pub const RESERVED_ID_MAX: u64 = 0x1F;

/// Verifies if a given tag id represents an implicit tag.
///
/// Arguments:
///
/// * `id`: The tag id to be verified;
///
/// Returns:
///
/// * true if the tag id is implicit or false otherwise.
///
pub fn is_implicit_tag(id: u64) -> bool {
    id < IMPLICIT_ID_MAX
}

/// Verifies if a given tag id represents a reserved tag.
///
/// Arguments:
///
/// * `id`: The tag id to be verified;
///
/// Returns:
///
/// * true if the tag id is reserved or false otherwise.
///
pub fn is_reserved_tag(id: u64) -> bool {
    id < RESERVED_ID_MAX
}

/// This trait defines the ability to convert
pub trait ILTagAsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub trait ILTag: ILTagAsAny {
    /// Returns the ID of the tag.
    fn id(&self) -> u64;

    /// Verifies if this tag is implicity.
    fn is_implicity(&self) -> bool {
        is_implicit_tag(self.id())
    }

    /// Verifies if this tag is reserved.
    fn is_reserved(&self) -> bool {
        is_reserved_tag(self.id())
    }

    /// Retuns the size of the payload in bytes.
    fn payload_size(&self) -> usize;

    /// Returns the total size of the tag in bytes.
    fn size(&self) -> usize {
        let mut size: usize = encoded_size(self.id());
        if !self.is_implicity() {
            size += encoded_size(self.payload_size() as u64);
        }
        size + self.payload_size()
    }

    /// Serializes the payload of this tag.
    ///
    /// Arguments:
    ///
    /// * `writer`: The writer that will receive the encoded value;
    ///
    /// Returns:
    ///
    /// * `Ok()`: On success.
    /// * `Err(())`: If the buffer is too small to hold the encoded value.
    ///
    fn serialize_value(&self, writer: &mut dyn DataWriter) -> Result<()>;

    /// Serializes this tag.
    ///
    /// Arguments:
    ///
    /// * `writer`: The writer that will receive the encoded value;
    ///
    /// Returns:
    ///
    /// * `Ok()`: On success.
    /// * `Err(())`: If the buffer is too small to hold the encoded value.
    ///
    fn serialize(&self, writer: &mut dyn DataWriter) -> Result<()> {
        match writer.write_ilint(self.id()) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
        if !self.is_implicity() {
            match writer.write_ilint(self.payload_size() as u64) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            }
        }
        self.serialize_value(writer)
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        reader: &mut dyn DataReader,
    ) -> Result<()> {
        if self.is_implicity() {
            panic!("The default implementation does not support implicity values.")
        }
        let size = match reader.read_ilint() {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        self.deserialize_value_core(factory, size as usize, reader)
    }

    fn deserialize_value_core(
        &mut self,
        factory: &dyn ILTagFactory,
        payload_size: usize,
        reader: &mut dyn DataReader,
    ) -> Result<()>;
}

pub trait ILTagFactory {
    fn as_ref(&self) -> &dyn ILTagFactory;

    fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>>;

    fn deserialize(&self, reader: &mut dyn DataReader) -> Result<Box<dyn ILTag>> {
        let tag_id = match reader.read_ilint() {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        let mut tag = match self.create_tag(tag_id) {
            Some(v) => v,
            _ => return Err(ErrorKind::UnknownTag),
        };
        match tag.deserialize_value(self.as_ref(), reader) {
            Ok(()) => Ok(tag),
            Err(e) => Err(e),
        }
    }
}

impl<T: ILTag> ILTagAsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// This struct implements a raw tag. It can be used
/// to store any non implicity tag.
pub struct RawTag {
    /// Id of the tag.
    id: u64,
    /// The payload.
    payload: Vec<u8>,
}

impl RawTag {
    /// Creates a new instance of this struct.
    ///
    /// Arguments:
    ///
    /// * `id`: The tag id;
    ///
    /// Returns:
    ///
    /// * `Ok(value)`: The new instace of the raw tag.
    /// * `Err(ErrorKind::UnsupportedTag)`: If the tag id is unsupported.
    pub fn new(id: u64) -> Result<RawTag> {
        if is_implicit_tag(id) {
            Err(ErrorKind::UnsupportedTag)
        } else {
            Ok(RawTag {
                id,
                payload: Vec::new(),
            })
        }
    }

    /// Returns an immutable reference to the payload.
    pub fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }

    /// Returns a mutable reference to the payload.
    pub fn get_mut_payload(&mut self) -> &mut Vec<u8> {
        &mut self.payload
    }
}

impl ILTag for RawTag {
    fn id(&self) -> u64 {
        self.id
    }
    fn payload_size(&self) -> usize {
        self.payload.len()
    }

    fn serialize_value(&self, writer: &mut dyn DataWriter) -> Result<()> {
        match writer.write_all(self.payload.as_slice()) {
            Ok(()) => Ok(()),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn deserialize_value_core(
        &mut self,
        _factory: &dyn ILTagFactory,
        payload_size: usize,
        reader: &mut dyn DataReader,
    ) -> Result<()> {
        self.get_mut_payload()
            .resize_with(payload_size, Default::default);
        match reader.read_all(self.payload.as_mut_slice()) {
            Ok(()) => Ok(()),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }
}
