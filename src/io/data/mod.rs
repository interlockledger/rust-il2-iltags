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
//! This module implements functions and traits for [`Reader`] and [`Writer`]
//! that allows the manipulation of basic data types as defined by the
//! **ILTags** standard.
//!
//! All interger types are handled as being encoded as Big Endian values as
//! defined by the **ILTags** specification.
#[cfg(test)]
pub mod test_samples;
#[cfg(test)]
mod tests;

use super::{ErrorKind, Reader, Result, Writer};
use crate::ilint::{decode, encode, signed_decode, signed_encode};

/// Extracts an `u8` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_u8(reader: &mut dyn Reader) -> Result<u8> {
    reader.read()
}

/// Extracts an `i8` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_i8(reader: &mut dyn Reader) -> Result<i8> {
    match reader.read() {
        Ok(v) => Ok(v as i8),
        Err(e) => Err(e),
    }
}

/// Extracts an `u16` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_u16(reader: &mut dyn Reader) -> Result<u16> {
    let mut tmp: [u8; 2] = [0; 2];
    reader.read_all(&mut tmp)?;
    Ok(u16::from_be_bytes(tmp))
}

/// Extracts an `i16` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_i16(reader: &mut dyn Reader) -> Result<i16> {
    let mut tmp: [u8; 2] = [0; 2];
    reader.read_all(&mut tmp)?;
    Ok(i16::from_be_bytes(tmp))
}

/// Extracts an `u32` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_u32(reader: &mut dyn Reader) -> Result<u32> {
    let mut tmp: [u8; 4] = [0; 4];
    reader.read_all(&mut tmp)?;
    Ok(u32::from_be_bytes(tmp))
}

/// Extracts an `i32` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_i32(reader: &mut dyn Reader) -> Result<i32> {
    let mut tmp: [u8; 4] = [0; 4];
    reader.read_all(&mut tmp)?;
    Ok(i32::from_be_bytes(tmp))
}

/// Extracts an `u64` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_u64(reader: &mut dyn Reader) -> Result<u64> {
    let mut tmp: [u8; 8] = [0; 8];
    reader.read_all(&mut tmp)?;
    Ok(u64::from_be_bytes(tmp))
}

/// Extracts an `i64` from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_i64(reader: &mut dyn Reader) -> Result<i64> {
    let mut tmp: [u8; 8] = [0; 8];
    reader.read_all(&mut tmp)?;
    Ok(i64::from_be_bytes(tmp))
}

/// Extracts an ILInt value from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_ilint(reader: &mut dyn Reader) -> Result<u64> {
    match decode(reader) {
        Ok(value) => Ok(value),
        Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
        _ => Err(ErrorKind::CorruptedData),
    }
}

/// Extracts a signed ILInt value from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
///
/// New since 1.3.0.
pub fn read_signed_ilint(reader: &mut dyn Reader) -> Result<i64> {
    match signed_decode(reader) {
        Ok(value) => Ok(value),
        Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
        _ => Err(ErrorKind::CorruptedData),
    }
}

/// Extracts an `f32` from the specified [`Reader`]. It is
/// always expected to be encoded as a `binary32`
/// from *IEEE 754-2008*.
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_f32(reader: &mut dyn Reader) -> Result<f32> {
    let tmp: u32 = read_u32(reader)?;
    Ok(f32::from_bits(tmp))
}

/// Extracts an `f64` from the specified [`Reader`]. It is
/// always expected to be encoded as a `binary64`
/// from *IEEE 754-2008*.
///
/// Arguments:
/// - `reader`: The [`Reader`];
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_f64(reader: &mut dyn Reader) -> Result<f64> {
    let tmp: u64 = read_u64(reader)?;
    Ok(f64::from_bits(tmp))
}

/// Extracts an UTF-8 string from the specified [`Reader`].
///
/// Arguments:
/// - `reader`: The [`Reader`];
/// - `size`: The size in bytes;
///
/// Returns:
/// - Ok(v): The value read;
/// - Err(_): If the value could not be extracted;
pub fn read_string(reader: &mut dyn Reader, size: usize) -> Result<String> {
    let mut tmp: Vec<u8> = vec![0; size];
    reader.read_all(tmp.as_mut_slice())?;
    match String::from_utf8(tmp) {
        Ok(s) => Ok(s),
        _ => Err(ErrorKind::CorruptedData),
    }
}

/// Writes an `u8` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_u8(v: u8, writer: &mut dyn Writer) -> Result<()> {
    writer.write(v)
}

/// Writes an `i8` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_i8(v: i8, writer: &mut dyn Writer) -> Result<()> {
    writer.write(v as u8)
}

/// Writes an `u16` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_u16(v: u16, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `i16` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_i16(v: i16, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `u32` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_u32(v: u32, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `i32` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_i32(v: i32, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `u64` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_u64(v: u64, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `i64` value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_i64(v: i64, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an ILInt value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_ilint(v: u64, writer: &mut dyn Writer) -> Result<()> {
    match encode(v, writer) {
        Ok(()) => Ok(()),
        Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
        _ => Err(ErrorKind::UnableToWriteData),
    }
}

/// Writes a signed ILInt value to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
///
/// New since 1.2.1
pub fn write_signed_ilint(v: i64, writer: &mut dyn Writer) -> Result<()> {
    match signed_encode(v, writer) {
        Ok(()) => Ok(()),
        Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
        _ => Err(ErrorKind::UnableToWriteData),
    }
}

/// Writes an `f32` value to the specified [`Writer`]. It is
/// always encoded as a `binary32` from *IEEE 754-2008*.
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_f32(v: f32, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an `f64` value to the specified [`Writer`]. It is
/// always encoded as a `binary64` from *IEEE 754-2008*.
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_f64(v: f64, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.to_be_bytes())
}

/// Writes an UTF-8 string to the specified [`Writer`].
///
/// Arguments:
/// - `v`: The value;
/// - `writer`: The [`Writer`];
///
/// Returns:
/// - Ok(()): For success;
/// - Err(_): For failure;
pub fn write_string(v: &str, writer: &mut dyn Writer) -> Result<()> {
    writer.write_all(&v.as_bytes())
}

//=============================================================================
// ValueReader
//-----------------------------------------------------------------------------
/// This trait adds the ability to read fixed size values.
///
/// Since 1.1.0.
pub trait ValueReader<T: Sized + Copy> {
    /// Reads the value.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn read_value(&mut self) -> Result<T>;
}

// Default implementation of ValueReader for the concrete type and dyn Reader.
macro_rules! valuereader_def_impl {
    ($type: ty, $read_func: ident) => {
        impl ValueReader<$type> for dyn Reader + '_ {
            #[inline]
            fn read_value(&mut self) -> Result<$type> {
                $read_func(self)
            }
        }

        impl<T: Reader> ValueReader<$type> for T {
            #[inline]
            fn read_value(&mut self) -> Result<$type> {
                $read_func(self)
            }
        }
    };
}

valuereader_def_impl!(u8, read_u8);
valuereader_def_impl!(i8, read_i8);
valuereader_def_impl!(u16, read_u16);
valuereader_def_impl!(i16, read_i16);
valuereader_def_impl!(u32, read_u32);
valuereader_def_impl!(i32, read_i32);
valuereader_def_impl!(u64, read_u64);
valuereader_def_impl!(i64, read_i64);
valuereader_def_impl!(f32, read_f32);
valuereader_def_impl!(f64, read_f64);

//=============================================================================
// ILIntReader
//-----------------------------------------------------------------------------
/// This trait adds the ability to read ILInt values.
pub trait ILIntReader {
    /// Reads the ILInt.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn read_ilint(&mut self) -> Result<u64>;
}

impl ILIntReader for dyn Reader + '_ {
    #[inline]
    fn read_ilint(&mut self) -> Result<u64> {
        read_ilint(self)
    }
}

impl<T: Reader> ILIntReader for T {
    #[inline]
    fn read_ilint(&mut self) -> Result<u64> {
        read_ilint(self)
    }
}

//=============================================================================
// SignedILIntReader
//-----------------------------------------------------------------------------
/// This trait adds the ability to read signed ILInt values.
///
/// New since 1.3.0.
pub trait SignedILIntReader {
    /// Reads the ILInt.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn read_signed_ilint(&mut self) -> Result<i64>;
}

impl SignedILIntReader for dyn Reader + '_ {
    #[inline]
    fn read_signed_ilint(&mut self) -> Result<i64> {
        read_signed_ilint(self)
    }
}

impl<T: Reader> SignedILIntReader for T {
    #[inline]
    fn read_signed_ilint(&mut self) -> Result<i64> {
        read_signed_ilint(self)
    }
}

//=============================================================================
// StringValueReader
//-----------------------------------------------------------------------------
/// This trait adds the ability to read UTF-8 String values.
pub trait StringValueReader {
    /// Reads an UTF-8 String with a given size.
    ///
    /// Arguments:
    /// - `size`: Number of bytes to read.
    ///
    /// Returns:
    /// - Ok(v): For success;
    /// - Err(_): For failure;
    fn read_string(&mut self, size: usize) -> Result<String>;
}

impl StringValueReader for dyn Reader + '_ {
    #[inline]
    fn read_string(&mut self, size: usize) -> Result<String> {
        read_string(self, size)
    }
}

impl<T: Reader> StringValueReader for T {
    #[inline]
    fn read_string(&mut self, size: usize) -> Result<String> {
        read_string(self, size)
    }
}

//=============================================================================
// ValueWriter
//-----------------------------------------------------------------------------
/// This trait adds the ability to write fixed size values.
pub trait ValueWriter<T> {
    /// Writes the value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;
    fn write_value(&mut self, value: T) -> Result<()>;
}

// Default implementation of ValueWriter for the concrete type and dyn Writer.
macro_rules! valuewriter_def_impl {
    ($type: ty, $write_func: ident) => {
        impl ValueWriter<$type> for dyn Writer + '_ {
            #[inline]
            fn write_value(&mut self, value: $type) -> Result<()> {
                $write_func(value, self)
            }
        }

        impl<T: Writer> ValueWriter<$type> for T {
            #[inline]
            fn write_value(&mut self, value: $type) -> Result<()> {
                $write_func(value, self)
            }
        }
    };
}

valuewriter_def_impl!(u8, write_u8);
valuewriter_def_impl!(i8, write_i8);
valuewriter_def_impl!(u16, write_u16);
valuewriter_def_impl!(i16, write_i16);
valuewriter_def_impl!(u32, write_u32);
valuewriter_def_impl!(i32, write_i32);
valuewriter_def_impl!(u64, write_u64);
valuewriter_def_impl!(i64, write_i64);
valuewriter_def_impl!(f32, write_f32);
valuewriter_def_impl!(f64, write_f64);
valuewriter_def_impl!(&str, write_string);

//=============================================================================
// ILIntWriter
//-----------------------------------------------------------------------------
/// This trait adds the ability to write ILInt values.
pub trait ILIntWriter {
    /// Writes the ILInt value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;    
    fn write_ilint(&mut self, value: u64) -> Result<()>;
}

impl ILIntWriter for dyn Writer + '_ {
    #[inline]
    fn write_ilint(&mut self, value: u64) -> Result<()> {
        write_ilint(value, self)
    }
}

impl<T: Writer> ILIntWriter for T {
    #[inline]
    fn write_ilint(&mut self, value: u64) -> Result<()> {
        write_ilint(value, self)
    }
}

//=============================================================================
// SignedILIntWriter
//-----------------------------------------------------------------------------
/// This trait adds the ability to write signed ILInt values.
///
/// New since 1.2.1
pub trait SignedILIntWriter {
    /// Writes the ILInt value.
    ///
    /// Arguments:
    /// - `value`: The value to write;
    ///
    /// Returns:
    /// - Ok(()): For success;
    /// - Err(_): For failure;    
    fn write_signed_ilint(&mut self, value: i64) -> Result<()>;
}

impl SignedILIntWriter for dyn Writer + '_ {
    #[inline]
    fn write_signed_ilint(&mut self, value: i64) -> Result<()> {
        write_signed_ilint(value, self)
    }
}

impl<T: Writer> SignedILIntWriter for T {
    #[inline]
    fn write_signed_ilint(&mut self, value: i64) -> Result<()> {
        write_signed_ilint(value, self)
    }
}
