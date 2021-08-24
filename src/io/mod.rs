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
//! This module implements the I/O abstraction used by this library. It allows
//! the usage of multiple data sources and/or repositories to perform the
//! operations.
pub mod array;
pub mod data;

#[cfg(test)]
mod tests;

/// Types of erros generated by this module.
#[derive(Debug)]
pub enum ErrorKind {
    /// Unable to read data.
    UnableToReadData,
    /// Unable to write data.
    UnableToWriteData,
    /// The data is corrupted.
    CorruptedData,
    /// Wrapper to standard `std::io::Error`.
    IOError(std::io::Error),
    /// Wrapper to a boxed error `std::error::Error`.
    BoxedError(Box<dyn std::error::Error>),
}

/// A specialized [`std::result::Result`] generated by functions and methods from this package.
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// The [`Reader`] trait is allows the extraction of bytes from a source.
///
/// It differs from most IO library as it defines all operations as
/// all-or-nothing operations. No partial reads are allowed.
///
/// Implementations of this trait are not required to be thread-safe.
pub trait Reader {
    /// Reads a single byte from the source.
    ///
    /// Returns:
    /// * `Ok(v)`: The value read;
    /// * `Err(ErrorKind)`: In case of error;
    fn read(&mut self) -> Result<u8>;

    /// Reads the specified number of bytes from the source.
    ///
    /// The default implementation just calls read() repeatedly,
    /// so each implementation is advised to provide a better
    /// implementation of this method if possible.
    ///
    /// Arguments:
    /// * `buff`: The output buffer;
    ///
    /// Returns:
    /// * `Ok(())`: On success;
    /// * `Err(ErrorKind)`: In case of error;
    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        for b in buff {
            *b = self.read()?;
        }
        Ok(())
    }

    /// Skips some bytes from the source.
    ///
    /// The default implementation just calls read_all() repeatedly
    /// using 512 byte chunks. So it is recommended provide better
    /// implementations whenever possible.
    ///
    /// Arguments:
    /// * `count`: Number of byte to skip;
    ///
    /// Returns:
    /// * `Ok(())`: On success;
    /// * `Err(ErrorKind)`: In case of error;
    fn skip(&mut self, count: usize) -> Result<()> {
        let mut buff: [u8; 512] = [0; 512];
        let mut r = count;
        while r > 0 {
            let chunk = std::cmp::min(r, buff.len());
            self.read_all(&mut buff[0..chunk])?;
            r -= chunk;
        }
        Ok(())
    }

    /// Skips some bytes from the source using a u64 as its size. On
    /// 64-bit systems, this is equivalent to [`Self::skip()`] as
    /// usize is a 64-bit value.
    ///
    /// This method will make a difference in 32-bit systems where
    /// usize is actually a 32-bit value.
    ///
    /// This implementation calls [`Self::skip()`] as may times
    /// as necessary to skip the required number of bytes.
    ///
    /// Arguments:
    /// * `count`: Number of byte to skip;
    ///
    /// Returns:
    /// * `Ok(())`: On success;
    /// * `Err(ErrorKind)`: In case of error;
    ///
    /// New since 1.4.0.    
    fn skip_u64(&mut self, count: u64) -> Result<()> {
        let mut remaining = count;
        while remaining > 0 {
            let skip = std::cmp::min(remaining, usize::MAX as u64);
            self.skip(skip as usize)?;
            remaining -= skip;
        }
        Ok(())
    }
}

/// The [`Writer`] trait allows the addition of bytes into the destination.
///
/// It differs from most IO library as it defines all operations as
/// all-or-nothing operations. No partial writes are allowed.
///
/// Implementations of this trait are not required to be thread-safe.
pub trait Writer {
    /// Writes a single byte.
    ///
    /// Arguments:
    /// * `value`: The value to be written;
    ///
    /// Returns:
    /// * `Ok(())`: On success;
    /// * `Err(ErrorKind)`: In case of error;
    fn write(&mut self, value: u8) -> Result<()>;

    /// Writes a byte slice. As this default implementation
    /// calls `write()` multiple times, so it is strongly recommended
    /// that each implementation provides a more efficient version for
    /// this method if possible.
    ///
    /// Arguments:
    /// * `buff`: The value to be written;
    ///
    /// Returns:
    /// * `Ok(())`: On success;
    /// * `Err(ErrorKind)`: In case of error;
    fn write_all(&mut self, buff: &[u8]) -> Result<()> {
        for b in buff {
            self.write(*b)?;
        }
        Ok(())
    }

    fn as_writer(&mut self) -> &mut dyn Writer;
}

/// The `LimitedReader` implements a [`Reader`] that wraps another
/// [`Reader`] but defines a limited to the amount of bytes that can be
/// extracted from it.
///
/// This wrapper was designed to ease the implementation of the deserialization
/// of the tags.
///
/// It is important to notice that [`LimitedReader`] will test the
/// limits prior to the attempt to read the data, thus failed
/// attempts will not consume data from the inner reader.
pub struct LimitedReader<'a> {
    source: &'a mut dyn Reader,
    available: usize,
}

impl<'a> LimitedReader<'a> {
    /// Creates a new instance of this struct.
    ///
    /// Parameters:
    /// * `src`: A mutable reference to the source Reader.
    /// * `available`: Number of bytes available for reading.
    pub fn new(src: &mut dyn Reader, available: usize) -> LimitedReader {
        LimitedReader {
            source: src,
            available,
        }
    }

    /// Verifies if it is possible to extract a given number of bytes
    /// from the source.
    ///
    /// Parameters:
    /// * `size`: The number of bytes to read.
    ///
    /// Returns:
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: If the specified number of bytes is not available.
    pub fn can_read(&self, size: usize) -> Result<()> {
        if size > self.available {
            Err(ErrorKind::UnableToReadData)
        } else {
            Ok(())
        }
    }

    /// Returns the number of available bytes.
    ///
    /// Returns:
    /// * The number of available bytes.
    pub fn available(&self) -> usize {
        self.available
    }

    /// Skips the required number of bytes required to achive the end
    /// of the specified limit.
    ///
    /// Returns:
    /// * `Ok(())`: On success.
    /// * `Err(ErrorKind)`: If the specified number of bytes is not available.
    pub fn goto_end(&mut self) -> Result<()> {
        if self.available > 0 {
            let ret = self.source.skip(self.available);
            if ret.is_ok() {
                self.available = 0;
            }
            ret
        } else {
            Ok(())
        }
    }

    /// Verifies if the this reader is empty.
    ///
    /// Returns:
    /// - true: if it is empty;
    /// - false: if it is not empty;
    pub fn empty(&self) -> bool {
        self.available == 0
    }
}

impl<'a> Reader for LimitedReader<'a> {
    fn read(&mut self) -> Result<u8> {
        self.can_read(1)?;
        let ret = self.source.read();
        if ret.is_ok() {
            self.available -= 1;
        }
        ret
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        self.can_read(buff.len())?;
        let ret = self.source.read_all(buff);
        if ret.is_ok() {
            self.available -= buff.len();
        }
        ret
    }
}

/// This struct implements a [`Reader`] that uses a
/// [`std::io::Read`] as the source of bytes.
pub struct ReadReader<'a, T: std::io::Read> {
    source: &'a mut T,
}

impl<'a, T: std::io::Read> ReadReader<'a, T> {
    /// Creates a new instance of ReadReader.
    ///
    /// Parameters:
    /// * `src`: The source of bytes.
    pub fn new(src: &'a mut T) -> ReadReader<'a, T> {
        ReadReader { source: src }
    }
}

impl<'a, T: std::io::Read> Reader for ReadReader<'a, T> {
    fn read(&mut self) -> Result<u8> {
        let mut buff: [u8; 1] = [0; 1];
        match self.source.read_exact(&mut buff) {
            Ok(()) => Ok(buff[0]),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        match self.source.read_exact(buff) {
            Ok(()) => Ok(()),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }
}

/// This struct implements a [`Writer`] that uses a
/// [`std::io::Write`] as the destination of bytes.
pub struct WriteWriter<'a> {
    dest: &'a mut dyn std::io::Write,
}

impl<'a> WriteWriter<'a> {
    /// Creates a new instance of `WriteWriter`.
    ///
    /// Parameters:
    /// * `dst`: The destination for the bytes.
    pub fn new(dst: &'a mut dyn std::io::Write) -> WriteWriter<'a> {
        WriteWriter { dest: dst }
    }
}

impl<'a> Writer for WriteWriter<'a> {
    fn write(&mut self, value: u8) -> Result<()> {
        let tmp: [u8; 1] = [value];
        match self.dest.write(&tmp) {
            Ok(1) => Ok(()),
            Ok(_) => Err(ErrorKind::UnableToWriteData),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn write_all(&mut self, buff: &[u8]) -> Result<()> {
        match self.dest.write(&buff) {
            Ok(n) if n == buff.len() => Ok(()),
            Ok(_) => Err(ErrorKind::UnableToWriteData),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn as_writer(&mut self) -> &mut dyn Writer {
        self
    }
}

//=============================================================================
// Reader for std::io::Read + std::io::Seek
//-----------------------------------------------------------------------------
// New since 1.4.0.
impl<T> Reader for T
where
    T: std::io::Read + std::io::Seek,
{
    fn read(&mut self) -> Result<u8> {
        let mut buff: [u8; 1] = [0];
        self.read_all(&mut buff)?;
        Ok(buff[0])
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        match std::io::Read::read_exact(self, buff) {
            Ok(()) => Ok(()),
            Err(_) => Err(ErrorKind::UnableToReadData),
        }
    }

    fn skip(&mut self, count: usize) -> Result<()> {
        self.skip_u64(count as u64)
    }

    fn skip_u64(&mut self, count: u64) -> Result<()> {
        let new_pos = match self.seek(std::io::SeekFrom::Current(count as i64)) {
            Ok(p) => p,
            Err(_) => return Err(ErrorKind::UnableToReadData),
        };
        let end_pos = match self.seek(std::io::SeekFrom::End(0)) {
            Ok(p) => p,
            Err(_) => return Err(ErrorKind::UnableToReadData),
        };
        if new_pos <= end_pos && self.seek(std::io::SeekFrom::Start(new_pos)).is_ok() {
            Ok(())
        } else {
            Err(ErrorKind::UnableToReadData)
        }
    }
}

//=============================================================================
// Writer for std::io::Write + std::io::Seek
//-----------------------------------------------------------------------------
// New since 1.4.0.
impl<T> Writer for T
where
    T: ::std::io::Write + ::std::io::Seek,
{
    fn write(&mut self, value: u8) -> Result<()> {
        let mut buff: [u8; 1] = [value];
        Writer::write_all(self, &mut buff)
    }

    fn write_all(&mut self, buff: &[u8]) -> Result<()> {
        match std::io::Write::write_all(self, buff) {
            Ok(()) => Ok(()),
            Err(_) => Err(ErrorKind::UnableToWriteData),
        }
    }

    fn as_writer(&mut self) -> &mut dyn Writer {
        self
    }
}
