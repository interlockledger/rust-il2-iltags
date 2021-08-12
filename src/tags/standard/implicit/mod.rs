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
//! This module implements all standard implicit tags defined by
//! [ILTags Specification](https://github.com/interlockledger/specification/tree/master/ILTags).
#[cfg(test)]
mod tests;

use super::constants::*;
use super::{DefaultWithId, ErrorKind, ILTag, ILTagFactory, Result};
use crate::io::{Reader, Writer};
use crate::tags::serialization::*;
use ::std::any::Any;

/// This macro implements the default of a simple type value tag.
/// It requires that the target struct has 2 fields,
/// an `id` as `u64` and `value` as the target type.
///
/// It defines the following functions:
/// - `new() -> Self`;
/// - `with_value(value: $value_type) - Self`;
/// - `with_id() -> Self`;
/// - `with_id_value(id: u64, value: bool) -> Self`;
/// - `value(&self) -> $value_type`;
/// - `value(&self) -> $value_type`;
///
/// As such, it can be used as follows:
///
/// ```
/// pub struct ILInt8 {
///     id: u64,
///     value: u8,
/// }
///
/// simple_value_tag_struct_impl(ILInt8, i8, IL_INT8_TAG_ID);
/// ```
///
/// Arguments:
/// - `$value_type`: Type of the target value (e.g.: u8, i8, etc);
/// - `$default_id`: The default tag id (e.g: IL_INT8_TAG_ID);
macro_rules! simple_value_tag_struct_impl {
    ($tag_type: ty, $value_type: ty, $default_id: ident) => {
        impl $tag_type {
            /// Constructs this struct using the default tag id and
            /// value.
            pub fn new() -> Self {
                Self::with_id($default_id)
            }

            /// Constructs this struct using the default tag id.
            ///
            /// Arguments:
            /// - `value`: The initial value;
            pub fn with_value(value: $value_type) -> Self {
                Self::with_id_value($default_id, value)
            }

            /// Constructs this struct using the given tag id and
            /// default value.
            ///
            /// Arguments:
            /// - `id`: The specified id;
            pub fn with_id(id: u64) -> Self {
                Self {
                    id,
                    value: <$value_type>::default(),
                }
            }

            /// Constructs this struct using the given tag id and
            /// default value.
            ///
            /// Arguments:
            /// - `id`: The specified id;
            /// - `value`: The initial value;
            pub fn with_id_value(id: u64, value: $value_type) -> Self {
                Self { id, value }
            }

            /// Returns the current value of this tag.
            ///
            /// Returns:
            /// - The current value of the tag.
            pub fn value(&self) -> $value_type {
                self.value
            }

            /// Sets the current value of this tag.
            ///
            /// Arguments:
            /// - `value`: The initial value;
            pub fn set_value(&mut self, value: $value_type) {
                self.value = value
            }
        }
    };
}

/// This macro creates the ILTag implementation for integer values.
///
/// Arguments:
/// - `$tag_type`: Name of the tag struct;
/// - `$value_size`: Size of the value;
/// - `$read_func`: Integer read function from `crate::io::data`;
/// - `$write_func`: Integer write function from `crate::io::data`;
macro_rules! int_iltag_impl {
    ($tag_type: ty, $value_size: expr) => {
        impl ILTag for $tag_type {
            iltag_base_func_impl!();

            fn value_size(&self) -> u64 {
                $value_size
            }

            fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
                writer.serialize_value(self.value)
            }

            fn deserialize_value(
                &mut self,
                _factory: &dyn ILTagFactory,
                value_size: usize,
                reader: &mut dyn Reader,
            ) -> Result<()> {
                match value_size {
                    $value_size => (),
                    _ => return Err(ErrorKind::CorruptedData),
                };
                self.value = reader.deserialize_value()?;
                Ok(())
            }
        }
    };
}

/// Returns the size of the implicit tag.
///
/// With the exception of `ILILInt64Tag`, all implicity tags
/// have fixed size. For `ILILInt64Tag`, it will always
/// return its maximum size which is 9.
///
/// Arguments:
/// - id: The tag id;
///
/// Returns:
/// - The size of the tag in bytes or 0 if the id is
/// not a valid implicit tag.
pub fn implicit_tag_size(id: u64) -> u64 {
    match id {
        IL_NULL_TAG_ID => 0,
        IL_BOOL_TAG_ID => 1,
        IL_INT8_TAG_ID => 1,
        IL_UINT8_TAG_ID => 1,
        IL_INT16_TAG_ID => 2,
        IL_UINT16_TAG_ID => 2,
        IL_INT32_TAG_ID => 4,
        IL_UINT32_TAG_ID => 4,
        IL_INT64_TAG_ID => 8,
        IL_UINT64_TAG_ID => 8,
        IL_ILINT_TAG_ID => 9,
        IL_BIN32_TAG_ID => 4,
        IL_BIN64_TAG_ID => 8,
        IL_BIN128_TAG_ID => 16,
        IL_SIGNED_ILINT_TAG_ID => 9,
        _ => 0,
    }
}

//=============================================================================
// ILNullTag
//-----------------------------------------------------------------------------
/// This struct implements the null standard tag.
///
/// By default it sets the tag id to [`IL_NULL_TAG_ID`].
pub struct ILNullTag {
    id: u64,
}

impl ILNullTag {
    pub fn new() -> ILNullTag {
        ILNullTag::with_id(IL_NULL_TAG_ID)
    }

    pub fn with_id(id: u64) -> ILNullTag {
        ILNullTag { id }
    }
}

impl ILTag for ILNullTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        0
    }

    fn serialize_value(&self, _writer: &mut dyn Writer) -> Result<()> {
        Ok(())
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        _reader: &mut dyn Reader,
    ) -> Result<()> {
        match value_size {
            0 => Ok(()),
            _ => Err(ErrorKind::CorruptedData),
        }
    }
}

iltag_default_impl!(ILNullTag);

//=============================================================================
// ILBoolTag
//-----------------------------------------------------------------------------
/// This struct implements the boolean standard tag.
///
/// By default it sets the tag id to [`IL_BOOL_TAG_ID`].
pub struct ILBoolTag {
    id: u64,
    value: bool,
}

simple_value_tag_struct_impl!(ILBoolTag, bool, IL_BOOL_TAG_ID);

impl ILTag for ILBoolTag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        1
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        writer.serialize_value(if self.value { 1 } else { 0 } as u8)
    }

    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        match value_size {
            1 => (),
            _ => return Err(ErrorKind::CorruptedData),
        };
        let v: u8 = reader.deserialize_value()?;
        self.value = match v {
            0 => false,
            1 => true,
            _ => return Err(ErrorKind::CorruptedData),
        };
        Ok(())
    }
}

iltag_default_impl!(ILBoolTag);

//=============================================================================
// ILInt8Tag
//-----------------------------------------------------------------------------
/// This struct implements the i8 standard tag.
///
/// By default it sets the tag id to [`IL_INT8_TAG_ID`].
pub struct ILInt8Tag {
    id: u64,
    value: i8,
}

simple_value_tag_struct_impl!(ILInt8Tag, i8, IL_INT8_TAG_ID);

int_iltag_impl!(ILInt8Tag, 1);

iltag_default_impl!(ILInt8Tag);

//=============================================================================
// ILUInt8Tag
//-----------------------------------------------------------------------------
/// This struct implements the u8 standard tag.
///
/// By default it sets the tag id to [`IL_UINT8_TAG_ID`].
pub struct ILUInt8Tag {
    id: u64,
    value: u8,
}

simple_value_tag_struct_impl!(ILUInt8Tag, u8, IL_UINT8_TAG_ID);

int_iltag_impl!(ILUInt8Tag, 1);

iltag_default_impl!(ILUInt8Tag);

//=============================================================================
// ILInt16Tag
//-----------------------------------------------------------------------------
/// This struct implements the i16 standard tag.
///
/// By default it sets the tag id to [`IL_INT16_TAG_ID`].
pub struct ILInt16Tag {
    id: u64,
    value: i16,
}

simple_value_tag_struct_impl!(ILInt16Tag, i16, IL_INT16_TAG_ID);

int_iltag_impl!(ILInt16Tag, 2);

iltag_default_impl!(ILInt16Tag);

//=============================================================================
// ILUInt16Tag
//-----------------------------------------------------------------------------
/// This struct implements the u16 standard tag.
///
/// By default it sets the tag id to [`IL_UINT16_TAG_ID`].
pub struct ILUInt16Tag {
    id: u64,
    value: u16,
}

simple_value_tag_struct_impl!(ILUInt16Tag, u16, IL_UINT16_TAG_ID);

int_iltag_impl!(ILUInt16Tag, 2);

iltag_default_impl!(ILUInt16Tag);

//=============================================================================
// ILInt32Tag
//-----------------------------------------------------------------------------
/// This struct implements the i32 standard tag.
///
/// By default it sets the tag id to [`IL_INT32_TAG_ID`].
pub struct ILInt32Tag {
    id: u64,
    value: i32,
}

simple_value_tag_struct_impl!(ILInt32Tag, i32, IL_INT32_TAG_ID);

int_iltag_impl!(ILInt32Tag, 4);

iltag_default_impl!(ILInt32Tag);

//=============================================================================
// ILUInt32Tag
//-----------------------------------------------------------------------------
/// This struct implements the u32 standard tag.
///
/// By default it sets the tag id to [`IL_UINT32_TAG_ID`].
pub struct ILUInt32Tag {
    id: u64,
    value: u32,
}

simple_value_tag_struct_impl!(ILUInt32Tag, u32, IL_UINT32_TAG_ID);

int_iltag_impl!(ILUInt32Tag, 4);

iltag_default_impl!(ILUInt32Tag);

//=============================================================================
// ILInt64Tag
//-----------------------------------------------------------------------------
/// This struct implements the i64 standard tag.
///
/// By default it sets the tag id to [`IL_INT64_TAG_ID`].
pub struct ILInt64Tag {
    id: u64,
    value: i64,
}

simple_value_tag_struct_impl!(ILInt64Tag, i64, IL_INT64_TAG_ID);

int_iltag_impl!(ILInt64Tag, 8);

iltag_default_impl!(ILInt64Tag);

//=============================================================================
// ILUInt64Tag
//-----------------------------------------------------------------------------
/// This struct implements the u64 standard tag.
///
/// By default it sets the tag id to [`IL_UINT64_TAG_ID`].
pub struct ILUInt64Tag {
    id: u64,
    value: u64,
}

simple_value_tag_struct_impl!(ILUInt64Tag, u64, IL_UINT64_TAG_ID);

int_iltag_impl!(ILUInt64Tag, 8);

iltag_default_impl!(ILUInt64Tag);

//=============================================================================
// ILBin32Tag
//-----------------------------------------------------------------------------
/// This struct implements the bin32 (f32) standard tag.
///
/// By default it sets the tag id to [`IL_BIN32_TAG_ID`].
pub struct ILBin32Tag {
    id: u64,
    value: f32,
}

simple_value_tag_struct_impl!(ILBin32Tag, f32, IL_BIN32_TAG_ID);

int_iltag_impl!(ILBin32Tag, 4);

iltag_default_impl!(ILBin32Tag);

//=============================================================================
// ILBin64Tag
//-----------------------------------------------------------------------------
/// This struct implements the bin64 (f64) standard tag.
///
/// By default it sets the tag id to [`IL_BIN64_TAG_ID`].
pub struct ILBin64Tag {
    id: u64,
    value: f64,
}

simple_value_tag_struct_impl!(ILBin64Tag, f64, IL_BIN64_TAG_ID);

int_iltag_impl!(ILBin64Tag, 8);

iltag_default_impl!(ILBin64Tag);

//=============================================================================
// ILILint64Tag
//-----------------------------------------------------------------------------
/// This struct implements the bin128 standard tag. Since Rust does not
/// implement the IEEE754 128-bit floating point yet, this tag only holds its
/// raw bytes.
///
/// By default it sets the tag id to [`IL_BIN128_TAG_ID`].
pub struct ILBin128Tag {
    id: u64,
    value: [u8; 16],
}

impl ILBin128Tag {
    /// Constructs this struct using the default tag id and
    /// value.
    pub fn new() -> Self {
        Self::with_id(IL_BIN128_TAG_ID)
    }

    /// Constructs this struct using the default tag id.
    ///
    /// Arguments:
    /// - `value`: The initial value;
    pub fn with_value(value: &[u8; 16]) -> Self {
        Self::with_id_value(IL_BIN128_TAG_ID, value)
    }

    /// Constructs this struct using the given tag id and
    /// default value.
    ///
    /// Arguments:
    /// - `id`: The specified id;
    pub fn with_id(id: u64) -> Self {
        Self { id, value: [0; 16] }
    }

    /// Constructs this struct using the given tag id and
    /// default value.
    ///
    /// Arguments:
    /// - `id`: The specified id;
    /// - `value`: The initial value;
    pub fn with_id_value(id: u64, value: &[u8; 16]) -> Self {
        assert!(value.len() == 16);
        let mut inst = Self { id, value: [0; 16] };
        inst.value.copy_from_slice(value);
        inst
    }

    /// Returns the current value of this tag.
    ///
    /// Returns:
    /// - The current value of the tag.
    pub fn value(&self) -> &[u8; 16] {
        &self.value
    }

    /// Sets the current value of this tag.
    ///
    /// Arguments:
    /// - `value`: The initial value. Must be an ;
    pub fn set_value(&mut self, value: &[u8; 16]) {
        self.value.copy_from_slice(value);
    }
}

impl ILTag for ILBin128Tag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        16
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        match writer.write_all(&self.value) {
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
        match value_size {
            16 => (),
            _ => return Err(ErrorKind::CorruptedData),
        }
        match reader.read_all(&mut self.value) {
            Ok(v) => v,
            Err(e) => return Err(ErrorKind::IOError(e)),
        };
        Ok(())
    }
}

iltag_default_impl!(ILBin128Tag);

//=============================================================================
// ILILint64Tag
//-----------------------------------------------------------------------------
/// This struct implements the ILInt standard tag.  This tag and [`ILSignedILInt64Tag`]
/// are the only implicit tags whose its value size can vary from 1 to 9 bytes.
///
/// By default it sets the tag id to [`IL_ILINT_TAG_ID`].
pub struct ILILInt64Tag {
    id: u64,
    value: u64,
}

simple_value_tag_struct_impl!(ILILInt64Tag, u64, IL_ILINT_TAG_ID);

impl ILTag for ILILInt64Tag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        crate::ilint::encoded_size(self.value) as u64
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        writer.serialize_ilint(self.value)
    }

    /// This implementation follows the specification of `ILTag::deserialize_value()`
    /// except for the fact that it does not check the `value_size` as it may vary
    /// from 1 to 9 bytes depending on the actual value stored.
    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        _value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        self.value = reader.deserialize_ilint()?;
        Ok(())
    }
}

iltag_default_impl!(ILILInt64Tag);

//=============================================================================
// ILSignedILint64Tag
//-----------------------------------------------------------------------------
/// This struct implements the ILInt standard tag. This tag and [`ILILInt64Tag`]
/// are the only implicit tags whose its value size can vary from 1 to 9 bytes.
///
/// By default it sets the tag id to [`IL_SIGNED_ILINT_TAG_ID`].
///
/// New since 1.3.0.
pub struct ILSignedILInt64Tag {
    id: u64,
    value: i64,
}

simple_value_tag_struct_impl!(ILSignedILInt64Tag, i64, IL_SIGNED_ILINT_TAG_ID);

impl ILTag for ILSignedILInt64Tag {
    iltag_base_func_impl!();

    fn value_size(&self) -> u64 {
        crate::ilint::signed_encoded_size(self.value) as u64
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        writer.serialize_signed_ilint(self.value)
    }

    /// This implementation follows the specification of `ILTag::deserialize_value()`
    /// except for the fact that it does not check the `value_size` as it may vary
    /// from 1 to 9 bytes depending on the actual value stored.
    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        _value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        self.value = reader.deserialize_signed_ilint()?;
        Ok(())
    }
}

iltag_default_impl!(ILSignedILInt64Tag);
