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
//! This module contains the implementation of the **IL2 ILInt** standard
//! as defined in [ILInt Specification](https://github.com/interlockledger/specification/tree/master/ILInt)
//!
//! This code is based on the **IL2 ILInt** implementation for Rust found at
//! [InterlockLedger ILInt for Rust](https://github.com/interlockledger/rust-il2-ilint).
//! This version does not share the same interface of the original library and thus
//! is not compatible with it.
#[cfg(test)]
mod tests;

use super::io::{Reader, Writer};

/// Error codes generated by this module.
pub enum ErrorKind {
    /// The encoded value is larger than 2^64 - 1.
    ValueOverflow,
    InvalidFormat,
    /// I/O error.
    IOError(crate::io::ErrorKind),
}

/// Alias to the errors .
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// LInt base value. All values smaller than this value are encoded as
/// a single byte.
pub const ILINT_BASE: u8 = 0xF8;

/// Value of ILINT_BASE as U64.
pub const ILINT_BASE_U64: u64 = ILINT_BASE as u64;

/// Returns the size of the given value encoded as an ILInt.
///
/// Arguments:
///
/// * `value` : The value to be encoded.
///
/// Returns:
///
/// * The number of bytes required to encode the value.
///
pub fn encoded_size(value: u64) -> usize {
    match value {
        value if value < ILINT_BASE_U64 => 1,
        value if value <= (0xFF + ILINT_BASE_U64) => 2,
        value if value <= (0xFFFF + ILINT_BASE_U64) => 3,
        value if value <= (0x00FF_FFFF + ILINT_BASE_U64) => 4,
        value if value <= (0xFFFF_FFFF + ILINT_BASE_U64) => 5,
        value if value <= (0x00FF_FFFF_FFFF + ILINT_BASE_U64) => 6,
        value if value <= (0xFFFF_FFFF_FFFF + ILINT_BASE_U64) => 7,
        value if value <= (0x00FF_FFFF_FFFF_FFFF + ILINT_BASE_U64) => 8,
        _ => 9,
    }
}

/// Encodes the given value into a ILInt value.
///
/// Arguments:
///
/// * `value`: The value to be encoded;
/// * `writer`: The writer that will receive the encoded value;
///
/// Returns:
///
/// * `Ok(())`: In case of success.
/// * `Err(ErrorKind)`: In case of an I/O error.
///
pub fn encode(value: u64, writer: &mut dyn Writer) -> Result<()> {
    let size = encoded_size(value);
    if size == 1 {
        match writer.write(value as u8) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
    } else {
        // Header
        match writer.write((ILINT_BASE + ((size - 2) as u8)) as u8) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
        let v = value - ILINT_BASE_U64;
        let mut shift = 8 * (size - 1);
        for _i in 1..size {
            shift -= 8;
            match writer.write(((v >> shift) & 0xFF) as u8) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            }
        }
    }
    Ok(())
}

/// Determines the size of the ILInt based on its header (the
/// first byte).
///
/// Arguments:
///
/// * `header`: The header of the ILInt. It is always the first byte of
///   the ILInt value;
///
/// Returns:
///
/// * The size of the ILInt in bytes, including the header.
///
pub fn decoded_size(header: u8) -> usize {
    if header < ILINT_BASE {
        1
    } else {
        (header - ILINT_BASE + 2) as usize
    }
}

/// Decodes the body of a multi-byte ILInt.
///
/// Arguments:
/// * `body`: The multibyte
///
/// Returns:
///
/// * `Ok(u64)`: The value of the ILInt.
/// * `Err(ErrorKind)`: In case of error.
pub fn decode_body(body: &[u8]) -> Result<u64> {
    let mut v: u64 = 0;

    if body.is_empty() || body.len() > 8 {
        Err(ErrorKind::InvalidFormat)
    } else {
        for x in body {
            v = (v << 8) + (*x as u64);
        }
        if v > 0xFFFF_FFFF_FFFF_FF07 {
            Err(ErrorKind::ValueOverflow)
        } else {
            Ok(v + ILINT_BASE_U64)
        }
    }
}

/// Decodes an ILInt from a byte slice.
///
/// Arguments:
/// * `value`: The ILInt value.
///
/// Returns:
///
/// * `Ok((u64,usize))`: The value of the ILInt and the number of bytes used.
/// * `Err(ErrorKind)`: In case of error.
pub fn decode_from_bytes(value: &[u8]) -> Result<(u64, usize)> {
    if value.is_empty() {
        Err(ErrorKind::InvalidFormat)
    } else {
        let size = decoded_size(value[0]);
        if size == 1 {
            Ok((value[0] as u64, size))
        } else if value.len() < size {
            Err(ErrorKind::InvalidFormat)
        } else {
            match decode_body(&value[1..size]) {
                Ok(v) => Ok((v, size)),
                Err(x) => Err(x),
            }
        }
    }
}

/// Decodes an ILInt value.
///
/// Arguments:
///
/// * `reader`: The reader that contains the encoded
/// value;
///
/// Returns:
///
/// * `Ok(u64)`: On success, returns the value read.
/// * `Err(ErrorKind)`: In case of error.  
///
pub fn decode(reader: &mut dyn Reader) -> Result<u64> {
    let header = match reader.read() {
        Ok(v) => v,
        Err(e) => return Err(ErrorKind::IOError(e)),
    };
    let size = decoded_size(header);
    if size == 1 {
        Ok(header as u64)
    } else {
        let mut tmp: [u8; 8] = [0; 8];
        match reader.read_all(&mut tmp[0..size - 1]) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
        decode_body(&tmp[0..size - 1])
    }
}
