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
//! This module implements some extension traits for Reader and Writer that
//! allows the manipulation of basic data types.

#[cfg(test)]
mod tests;

use super::{ErrorKind, Reader, Result, Writer};
use crate::ilint::{decode, encode};

macro_rules! data_reader_read_be_bytes {
    ($self: ident, $type: ident) => {{
        let mut tmp: [u8; std::mem::size_of::<$type>()] = [0; std::mem::size_of::<$type>()];
        $self.read_all(&mut tmp)?;
        Ok($type::from_be_bytes(tmp))
    }};
}

/// The trait IntDataReader<T> implements defines the
/// ability to read signed and unsigned integer types.
///
/// By default it is implemented for all Readers for
/// u8, u16, u32, u64, i8, i16, i32 and i64.
pub trait IntDataReader<T>: Reader {
    fn read_int(&mut self) -> Result<T>;
}

impl<R: Reader> IntDataReader<u8> for R {
    fn read_int(&mut self) -> Result<u8> {
        self.read()
    }
}

impl<R: Reader> IntDataReader<u16> for R {
    fn read_int(&mut self) -> Result<u16> {
        data_reader_read_be_bytes!(self, u16)
    }
}

impl<R: Reader> IntDataReader<u32> for R {
    fn read_int(&mut self) -> Result<u32> {
        data_reader_read_be_bytes!(self, u32)
    }
}

impl<R: Reader> IntDataReader<u64> for R {
    fn read_int(&mut self) -> Result<u64> {
        data_reader_read_be_bytes!(self, u64)
    }
}

impl<R: Reader> IntDataReader<i8> for R {
    fn read_int(&mut self) -> Result<i8> {
        match self.read() {
            Ok(v) => Ok(v as i8),
            Err(e) => Err(e),
        }
    }
}

impl<R: Reader> IntDataReader<i16> for R {
    fn read_int(&mut self) -> Result<i16> {
        data_reader_read_be_bytes!(self, i16)
    }
}

impl<R: Reader> IntDataReader<i32> for R {
    fn read_int(&mut self) -> Result<i32> {
        data_reader_read_be_bytes!(self, i32)
    }
}

impl<R: Reader> IntDataReader<i64> for R {
    fn read_int(&mut self) -> Result<i64> {
        data_reader_read_be_bytes!(self, i64)
    }
}

pub trait ILIntDataReader: Reader {
    fn read_ilint(&mut self) -> Result<u64>;
}

impl<R: Reader> ILIntDataReader for R {
    fn read_ilint(&mut self) -> Result<u64> {
        match decode(self) {
            Ok(value) => Ok(value),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::CorruptedData),
        }
    }
}

pub trait FloatDataReader<T>: Reader {
    fn read_float(&mut self) -> Result<T>;
}

impl<T: IntDataReader<u32>> FloatDataReader<f32> for T {
    fn read_float(&mut self) -> Result<f32> {
        let tmp: u32 = self.read_int()?;
        Ok(f32::from_bits(tmp))
    }
}

impl<T: IntDataReader<u64>> FloatDataReader<f64> for T {
    fn read_float(&mut self) -> Result<f64> {
        let tmp: u64 = self.read_int()?;
        Ok(f64::from_bits(tmp))
    }
}

pub trait StringDataReader: Reader {
    fn read_string(&mut self, size: usize) -> Result<String>;
}

impl<T: Reader> StringDataReader for T {
    fn read_string(&mut self, size: usize) -> Result<String> {
        let mut tmp: Vec<u8> = vec![0; size];
        self.read_all(tmp.as_mut_slice())?;
        match String::from_utf8(tmp) {
            Ok(s) => Ok(s),
            _ => Err(ErrorKind::CorruptedData),
        }
    }
}

/// The DataReader is a Reader that implements some
/// data functions.
pub trait DataReader:
    IntDataReader<u8>
    + IntDataReader<u16>
    + IntDataReader<u32>
    + IntDataReader<u64>
    + IntDataReader<i8>
    + IntDataReader<i16>
    + IntDataReader<i32>
    + IntDataReader<i64>
    + FloatDataReader<f32>
    + FloatDataReader<f64>
    + ILIntDataReader
    + StringDataReader
{
}

macro_rules! data_writer_write_be_bytes {
    ($self: ident, $value: expr) => {
        $self.write_all(&$value.to_be_bytes())
    };
}

pub trait IntDataWriter<T>: Writer {
    fn write_int(&mut self, v: T) -> Result<()>;
}

impl<T: Writer> IntDataWriter<u8> for T {
    fn write_int(&mut self, v: u8) -> Result<()> {
        self.write(v)
    }
}

impl<T: Writer> IntDataWriter<u16> for T {
    fn write_int(&mut self, v: u16) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> IntDataWriter<u32> for T {
    fn write_int(&mut self, v: u32) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> IntDataWriter<u64> for T {
    fn write_int(&mut self, v: u64) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> IntDataWriter<i8> for T {
    fn write_int(&mut self, v: i8) -> Result<()> {
        self.write(v as u8)
    }
}

impl<T: Writer> IntDataWriter<i16> for T {
    fn write_int(&mut self, v: i16) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> IntDataWriter<i32> for T {
    fn write_int(&mut self, v: i32) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> IntDataWriter<i64> for T {
    fn write_int(&mut self, v: i64) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

pub trait ILIntDataWriter: Writer {
    fn write_ilint(&mut self, v: u64) -> Result<()>;
}

impl<T: Writer> ILIntDataWriter for T {
    fn write_ilint(&mut self, v: u64) -> Result<()> {
        match encode(v, self) {
            Ok(()) => Ok(()),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::UnableToWriteData),
        }
    }
}

pub trait FloatDataWriter<T>: Writer {
    fn write_float(&mut self, v: T) -> Result<()>;
}

impl<T: Writer> FloatDataWriter<f32> for T {
    fn write_float(&mut self, v: f32) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<T: Writer> FloatDataWriter<f64> for T {
    fn write_float(&mut self, v: f64) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

pub trait StringDataWriter {
    fn write_string(&mut self, value: &str) -> Result<()>;
}

impl<T: Writer> StringDataWriter for T {
    fn write_string(&mut self, value: &str) -> Result<()> {
        self.write_all(value.as_bytes())
    }
}

pub trait DataWriter:
    IntDataWriter<u8>
    + IntDataWriter<u16>
    + IntDataWriter<u32>
    + IntDataWriter<u64>
    + IntDataWriter<i8>
    + IntDataWriter<i16>
    + IntDataWriter<i32>
    + IntDataWriter<i64>
    + FloatDataWriter<f32>
    + FloatDataWriter<f64>
    + ILIntDataWriter
    + StringDataWriter
{
}
