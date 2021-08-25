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
//=============================================================================
//! This module implements the integration between this library and Rust's
//! [`std::io`] components.
#[cfg(test)]
mod tests;

use super::*;

//=============================================================================
// ErrorKind conversion.
//-----------------------------------------------------------------------------
impl std::convert::From<std::io::Error> for ErrorKind {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::UnexpectedEof => Self::EndOfData,
            _ => ErrorKind::BoxedError(Box::new(e)),
        }
    }
}

//=============================================================================
// SizedSeek
//-----------------------------------------------------------------------------
/// This trait adds the ability to check how may bytes are available.
pub trait SizedSeek {
    /// Length of the data.
    fn len(&mut self) -> Result<u64>;

    /// Returns true if its len() is 0.
    fn is_empty(&mut self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// The current offset
    fn offset(&mut self) -> Result<u64>;
    /// Returns the number of bytes available for reading.
    fn available(&mut self) -> Result<u64> {
        Ok(self.len()? - self.offset()?)
    }
}

impl<T: std::io::Seek> SizedSeek for T {
    fn len(&mut self) -> Result<u64> {
        // Replace this implementation with Seek::stream_len() when
        // it became available.
        let curr = self.seek(std::io::SeekFrom::Current(0))?;
        let end = self.seek(std::io::SeekFrom::End(0))?;
        self.seek(std::io::SeekFrom::Start(curr))?;
        Ok(end)
    }

    fn offset(&mut self) -> Result<u64> {
        Ok(self.stream_position()?)
    }
}

//=============================================================================
// ReadReader
//-----------------------------------------------------------------------------
/// This struct implements a [`Reader`] that uses a [`std::io::Read`] as the
/// source of bytes.
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

//=============================================================================
// WriteWriter
//-----------------------------------------------------------------------------
/// This struct implements a [`Writer`] that uses a [`std::io::Write`] as the
/// destination of bytes.
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
        if count == 0 {
            Ok(())
        } else {
            let curr_pos = self.seek(std::io::SeekFrom::Current(0))?;
            let end_pos = self.seek(std::io::SeekFrom::End(0))?;
            if curr_pos == end_pos {
                Err(ErrorKind::EndOfData)
            } else {
                let new_pos = curr_pos + count;
                if new_pos <= end_pos {
                    self.seek(std::io::SeekFrom::Start(new_pos))?;
                    Ok(())
                } else {
                    Err(ErrorKind::UnableToReadData)
                }
            }
        }
    }
}

//=============================================================================
// Writer for std::io::Write + std::io::Seek
//-----------------------------------------------------------------------------
// New since 1.4.0.
impl<T> Writer for T
where
    T: std::io::Write + std::io::Seek,
{
    fn write(&mut self, value: u8) -> Result<()> {
        let buff: [u8; 1] = [value];
        Writer::write_all(self, &buff)
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
