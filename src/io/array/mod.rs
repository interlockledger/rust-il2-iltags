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
//! This module contains the implementation of [`Reader`] and [`Writer`] for
//! arrays, slices and vectors.
use super::{ErrorKind, Reader, Result, Writer};
use std::cmp::min;

#[cfg(test)]
mod tests;

//=============================================================================
// ByteArrayReader
//-----------------------------------------------------------------------------
/// [`ByteArrayReader`] implements a [`Reader`] that
/// can extract bytes from a borrowed slice of bytes.
pub struct ByteArrayReader<'a> {
    array: &'a [u8],
    offset: usize,
}

impl<'a> ByteArrayReader<'a> {
    pub fn new(buff: &'a [u8]) -> ByteArrayReader<'a> {
        ByteArrayReader {
            array: buff,
            offset: 0,
        }
    }

    /// Returns the current reading position.
    ///
    /// Returns:
    /// - The current offset. It is guaranteed to be at most
    /// the total size of the data.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Sets the current reading position.
    ///
    /// Arguments:
    /// - `offset`: The new position. It if is larger
    ///   than the total length, it will assume the
    ///   total length;
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = std::cmp::min(offset, self.array.len());
    }

    /// Returns a reference to the inner data as a
    /// slice.
    pub fn as_slice(&self) -> &[u8] {
        self.array
    }

    /// Returns the remanining number of bytes.
    pub fn available(&self) -> usize {
        self.array.len() - self.offset
    }

    /// Verifies if the specified number of bytes can be
    /// read from this struct.
    ///
    /// Returns:
    /// - `Result(())`: If it is possible to read the specified
    ///   number of bytes;
    /// - `Result(ErrorKind::UnableToReadData)`: If it is not
    ///   possible to read the specified number of bytes;
    pub fn can_read(&self, count: usize) -> Result<()> {
        if self.available() < count {
            Err(ErrorKind::UnableToReadData)
        } else {
            Ok(())
        }
    }
}

impl<'a> Reader for ByteArrayReader<'a> {
    fn read(&mut self) -> Result<u8> {
        self.can_read(1)?;
        let r = self.array[self.offset];
        self.offset += 1;
        Ok(r)
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        self.can_read(buff.len())?;
        buff.copy_from_slice(&self.array[self.offset..(self.offset + buff.len())]);
        self.offset += buff.len();
        Ok(())
    }

    fn skip(&mut self, count: usize) -> Result<()> {
        self.can_read(count)?;
        self.offset += count;
        Ok(())
    }
}

//=============================================================================
// VecReader
//-----------------------------------------------------------------------------
/// [`VecReader`] implements a [`Writer`] that uses a Vec<u8> a its backend.
///
/// It differs from [`ByteArrayReader`] by the fact that it copies the data
/// into a vector owned by it instead of borrowing the data from a byte array
/// slice.
pub struct VecReader {
    vector: Vec<u8>,
    offset: usize,
}

impl VecReader {
    /// Creates a new `VecReader` with the data copied
    /// from the specified slice.
    pub fn new(value: &[u8]) -> VecReader {
        let mut v: Vec<u8> = Vec::with_capacity(value.len());
        v.extend_from_slice(value);
        VecReader {
            vector: v,
            offset: 0,
        }
    }

    /// Returns the current reading position.
    ///
    /// Returns:
    /// - The current offset. It is guaranteed to be at most
    /// the total size of the data.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Sets the current reading position.
    ///
    /// Arguments:
    /// - `offset`: The new position. It if is larger
    ///   than the total length, it will assume the
    ///   total length;
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = std::cmp::min(offset, self.vector.len());
    }

    /// Returns the remanining number of bytes.    
    pub fn available(&self) -> usize {
        self.vector.len() - self.offset
    }

    /// Verifies if the specified number of bytes can be
    /// read from this struct.
    ///
    /// Returns:
    /// - `Result(())`: If it is possible to read the specified
    ///   number of bytes;
    /// - `Result(ErrorKind::UnableToReadData)`: If it is not
    ///   possible to read the specified number of bytes;
    pub fn can_read(&self, count: usize) -> Result<()> {
        if self.available() < count {
            Err(ErrorKind::UnableToReadData)
        } else {
            Ok(())
        }
    }

    /// Returns a reference to the inner data as a
    /// slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.vector
    }

    /// Returns aread-only reference to the inner vector.
    pub fn vec(&self) -> &Vec<u8> {
        &self.vector
    }
}

impl<'a> Reader for VecReader {
    fn read(&mut self) -> Result<u8> {
        self.can_read(1)?;
        let r = self.vector[self.offset];
        self.offset += 1;
        Ok(r)
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        self.can_read(buff.len())?;
        let new_offs = self.offset + buff.len();
        buff.copy_from_slice(&self.vector[self.offset..new_offs]);
        self.offset = new_offs;
        Ok(())
    }

    fn skip(&mut self, count: usize) -> Result<()> {
        self.can_read(count)?;
        self.offset += count;
        Ok(())
    }
}

//=============================================================================
// Base VecWriter methods
//-----------------------------------------------------------------------------
macro_rules! basevecwriter_basic_impl {
    () => {
        /// Returns the current writing position.
        ///
        /// Returns:
        /// - The current offset. It is guaranteed to be at most
        /// the total size of the data.
        pub fn get_offset(&self) -> usize {
            self.offset
        }

        /// Sets the current writing position.
        ///
        /// Arguments:
        /// - `offset`: The new position. It if is larger
        ///   than the total length, it will assume the
        ///   total length;
        pub fn set_offset(&mut self, offset: usize) {
            self.offset = std::cmp::min(offset, self.vector.len());
        }

        /// Returns true if this instance is locked for writing.
        pub fn is_read_only(&self) -> bool {
            self.read_only
        }

        /// Sets the read-only flag.
        ///
        /// Arguments:
        /// - `read_only`: The new value;
        pub fn set_read_only(&mut self, read_only: bool) {
            self.read_only = read_only;
        }

        /// Verifies if the it is possible to write into this
        /// `VecWriter`.
        ///
        /// Returns:
        /// - `Ok(())`: If it is possible to write;
        /// - `Err(ErrorKind::UnableToWriteData)`: If it is not possible
        /// to write;
        pub fn can_write(&self) -> Result<()> {
            if self.read_only {
                Err(ErrorKind::UnableToWriteData)
            } else {
                Ok(())
            }
        }

        /// Returns the current data as a slice.
        pub fn as_slice(&self) -> &[u8] {
            &self.vector.as_slice()
        }

        /// Returns aread-only reference to the inner vector.
        pub fn vec(&self) -> &Vec<u8> {
            &self.vector
        }
    };
}

macro_rules! basevecwriter_writer_impl {
    () => {
        fn write(&mut self, value: u8) -> Result<()> {
            self.can_write()?;
            if self.offset == self.vector.len() {
                self.vector.push(value);
            } else {
                self.vector[self.offset] = value;
            }
            self.offset += 1;
            Ok(())
        }

        fn write_all(&mut self, buff: &[u8]) -> Result<()> {
            self.can_write()?;
            let new_offset = self.offset + buff.len();
            if new_offset > self.vector.len() {
                self.vector.resize(new_offset, 0);
            }
            self.vector[self.offset..new_offset].copy_from_slice(buff);
            self.offset = new_offset;
            Ok(())
        }

        fn as_writer(&mut self) -> &mut dyn Writer {
            self
        }
    };
}

//=============================================================================
// VecWriter
//-----------------------------------------------------------------------------
/// [`VecWriter`] implements a [`Writer`] that uses a Vec<u8> a its backend.
pub struct VecWriter {
    vector: Vec<u8>,
    offset: usize,
    read_only: bool,
}

impl VecWriter {
    /// Creates a new empty instance of this struct. The new struct
    /// is set as writeable by default.
    pub fn new() -> VecWriter {
        VecWriter {
            vector: Vec::new(),
            offset: 0,
            read_only: false,
        }
    }

    /// Creates a new empty instance of this struct with an
    /// initial capacity set.
    ///
    /// Arguments:
    /// - `capacity`: The reserved capacity;
    pub fn with_capacity(capacity: usize) -> VecWriter {
        VecWriter {
            vector: Vec::with_capacity(capacity),
            offset: 0,
            read_only: false,
        }
    }

    basevecwriter_basic_impl!();
}

impl Writer for VecWriter {
    basevecwriter_writer_impl!();
}

impl Default for VecWriter {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// BorrowedVecWriter
//-----------------------------------------------------------------------------
/// [`BorrowedVecWriter`] implements a [`Writer`] that uses a borrowed Vec<u8>
/// as its backend.
///
/// The borrowed vector is used as is. This means that:
/// - Any existing data will be overwritten from the initial offset;
/// - The vector will be extended if required but will never shrink to
///   accomodate the amount of data written;
pub struct BorrowedVecWriter<'a> {
    vector: &'a mut Vec<u8>,
    initial_offset: usize,
    offset: usize,
    read_only: bool,
}

impl<'a> BorrowedVecWriter<'a> {
    /// Creates a new instance of this struct. The new struct
    /// is set as writeable by default.
    ///
    /// Arguments:
    /// - `vector`: The inner vector;
    pub fn new(vector: &'a mut Vec<u8>) -> Self {
        Self::with_offset(vector, 0)
    }

    /// Creates a new instance of this struct. The new struct
    /// is set as writeable by default.
    ///
    /// Arguments:
    /// - `vector`: The inner vector;
    /// - `offset`: The initial offset. If it is set to a value larger
    ///   than vector length, it will assume the vector length;
    pub fn with_offset(vector: &'a mut Vec<u8>, offset: usize) -> Self {
        let curr_offs = min(offset, vector.len());
        Self {
            vector,
            initial_offset: curr_offs,
            offset: curr_offs,
            read_only: false,
        }
    }

    basevecwriter_basic_impl!();

    /// Returns the number of bytes actually written.
    pub fn bytes_written(&self) -> usize {
        self.offset - self.initial_offset
    }
}

impl<'a> Writer for BorrowedVecWriter<'a> {
    basevecwriter_writer_impl!();
}

//=============================================================================
// ByteArrayWriter
//-----------------------------------------------------------------------------
/// [`ByteArrayWriter`] implements a [`Writer`] that uses a borrowed byte slice
/// as its backend.
///
/// New since 1.3.1.
pub struct ByteArrayWriter<'a> {
    array: &'a mut [u8],
    offset: usize,
}

impl<'a> ByteArrayWriter<'a> {
    /// Creates a new instance of this struct.
    ///
    /// Arguments:
    /// - `array`: The borrowed array;
    pub fn new(array: &'a mut [u8]) -> Self {
        Self { array, offset: 0 }
    }

    /// Returns the current writing position.
    ///
    /// Returns:
    /// - The current offset. It is guaranteed to be at most
    /// the total size of the data.
    pub fn get_offset(&self) -> usize {
        self.offset
    }

    /// Sets the current writing position.
    ///
    /// Arguments:
    /// - `offset`: The new position. It if is larger
    ///   than the total length, it will assume the
    ///   total length;
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = std::cmp::min(offset, self.array.len());
    }

    /// Return the number of bytes available.
    #[inline]
    pub fn available(&self) -> usize {
        self.array.len() - self.offset
    }

    fn can_write(&self, len: usize) -> Result<()> {
        if self.available() >= len {
            Ok(())
        } else {
            Err(ErrorKind::UnableToWriteData)
        }
    }
}

impl<'a> Writer for ByteArrayWriter<'a> {
    fn write(&mut self, value: u8) -> Result<()> {
        self.can_write(1)?;
        self.array[self.offset] = value;
        self.offset += 1;
        Ok(())
    }

    fn write_all(&mut self, buff: &[u8]) -> Result<()> {
        self.can_write(buff.len())?;
        self.array[self.offset..self.offset + buff.len()].copy_from_slice(buff);
        self.offset += buff.len();
        Ok(())
    }

    fn as_writer(&mut self) -> &mut dyn Writer {
        self
    }
}
