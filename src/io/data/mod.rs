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
//! This module implements extension traits for [`Reader`] and [`Writer`] that
//! allows the manipulation of basic data types as defined by the **ILTags**
//! standard.
#[cfg(test)]
mod tests;

use super::{ErrorKind, Reader, Result, Writer};
use crate::ilint::{decode, encode};

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
