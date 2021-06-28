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
//! This module implements all standard explicit tags defined by
//! [ILTags Specification](https://github.com/interlockledger/specification/tree/master/ILTags).
use super::constants::*;
use super::{DefaultWithId, ErrorKind, ILTag, ILTagFactory, Result};
use crate::io::data::*;
use crate::io::{LimitedReader, Reader, Writer};
use crate::tags::{
    deserialize_bytes, deserialize_ilint, serialize_bytes, serialize_ilint, tag_size_to_usize,
    ILRawTag,
};
use ::std::any::Any;
use ::std::collections::HashMap;

/// This macro defines the methods for tags that uses an ILRawTag as
/// its inner implementation.
///
/// This struct must contain a single field called inner using the type
/// ILRawTag. Example:
///
/// ```
/// pub struct NewStruct {
///     inner: ILRawTag,
/// }
///
/// impl  NewStruct {
///     std_byte_array_tag_func_impl!();
///     ...
/// }
/// ```
///
/// It defines the following methods:
/// - `pub fn new() -> Self`;
/// - `pub fn with_value(value: &[u8]) -> Self`;
/// - `pub fn value(&self) -> &Vec<u8>`;
/// - `pub fn mut_value(&mut self) -> &mut Vec<u8>`;
///
macro_rules! std_byte_array_tag_func_impl {
    ($tag_id: expr) => {
        /// Creates a new instance of this struct.
        pub fn new() -> Self {
            Self {
                inner: ILRawTag::new($tag_id),
            }
        }

        /// Creates a new instance of this struct with the
        /// specified initial value.
        ///
        /// Arguments:
        /// * `value`: A byte slice with the initial value;
        pub fn with_value(value: &[u8]) -> Self {
            Self {
                inner: ILRawTag::with_value($tag_id, value),
            }
        }

        /// Returns an immutable reference to the value.
        pub fn value(&self) -> &Vec<u8> {
            self.inner.value()
        }

        /// Returns a mutable reference to the value.
        pub fn mut_value(&mut self) -> &mut Vec<u8> {
            self.inner.mut_value()
        }
    };
}

//=============================================================================
// ILByteArrayTag
//-----------------------------------------------------------------------------
/// This struct the standard byte array tag. It is equivalent `ILRawTag` but
/// always set the tag id to `IL_BYTES_TAG_ID`.
pub struct ILByteArrayTag {
    inner: ILRawTag,
}

impl ILByteArrayTag {
    std_byte_array_tag_func_impl!(IL_BYTES_TAG_ID);

    /// Creates a new instance of this struct with a given capacity.
    ///
    /// Arguments:
    /// * `capacity`: The expected initial capacity;
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: ILRawTag::with_capacity(IL_BYTES_TAG_ID, capacity),
        }
    }
}

inner_iltag_default_impl!(ILByteArrayTag);

impl Default for ILByteArrayTag {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// ILStringTag
//-----------------------------------------------------------------------------
/// This struct the standard string tag.
pub struct ILStringTag {
    id: u64,
    value: String,
}

impl ILStringTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_STRING_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            value: String::default(),
        }
    }

    pub fn with_value(value: &str) -> Self {
        Self::with_id_value(IL_STRING_TAG_ID, value)
    }

    pub fn with_id_value(id: u64, value: &str) -> Self {
        Self {
            id,
            value: String::from(value),
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn mut_value(&mut self) -> &mut String {
        &mut self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value.replace_range(.., value);
    }
}

impl ILTag for ILStringTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        self.value.len() as u64
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        serialize_bytes(self.value.as_bytes(), writer)
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let tmp = deserialize_bytes(value_size, reader)?;
        match ::std::str::from_utf8(tmp.as_slice()) {
            Ok(v) => {
                self.value.replace_range(.., v);
                Ok(())
            }
            Err(_) => Err(ErrorKind::CorruptedData),
        }
    }
}

iltag_default_impl!(ILStringTag);

/// Computes the size of the standard string tag from its value,
/// without the need to create a tag instance.
///
/// Arguments:
/// - `s`: The string value;
///
/// Returns:
/// - The size of the tag with the given value.
pub fn string_tag_size_from_value(s: &str) -> u64 {
    let len = s.len() as u64;
    1 + crate::ilint::encoded_size(len) as u64 + len
}

/// Serializes a string into a standard string tag directly from
/// its value.
///
/// Arguments:
/// - `s`: The string;
/// - `writer`: The writer;
///
/// Returns:
/// - `Ok(())`: On success;
/// - `Err(e)`: In case of error.
pub fn serialize_string_tag_from_value(s: &str, writer: &mut dyn Writer) -> Result<()> {
    let len = s.len() as u64;

    serialize_ilint(IL_STRING_TAG_ID, writer)?;
    serialize_ilint(len, writer)?;
    serialize_bytes(s.as_bytes(), writer)
}

/// Extracts a string value from a standard string tag directly from
/// the data stream.
///
/// Arguments:
/// - `reader`: The reader;
///
/// Returns:
/// - `Ok(String)`: The string extracted from the data stream;
/// - `Err(e)`: In case of error.
pub fn deserialize_string_tag_from_value(reader: &mut dyn Reader) -> Result<String> {
    let id = deserialize_ilint(reader)?;
    if id != IL_STRING_TAG_ID {
        return Err(ErrorKind::CorruptedData);
    }
    let len = deserialize_ilint(reader)?;
    // Performs this conversion to ensure that
    let usize_len = tag_size_to_usize(len)?;
    let tmp = deserialize_bytes(usize_len, reader)?;
    let s = match ::std::str::from_utf8(tmp.as_slice()) {
        Ok(v) => v,
        Err(_) => return Err(ErrorKind::CorruptedData),
    };
    Ok(String::from(s))
}

//=============================================================================
// ILBigIntTag
//-----------------------------------------------------------------------------
/// This struct the standard big integer tag. It is equivalent to the `ILRawTag`
/// but fixes the tag id to `IL_BINT_TAG_ID`. It assumes that the value is always
/// encoded as a two's complement big endian value.
pub struct ILBigIntTag {
    inner: ILRawTag,
}

impl ILBigIntTag {
    std_byte_array_tag_func_impl!(IL_BINT_TAG_ID);
}

inner_iltag_default_impl!(ILBigIntTag);

impl Default for ILBigIntTag {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// ILBigDecTag
//-----------------------------------------------------------------------------
/// This struct the standard big decimal tag. This tag serializes the data
/// into a scale value (i32) and the integral part encoded as a two's complement
/// big endian value.
pub struct ILBigDecTag {
    inner: ILRawTag,
    scale: i32,
}

impl ILBigDecTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self {
            scale: 0,
            inner: ILRawTag::new(IL_BDEC_TAG_ID),
        }
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            scale: 0,
            inner: ILRawTag::new(id),
        }
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_value(scale: i32, value: &[u8]) -> Self {
        Self::with_id_value(IL_BDEC_TAG_ID, scale, value)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id_value(id: u64, scale: i32, value: &[u8]) -> Self {
        Self {
            scale,
            inner: ILRawTag::with_value(id, value),
        }
    }

    /// Returns an immutable reference to the value.
    pub fn scale(&self) -> i32 {
        self.scale
    }

    /// Returns a mutable reference to the value.
    pub fn set_scale(&mut self, scale: i32) {
        self.scale = scale
    }

    /// Returns an immutable reference to the value.
    pub fn value(&self) -> &Vec<u8> {
        self.inner.value()
    }

    /// Returns a mutable reference to the value.
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        self.inner.mut_value()
    }
}

impl ILTag for ILBigDecTag {
    inner_iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        4 + self.inner.value_size()
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        match write_i32(self.scale(), writer) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
        self.inner.serialize_value(writer)
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        if value_size < 4 {
            return Err(ErrorKind::CorruptedData);
        }
        self.scale = match read_i32(reader) {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        self.inner
            .deserialize_value(factory, value_size - 4, reader)
    }
}

iltag_default_impl!(ILBigDecTag);

//=============================================================================
// ILILIntArrayTag
//-----------------------------------------------------------------------------
/// This struct the standard ILInt array tag. It is an array of u64 values
/// encoded using ILInt format.
pub struct ILILIntArrayTag {
    id: u64,
    value: Vec<u64>,
}

impl ILILIntArrayTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_ILINTARRAY_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            value: Vec::new(),
        }
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_value(value: &[u64]) -> Self {
        Self::with_id_value(IL_BDEC_TAG_ID, value)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id_value(id: u64, value: &[u64]) -> Self {
        let mut v: Vec<u64> = Vec::with_capacity(value.len());
        v.extend_from_slice(value);
        Self { id, value: v }
    }

    /// Returns an immutable reference to the value.
    pub fn value(&self) -> &Vec<u64> {
        &self.value
    }

    /// Returns a mutable reference to the value.
    pub fn mut_value(&mut self) -> &mut Vec<u64> {
        &mut self.value
    }
}

impl ILTag for ILILIntArrayTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        let mut size: u64 = crate::ilint::encoded_size(self.value.len() as u64) as u64;
        for v in self.value.as_slice() {
            size += crate::ilint::encoded_size(*v) as u64;
        }
        size
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        match write_ilint(self.value.len() as u64, writer) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        for v in self.value.as_slice() {
            match write_ilint(*v as u64, writer) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            };
        }
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let mut lreader = LimitedReader::new(reader, value_size);
        let count = deserialize_ilint(&mut lreader)?;
        if count > value_size as u64 {
            return Err(ErrorKind::CorruptedData);
        }
        self.value.clear();
        self.value.reserve(count as usize);
        for _i in 0..count {
            self.value.push(deserialize_ilint(&mut lreader)?);
        }
        if lreader.empty() {
            Ok(())
        } else {
            Err(ErrorKind::CorruptedData)
        }
    }
}

iltag_default_impl!(ILILIntArrayTag);

//=============================================================================
// ILTagSeqTag
//-----------------------------------------------------------------------------
/// This struct the standard tag sequence tag.
pub struct ILTagSeqTag {
    id: u64,
    value: Vec<Box<dyn ILTag>>,
}

impl ILTagSeqTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_ILTAGSEQ_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            value: Vec::new(),
        }
    }

    /// Returns an immutable reference to the value.
    pub fn value(&self) -> &Vec<Box<dyn ILTag>> {
        &self.value
    }

    /// Returns a mutable reference to the value.
    pub fn mut_value(&mut self) -> &mut Vec<Box<dyn ILTag>> {
        &mut self.value
    }
}

impl ILTag for ILTagSeqTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        let mut size: u64 = 0;
        for v in self.value.as_slice() {
            size += v.size();
        }
        size
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for v in self.value.as_slice() {
            v.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        self.value.clear();
        let mut read: u64 = value_size as u64;
        while read > 0 {
            let t = factory.deserialize(reader)?;
            read -= t.size();
            self.value.push(t);
        }
        Ok(())
    }
}

iltag_default_impl!(ILTagSeqTag);

//=============================================================================
// ILTagArrayTag
//-----------------------------------------------------------------------------
/// This struct the standard tag array tag. It differs from ILTagSeqTag
/// because it serializes the number of entries before the serialization of the
/// tags.
pub struct ILTagArrayTag {
    inner: ILTagSeqTag,
}

impl ILTagArrayTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_ILTAGARRAY_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            inner: ILTagSeqTag::with_id(id),
        }
    }

    /// Returns an immutable reference to the value.
    pub fn value(&self) -> &Vec<Box<dyn ILTag>> {
        self.inner.value()
    }

    /// Returns a mutable reference to the value.
    pub fn mut_value(&mut self) -> &mut Vec<Box<dyn ILTag>> {
        self.inner.mut_value()
    }
}

impl ILTag for ILTagArrayTag {
    inner_iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        let size: u64 = crate::ilint::encoded_size(self.inner.value.len() as u64) as u64;
        size + self.inner.value_size()
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        serialize_ilint(self.inner.value.len() as u64, writer)?;
        self.inner.serialize_value(writer)
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let mut lreader = LimitedReader::new(reader, value_size);

        let count = deserialize_ilint(&mut lreader)?;
        if count > value_size as u64 {
            return Err(ErrorKind::CorruptedData);
        }
        self.inner.value.clear();
        self.inner.value.reserve(count as usize);
        for _i in 0..count {
            self.inner.value.push(factory.deserialize(&mut lreader)?);
        }
        if lreader.empty() {
            Ok(())
        } else {
            Err(ErrorKind::CorruptedData)
        }
    }
}

iltag_default_impl!(ILTagArrayTag);

//=============================================================================
// ILRangeTag
//-----------------------------------------------------------------------------
/// This struct the standard range tag. The range tag consists of a starting
/// value (u64) followed by the number of entries (u16).
pub struct ILRangeTag {
    id: u64,
    start: u64,
    count: u16,
}

impl ILRangeTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_RANGE_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            start: 0,
            count: 0,
        }
    }

    pub fn with_value(start: u64, count: u16) -> Self {
        Self::with_id_value(IL_RANGE_TAG_ID, start, count)
    }

    pub fn with_id_value(id: u64, start: u64, count: u16) -> Self {
        Self { id, start, count }
    }

    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn set_start(&mut self, start: u64) {
        self.start = start;
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn set_count(&mut self, count: u16) {
        self.count = count;
    }

    pub fn set_value(&mut self, start: u64, count: u16) {
        self.start = start;
        self.count = count;
    }
}

impl ILTag for ILRangeTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        (crate::ilint::encoded_size(self.start) + 2) as u64
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        match write_ilint(self.start, writer) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        match write_u16(self.count, writer) {
            Ok(()) => Ok(()),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let mut lreader = LimitedReader::new(reader, value_size);
        self.start = match read_ilint(&mut lreader) {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        self.count = match read_u16(&mut lreader) {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        if lreader.empty() {
            Ok(())
        } else {
            Err(ErrorKind::CorruptedData)
        }
    }
}

iltag_default_impl!(ILRangeTag);

//=============================================================================
// ILVersionTag
//-----------------------------------------------------------------------------
/// This struct the standard version tag. It encodes the 4 parts of the version
/// as i32 values. Those parts are named major, minor, revision and build.
pub struct ILVersionTag {
    id: u64,
    value: [i32; 4],
}

impl ILVersionTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_VERSION_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self { id, value: [0; 4] }
    }

    pub fn with_value(major: i32, minor: i32, release: i32, build: i32) -> Self {
        Self::with_id_value(IL_VERSION_TAG_ID, major, minor, release, build)
    }

    pub fn with_id_value(id: u64, major: i32, minor: i32, release: i32, build: i32) -> Self {
        Self {
            id,
            value: [major, minor, release, build],
        }
    }

    pub fn major(&self) -> i32 {
        self.value[0]
    }

    pub fn set_major(&mut self, major: i32) {
        self.value[0] = major;
    }

    pub fn minor(&self) -> i32 {
        self.value[1]
    }

    pub fn set_minor(&mut self, minor: i32) {
        self.value[1] = minor;
    }

    pub fn release(&self) -> i32 {
        self.value[2]
    }

    pub fn set_release(&mut self, release: i32) {
        self.value[2] = release;
    }

    pub fn build(&self) -> i32 {
        self.value[3]
    }

    pub fn set_build(&mut self, build: i32) {
        self.value[3] = build;
    }

    pub fn value(&self) -> &[i32; 4] {
        &self.value
    }

    pub fn set_value(&mut self, major: i32, minor: i32, release: i32, build: i32) {
        self.set_value_from_slice(&[major, minor, release, build]);
    }

    pub fn set_value_from_slice(&mut self, value: &[i32; 4]) {
        self.value.copy_from_slice(value);
    }
}

impl ILTag for ILVersionTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        16
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for v in &self.value {
            match write_i32(*v, writer) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            };
        }
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        if value_size != 16 {
            return Err(ErrorKind::CorruptedData);
        }
        for i in 0..4 {
            self.value[i] = match read_i32(reader) {
                Ok(v) => v,
                Err(e) => return Err(ErrorKind::IOError(e)),
            };
        }
        Ok(())
    }
}

iltag_default_impl!(ILVersionTag);

//=============================================================================
// ILOIDTag
//-----------------------------------------------------------------------------
/// This struct the standard OID tag. It is designed to store ITU Object
/// identifier values as an array of u64 values. It uses the same encoding
/// of `ILILIntArrayTag`.
pub struct ILOIDTag {
    inner: ILILIntArrayTag,
}

impl ILOIDTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_OID_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            inner: ILILIntArrayTag::with_id(id),
        }
    }

    pub fn with_value(value: &[u64]) -> Self {
        Self::with_id_value(IL_OID_TAG_ID, value)
    }

    pub fn with_id_value(id: u64, value: &[u64]) -> Self {
        Self {
            inner: ILILIntArrayTag::with_id_value(id, value),
        }
    }

    pub fn value(&self) -> &[u64] {
        self.inner.value()
    }

    pub fn mut_value(&mut self) -> &mut [u64] {
        self.inner.mut_value()
    }
}

inner_iltag_default_impl!(ILOIDTag);

iltag_default_impl!(ILOIDTag);

//=============================================================================
// ILDictTag
//-----------------------------------------------------------------------------
/// This struct implements the standard dictionary tag. It always maps string
/// to tags.
pub struct ILDictTag {
    id: u64,
    value: HashMap<String, Box<dyn ILTag>>,
}

impl ILDictTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_DICTIONARY_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            value: HashMap::default(),
        }
    }

    pub fn value(&self) -> &HashMap<String, Box<dyn ILTag>> {
        &self.value
    }

    pub fn mut_value(&mut self) -> &mut HashMap<String, Box<dyn ILTag>> {
        &mut self.value
    }
}

impl ILTag for ILDictTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        let mut size: u64 = 0;
        for (key, value) in self.value.iter() {
            size += string_tag_size_from_value(key);
            size += value.size();
        }
        size
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for (key, value) in self.value.iter() {
            serialize_string_tag_from_value(key, writer)?;
            value.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let mut lreader = LimitedReader::new(reader, value_size);
        self.value.clear();
        while lreader.available() > 0 {
            self.value.insert(
                deserialize_string_tag_from_value(&mut lreader)?,
                factory.deserialize(&mut lreader)?,
            );
        }
        Ok(())
    }
}

iltag_default_impl!(ILDictTag);

//=============================================================================
// ILStrDictTag
//-----------------------------------------------------------------------------
/// This struct implements the standard string dictionary tag. It always maps
/// strings to strings.
pub struct ILStrDictTag {
    id: u64,
    value: HashMap<String, String>,
}

impl ILStrDictTag {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::with_id(IL_STRING_DICTIONARY_TAG_ID)
    }

    /// Creates a new instance of this struct with the
    /// specified initial value.
    ///
    /// Arguments:
    /// * `value`: A byte slice with the initial value;
    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            value: HashMap::default(),
        }
    }

    pub fn value(&self) -> &HashMap<String, String> {
        &self.value
    }

    pub fn mut_value(&mut self) -> &mut HashMap<String, String> {
        &mut self.value
    }
}

impl ILTag for ILStrDictTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        let mut size: u64 = 0;
        for (key, value) in self.value.iter() {
            size += string_tag_size_from_value(key);
            size += string_tag_size_from_value(value);
        }
        size
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for (key, value) in self.value.iter() {
            serialize_string_tag_from_value(key, writer)?;
            serialize_string_tag_from_value(value, writer)?;
        }
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        let mut lreader = LimitedReader::new(reader, value_size);
        self.value.clear();
        while lreader.available() > 0 {
            self.value.insert(
                deserialize_string_tag_from_value(&mut lreader)?,
                deserialize_string_tag_from_value(&mut lreader)?,
            );
        }
        Ok(())
    }
}

iltag_default_impl!(ILStrDictTag);
