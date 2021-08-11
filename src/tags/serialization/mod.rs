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
//! This module implements serializer and deserializer traits. They are similar to
//! their counterparts from [`crate::io::data`] but returns [`crate::tags::Result`]
//! instead of [`crate::io::Result`]. Furthermore, they also adds the ability to
//! serialize and deserialize bytes and other usefull data types.
#[cfg(test)]
mod tests;

use super::Result;
use crate::io::data::*;
use crate::io::{Reader, Writer};

//=============================================================================
// ValueDeserializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to deserialize fixed size values.
///
/// Since 1.2.0.
pub trait ValueDeserializer<T: Sized + Copy> {
    /// Deserializes the value.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn deserialize_value(&mut self) -> Result<T>;
}

impl<T: Sized + Copy> ValueDeserializer<T> for dyn ValueReader<T> + '_ {
    #[inline]
    fn deserialize_value(&mut self) -> Result<T> {
        let v: T = self.read_value()?;
        Ok(v)
    }
}

impl<T: Sized + Copy, R: ValueReader<T>> ValueDeserializer<T> for R {
    #[inline]
    fn deserialize_value(&mut self) -> Result<T> {
        let v: T = self.read_value()?;
        Ok(v)
    }
}

// Default implementation of ValueDeserializer for dyn Reader.
macro_rules! valuedeserializer_def_impl {
    ($type: ty, $read_func: ident) => {
        impl ValueDeserializer<$type> for dyn Reader + '_ {
            #[inline]
            fn deserialize_value(&mut self) -> Result<$type> {
                let v: $type = $read_func(self)?;
                Ok(v)
            }
        }
    };
}

valuedeserializer_def_impl!(u8, read_u8);
valuedeserializer_def_impl!(i8, read_i8);
valuedeserializer_def_impl!(u16, read_u16);
valuedeserializer_def_impl!(i16, read_i16);
valuedeserializer_def_impl!(u32, read_u32);
valuedeserializer_def_impl!(i32, read_i32);
valuedeserializer_def_impl!(u64, read_u64);
valuedeserializer_def_impl!(i64, read_i64);
valuedeserializer_def_impl!(f32, read_f32);
valuedeserializer_def_impl!(f64, read_f64);

//=============================================================================
// ILIntDeserializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to deserialize ILInt values.
///
/// Since 1.2.0.
pub trait ILIntDeserializer {
    /// Deserializes an ILInt value.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn deserialize_ilint(&mut self) -> Result<u64>;
}

impl ILIntDeserializer for dyn ILIntReader + '_ {
    #[inline]
    fn deserialize_ilint(&mut self) -> Result<u64> {
        let v: u64 = self.read_ilint()?;
        Ok(v)
    }
}

impl<R: ILIntReader> ILIntDeserializer for R {
    #[inline]
    fn deserialize_ilint(&mut self) -> Result<u64> {
        let v: u64 = self.read_ilint()?;
        Ok(v)
    }
}

impl ILIntDeserializer for dyn Reader + '_ {
    #[inline]
    fn deserialize_ilint(&mut self) -> Result<u64> {
        let v: u64 = self.read_ilint()?;
        Ok(v)
    }
}

//=============================================================================
// StringDeserializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to deserialize UTF-8 strings.
///
/// Since 1.2.0.
pub trait StringDeserializer {
    /// Deserializes an UTF-8 String with a given size.
    ///
    /// Arguments:
    /// - `size`: Number of bytes to read.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn deserialize_string(&mut self, size: usize) -> Result<String>;
}

impl StringDeserializer for dyn StringValueReader + '_ {
    #[inline]
    fn deserialize_string(&mut self, size: usize) -> Result<String> {
        let v: String = self.read_string(size)?;
        Ok(v)
    }
}

impl<R: StringValueReader> StringDeserializer for R {
    #[inline]
    fn deserialize_string(&mut self, size: usize) -> Result<String> {
        let v: String = self.read_string(size)?;
        Ok(v)
    }
}

impl StringDeserializer for dyn Reader + '_ {
    #[inline]
    fn deserialize_string(&mut self, size: usize) -> Result<String> {
        let v: String = self.read_string(size)?;
        Ok(v)
    }
}

//=============================================================================
// ByteArrayDeserializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to deserialize byte arrays.
///
/// Since 1.2.0.
pub trait ByteArrayDeserializer {
    /// Deserializes a byte array of a given size.
    ///
    /// Arguments:
    /// - `size`: The number of bytes to read;
    ///
    /// Returns:
    /// - Ok(v): A vector with the bytes read;
    /// - Err(e): In case of error;    
    fn deserialize_bytes(&mut self, size: usize) -> Result<Vec<u8>>;

    /// Deserializes a byte array of a given size into a vector.
    ///
    /// Arguments:
    /// - `size`: The number of bytes to read;
    /// - `vec`: The vector that will receive the data;
    ///
    /// Returns:
    /// - Ok(v): A vector with the bytes read;
    /// - Err(e): In case of error;    
    fn deserialize_bytes_into_vec(&mut self, size: usize, vec: &mut Vec<u8>) -> Result<()> {
        vec.resize(size, 0);
        self.deserialize_bytes_into_slice(vec.as_mut_slice())
    }

    /// Deserializes a byte array of a given size into slice.
    ///
    /// Arguments:
    /// - `buff`: The vector that will receive the data;
    ///
    /// Returns:
    /// - Ok(v): A vector with the bytes read;
    /// - Err(e): In case of error;    
    fn deserialize_bytes_into_slice(&mut self, buff: &mut [u8]) -> Result<()>;
}

impl ByteArrayDeserializer for dyn Reader + '_ {
    #[inline]
    fn deserialize_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut ret: Vec<u8> = vec![0; size];
        self.read_all(ret.as_mut_slice())?;
        Ok(ret)
    }

    #[inline]
    fn deserialize_bytes_into_slice(&mut self, buff: &mut [u8]) -> Result<()> {
        self.read_all(buff)?;
        Ok(())
    }
}

impl<R: Reader> ByteArrayDeserializer for R {
    #[inline]
    fn deserialize_bytes(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut ret: Vec<u8> = vec![0; size];
        self.read_all(ret.as_mut_slice())?;
        Ok(ret)
    }

    #[inline]
    fn deserialize_bytes_into_slice(&mut self, buff: &mut [u8]) -> Result<()> {
        self.read_all(buff)?;
        Ok(())
    }
}

//=============================================================================
// ValueSerializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to serialize values.
///
/// Since 1.2.0.
pub trait ValueSerializer<T> {
    /// Serializes a value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    fn serialize_value(&mut self, value: T) -> Result<()>;
}

impl<T> ValueSerializer<T> for dyn ValueWriter<T> + '_ {
    #[inline]
    fn serialize_value(&mut self, value: T) -> Result<()> {
        self.write_value(value)?;
        Ok(())
    }
}

impl<T, W: ValueWriter<T>> ValueSerializer<T> for W {
    #[inline]
    fn serialize_value(&mut self, value: T) -> Result<()> {
        self.write_value(value)?;
        Ok(())
    }
}

// Default implementation of ValueWriter for the concrete type and dyn Writer.
macro_rules! valueserializer_def_impl {
    ($type: ty, $write_func: ident) => {
        impl ValueSerializer<$type> for dyn Writer + '_ {
            #[inline]
            fn serialize_value(&mut self, value: $type) -> Result<()> {
                $write_func(value, self)?;
                Ok(())
            }
        }
    };
}

valueserializer_def_impl!(u8, write_u8);
valueserializer_def_impl!(i8, write_i8);
valueserializer_def_impl!(u16, write_u16);
valueserializer_def_impl!(i16, write_i16);
valueserializer_def_impl!(u32, write_u32);
valueserializer_def_impl!(i32, write_i32);
valueserializer_def_impl!(u64, write_u64);
valueserializer_def_impl!(i64, write_i64);
valueserializer_def_impl!(f32, write_f32);
valueserializer_def_impl!(f64, write_f64);
valueserializer_def_impl!(&str, write_string);

//=============================================================================
// ILIntSerializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to serialize ILInt values.
///
/// Since 1.2.0.
pub trait ILIntSerializer {
    /// Serializes an ILInt value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    fn serialize_ilint(&mut self, value: u64) -> Result<()>;
}

impl ILIntSerializer for dyn ILIntWriter + '_ {
    #[inline]
    fn serialize_ilint(&mut self, value: u64) -> Result<()> {
        self.write_ilint(value)?;
        Ok(())
    }
}

impl<W: ILIntWriter> ILIntSerializer for W {
    #[inline]
    fn serialize_ilint(&mut self, value: u64) -> Result<()> {
        self.write_ilint(value)?;
        Ok(())
    }
}

impl ILIntSerializer for dyn Writer + '_ {
    #[inline]
    fn serialize_ilint(&mut self, value: u64) -> Result<()> {
        self.write_ilint(value)?;
        Ok(())
    }
}

//=============================================================================
// ByteArraySerializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to serialize byte arrays.
///
/// Since 1.2.0.
pub trait ByteArraySerializer {
    /// Serializes a byte array as a vector.
    ///
    /// This method has been deprecated because because it is equivalent to
    /// [`ByteArraySerializer::serialize_bytes()`].
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    #[inline]
    #[deprecated]
    #[allow(clippy::ptr_arg)]
    fn serialize_byte_vec(&mut self, vec: &Vec<u8>) -> Result<()> {
        self.serialize_bytes(vec)
    }

    /// Serializes a byte array as a slice.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    fn serialize_bytes(&mut self, buff: &[u8]) -> Result<()>;
}

impl ByteArraySerializer for dyn Writer + '_ {
    #[inline]
    fn serialize_bytes(&mut self, buff: &[u8]) -> Result<()> {
        self.write_all(buff)?;
        Ok(())
    }
}

impl<W: Writer> ByteArraySerializer for W {
    #[inline]
    fn serialize_bytes(&mut self, buff: &[u8]) -> Result<()> {
        self.write_all(buff)?;
        Ok(())
    }
}

//=============================================================================
// Legacy functions
//-----------------------------------------------------------------------------
/// Serializes an u64 as an ILInt value.
///
/// Arguments:
/// - `value`: The value to write;
/// - `writer`: The writer;
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(x): In case of error;
pub fn serialize_ilint(value: u64, writer: &mut dyn Writer) -> Result<()> {
    writer.serialize_ilint(value)
}

/// Unserializes an ILInt value.
///
/// Arguments:
/// - `reader`: The reader;
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(x): In case of error;
pub fn deserialize_ilint(reader: &mut dyn Reader) -> Result<u64> {
    reader.deserialize_ilint()
}

/// Serializes a byte array.
///
/// Arguments:
/// - `bytes`: The bytes to be written;
/// - `writer`: The writer;
///
/// Returns:
/// - Ok(()): On success;
/// - Err(e): In case of error;
pub fn serialize_bytes(bytes: &[u8], writer: &mut dyn Writer) -> Result<()> {
    writer.serialize_bytes(bytes)
}

/// Deserializes a byte array of a given size.
///
/// Arguments:
/// - `reader`: The reader;
/// - `size`: The number of bytes to read;
///
/// Returns:
/// - Ok(v): A vector with the bytes read;
/// - Err(e): In case of error;
pub fn deserialize_bytes(size: usize, reader: &mut dyn Reader) -> Result<Vec<u8>> {
    reader.deserialize_bytes(size)
}

/// Deserializes a byte array of a given size into a vector.
///
/// Arguments:
/// - `reader`: The reader;
/// - `size`: The number of bytes to read;
/// - `vec`: The vector that will receive the data;
///
/// Returns:
/// - Ok(v): A vector with the bytes read;
/// - Err(e): In case of error;
pub fn deserialize_bytes_into_vec(
    size: usize,
    reader: &mut dyn Reader,
    vec: &mut Vec<u8>,
) -> Result<()> {
    reader.deserialize_bytes_into_vec(size, vec)
}

//=============================================================================
// SignedILIntDeserializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to deserialize signed ILInt values.
///
/// Since 1.2.1.
pub trait SignedILIntDeserializer {
    /// Deserializes an ILInt value.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn deserialize_signed_ilint(&mut self) -> Result<i64>;
}

impl SignedILIntDeserializer for dyn SignedILIntReader + '_ {
    #[inline]
    fn deserialize_signed_ilint(&mut self) -> Result<i64> {
        let v: i64 = self.read_signed_ilint()?;
        Ok(v)
    }
}

impl<R: SignedILIntReader> SignedILIntDeserializer for R {
    #[inline]
    fn deserialize_signed_ilint(&mut self) -> Result<i64> {
        let v: i64 = self.read_signed_ilint()?;
        Ok(v)
    }
}

impl SignedILIntDeserializer for dyn Reader + '_ {
    #[inline]
    fn deserialize_signed_ilint(&mut self) -> Result<i64> {
        let v: i64 = self.read_signed_ilint()?;
        Ok(v)
    }
}

//=============================================================================
// SignedILIntSerializer
//-----------------------------------------------------------------------------
/// This trait adds the ability to serialize signed ILInt values.
///
/// Since 1.2.0.
pub trait SignedILIntSerializer {
    /// Serializes an ILInt value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    fn serialize_signed_ilint(&mut self, value: i64) -> Result<()>;
}

impl SignedILIntSerializer for dyn SignedILIntWriter + '_ {
    #[inline]
    fn serialize_signed_ilint(&mut self, value: i64) -> Result<()> {
        self.write_signed_ilint(value)?;
        Ok(())
    }
}

impl<W: SignedILIntWriter> SignedILIntSerializer for W {
    #[inline]
    fn serialize_signed_ilint(&mut self, value: i64) -> Result<()> {
        self.write_signed_ilint(value)?;
        Ok(())
    }
}

impl SignedILIntSerializer for dyn Writer + '_ {
    #[inline]
    fn serialize_signed_ilint(&mut self, value: i64) -> Result<()> {
        self.write_signed_ilint(value)?;
        Ok(())
    }
}
