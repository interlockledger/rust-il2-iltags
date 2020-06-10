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
#[cfg(test)]
mod tests;

use super::io::{Reader, Writer};

pub enum ErrorKind {
    ValueOverflow,
    IOError(crate::io::ErrorKind),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

/// LInt base value. All values smaller than this value are encoded as
/// a single byte.
pub const ILINT_BASE:u8 = 0xF8;

/// Value of ILINT_BASE as U64.
pub const ILINT_BASE_U64:u64 = ILINT_BASE as u64;

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

    if value < ILINT_BASE_U64 {
		1
	} else if value <= (0xFF + ILINT_BASE_U64) {
		2
	} else if value <= (0xFFFF + ILINT_BASE_U64) {
		3
	} else if value <= (0xFFFFFF + ILINT_BASE_U64){
		4
	} else if value <= (0xFFFFFFFF + ILINT_BASE_U64){
		5
	} else if value <= (0xFFFFFFFFFF + ILINT_BASE_U64){
		6
	} else if value <= (0xFFFFFFFFFFFF + ILINT_BASE_U64){
		7
	} else if value <= (0xFFFFFFFFFFFFFF + ILINT_BASE_U64){
		8
	} else {
		9
	}
}

/// Encodes the given value into a ILInt value.
/// 
/// Arguments:
///
/// * `value`: The value to be encoded;
/// * `enc`: The byte array that will receive the encoded value.
/// It must have at least encoded_size(value) bytes;
/// 
/// Returns:
/// 
/// * `Ok(size)`: The number of bytes used.
/// * `Err(())`: If the buffer is too small to hold the encoded value.
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
/// * The size of the ILInt in bytes.
/// 
pub fn decoded_size(header : u8) -> usize {
    
    if header < ILINT_BASE {
        1
    } else {
        (header - ILINT_BASE + 2) as usize
    }
}

/// Decodes an ILInt value.
/// 
/// Arguments:
///
/// * `value`: The ILInt value. It must have at least 1
///   byte;
/// 
/// Returns:
/// 
/// * The size of the ILInt in bytes.
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
        let mut v:u64 = 0;
        for _i in 1 .. size {
            let r = match reader.read() {
                Ok(v) => v,
                Err(e) => return Err(ErrorKind::IOError(e)),
            };
            v = (v << 8) + (r as u64);
        }
        if v > 0xFFFFFFFFFFFFFF07 {
            Err(ErrorKind::ValueOverflow)
        } else {
            Ok(v + ILINT_BASE_U64)
        }
    }
}
