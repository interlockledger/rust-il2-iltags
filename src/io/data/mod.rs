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
//! This module implements extension traits for Reader and Writer that
//! allows the manipulation of basic data types.
//#[cfg(test)]
//mod tests;

use super::{ErrorKind, Reader, Result, Writer};
use crate::ilint::{decode, encode};

/// The trait `IntDataReader` defines the
/// ability to extract signed and unsigned integer
/// values from a [`Reader`]. Those values
/// are always encoded in big endian format.
///
/// [`Reader`]: ../trait.Reader.html
pub trait IntDataReader<'a, T> {
    /// Reads an integer of type `T` from the Reader
    ///
    /// Returns:
    /// * `Ok(T)`: The value read.
    /// * `Err(ErrorKind)`: In case of error.
    fn read_int(&mut self) -> Result<T>;
}

/// This macro contains the base implementation of
/// `IntDataReader.read_int()`.
macro_rules! data_reader_read_be_bytes {
    ($self: ident, $type: ident) => {{
        let mut tmp: [u8; std::mem::size_of::<$type>()] = [0; std::mem::size_of::<$type>()];
        $self.read_all(&mut tmp)?;
        Ok($type::from_be_bytes(tmp))
    }};
}

impl<'a> IntDataReader<'a, u8> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<u8> {
        self.read()
    }
}

impl<'a> IntDataReader<'a, u16> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<u16> {
        data_reader_read_be_bytes!(self, u16)
    }
}

impl<'a> IntDataReader<'a, u32> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<u32> {
        data_reader_read_be_bytes!(self, u32)
    }
}

impl<'a> IntDataReader<'a, u64> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<u64> {
        data_reader_read_be_bytes!(self, u64)
    }
}

impl<'a> IntDataReader<'a, i8> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<i8> {
        match self.read() {
            Ok(v) => Ok(v as i8),
            Err(e) => Err(e),
        }
    }
}

impl<'a> IntDataReader<'a, i16> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<i16> {
        data_reader_read_be_bytes!(self, i16)
    }
}

impl<'a> IntDataReader<'a, i32> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<i32> {
        data_reader_read_be_bytes!(self, i32)
    }
}

impl<'a> IntDataReader<'a, i64> for dyn Reader<'a> {
    fn read_int(&mut self) -> Result<i64> {
        data_reader_read_be_bytes!(self, i64)
    }
}

/// The trait `ILIntDataReader` defines the
/// ability to extract an **ILInt** encoded value
/// from a [`Reader`].
///
/// [`Reader`]: ../trait.Reader.html
pub trait ILIntDataReader<'a> {
    /// Reads an **ILInt** value.
    ///
    /// Returns:
    /// * `Ok(T)`: The value read. It is always a **u64**.
    /// * `Err(ErrorKind)`: In case of error.    
    fn read_ilint(&mut self) -> Result<u64>;
}

impl<'a> ILIntDataReader<'a> for dyn Reader<'a> {
    fn read_ilint(&mut self) -> Result<u64> {
        match decode(self) {
            Ok(value) => Ok(value),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::CorruptedData),
        }
    }
}

/// The trait `FloatDataReader` defines the
/// ability to extract 32 and 64 bit floating
/// point values from a [`Reader`]. Those values
/// are always encoded in big endian IEEE 754-2008.
///
/// [`Reader`]: ../trait.Reader.html
pub trait FloatDataReader<'a, T> {
    /// Reads an float value.
    ///
    /// Returns:
    ///
    /// * `Ok(T)`: The value read.
    /// * `Err(ErrorKind)`: In case of error.    
    fn read_float(&mut self) -> Result<T>;
}

impl<'a> FloatDataReader<'a, f32> for dyn Reader<'a> {
    fn read_float(&mut self) -> Result<f32> {
        let tmp: u32 = self.read_int()?;
        Ok(f32::from_bits(tmp))
    }
}

impl<'a> FloatDataReader<'a, f64> for dyn Reader<'a> {
    fn read_float(&mut self) -> Result<f64> {
        let tmp: u64 = self.read_int()?;
        Ok(f64::from_bits(tmp))
    }
}

/// The trait `StringDataReader` defines the
/// ability to extract UTF-8 strings from a [`Reader`].
///
/// [`Reader`]: ../trait.Reader.html
pub trait StringDataReader<'a> {
    fn read_string(&mut self, size: usize) -> Result<String>;
}

impl<'a> StringDataReader<'a> for dyn Reader<'a> {
    fn read_string(&mut self, size: usize) -> Result<String> {
        let mut tmp: Vec<u8> = vec![0; size];
        self.read_all(tmp.as_mut_slice())?;
        match String::from_utf8(tmp) {
            Ok(s) => Ok(s),
            _ => Err(ErrorKind::CorruptedData),
        }
    }
}

/// The `DataReader` trait defines the combined ability
/// to read signed and unsigned integer, floating point values.
/// ILInt values and strings from a [`Reader`].
///
/// Since this trait requires the implementation of the same trait
/// for multiple types, each variant can be invoked by its full
/// qualified name such as ``DataReader::<u8>::read_int(r)``.
///
/// [`Reader`]: ../trait.Reader.html
pub trait DataReader<'a>:
    Reader<'a>
    + IntDataReader<'a, u8>
    + IntDataReader<'a, u16>
    + IntDataReader<'a, u32>
    + IntDataReader<'a, u64>
    + IntDataReader<'a, i8>
    + IntDataReader<'a, i16>
    + IntDataReader<'a, i32>
    + IntDataReader<'a, i64>
    + FloatDataReader<'a, f32>
    + FloatDataReader<'a, f64>
    + ILIntDataReader<'a>
    + StringDataReader<'a>
{
}

impl<'a> DataReader<'a> for dyn Reader<'a> {}

/// This macro defines the core implementation of
/// IntDataWriter.write_int().
macro_rules! data_writer_write_be_bytes {
    ($self: ident, $value: expr) => {
        $self.write_all(&$value.to_be_bytes())
    };
}

/// The trait `IntDataWriter` defines the
/// ability to write signed and unsigned integer
/// values to a [`Writer`]. Those values
/// are always encoded in big endian format.
///
/// [`Writer`]: ../trait.Writer.html
pub trait IntDataWriter<'a, T> {
    /// Writes the value.
    ///
    /// Parameters:
    /// * `v`: The value to write.
    ///
    /// Returns:
    ///
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: In case of error.
    fn write_int(&self, v: T, writer: &mut dyn Writer) -> Result<()>;
}

impl<'a> IntDataWriter<'a, u8> for () {
    fn write_int(&self, v: u8, writer: &mut dyn Writer) -> Result<()> {
        writer.write(v)
    }
}

impl<'a> IntDataWriter<'a, u16> for () {
    fn write_int(&self, v: u16, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

impl<'a> IntDataWriter<'a, u32> for () {
    fn write_int(&self, v: u32, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

impl<'a> IntDataWriter<'a, u64> for () {
    fn write_int(&self, v: u64, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

impl<'a> IntDataWriter<'a, i8> for () {
    fn write_int(&self, v: i8, writer: &mut dyn Writer) -> Result<()> {
        writer.write(v as u8)
    }
}

impl<'a> IntDataWriter<'a, i16> for () {
    fn write_int(&self, v: i16, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

impl<'a> IntDataWriter<'a, i32> for () {
    fn write_int(&self, v: i32, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

impl<'a> IntDataWriter<'a, i64> for () {
    fn write_int(&self, v: i64, writer: &mut dyn Writer) -> Result<()> {
        data_writer_write_be_bytes!(writer, v)
    }
}

/// The trait `ILIntDataWriter` defines the
/// ability to write ILInt encoded
/// values to a [`Writer`].
///
/// [`Writer`]: ../trait.Writer.html
pub trait ILIntDataWriter {
    /// Writes the value.
    ///
    /// Parameters:
    /// * `v`: The value to write.
    ///
    /// Returns:
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: In case of error.
    fn write_ilint(&mut self, v: u64, writer: &mut dyn Writer) -> Result<()>;
}

impl ILIntDataWriter for () {
    fn write_ilint(&mut self, v: u64, writer: &mut dyn Writer) -> Result<()> {
        match encode(v, writer) {
            Ok(()) => Ok(()),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::UnableToWriteData),
        }
    }
}

/// The trait `FloatDataWrite` defines the
/// ability to write 32 and 64 bit floating
/// point values to [`Writer`]. Those values
/// are always encoded in big endian IEEE 754-2008.
///
/// [`Writer`]: ../trait.Writer.html
pub trait FloatDataWriter<'a, T> {
    /// Writes the value.
    ///
    /// Parameters:
    /// * `v`: The value to write.
    ///
    /// Returns:
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: In case of error.
    fn write_float(&mut self, v: T) -> Result<()>;
}

impl<'a> FloatDataWriter<'a, f32> for dyn Writer {
    fn write_float(&mut self, v: f32) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

impl<'a> FloatDataWriter<'a, f64> for dyn Writer {
    fn write_float(&mut self, v: f64) -> Result<()> {
        data_writer_write_be_bytes!(self, v)
    }
}

/// The trait `StringDataWriter` defines the
/// ability to write UTF-8 strings to [`Writer`].
///
/// [`Writer`]: ../trait.Writer.html
pub trait StringDataWriter<'a> {
    /// Writes the value.
    ///
    /// Parameters:
    /// * `v`: The value to write.
    ///
    /// Returns:
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: In case of error.
    fn write_string(&mut self, value: &str) -> Result<()>;
}

impl<'a> StringDataWriter<'a> for dyn Writer {
    fn write_string(&mut self, value: &str) -> Result<()> {
        self.write_all(value.as_bytes())
    }
}
