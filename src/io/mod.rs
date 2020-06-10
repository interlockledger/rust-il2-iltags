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

mod data;
pub mod array;
pub mod vec;

pub use array::{ByteArrayReader, ByteArrayWriter};
pub use vec::{VecWriter};
pub use data::{DataReader,DataWriter};

pub enum ErrorKind {
    UnableToReadData,
    UnableToWriteData,
    CorruptedData,
    IOError(std::io::Error),
    BoxedError(Box<dyn std::error::Error>)
}

pub type Result<T> = std::result::Result<T, ErrorKind>;

/// The Reader trait is allows the extraction of bytes from a source.
pub trait Reader {
    
    /// Reads a single byte from the source.
    /// 
    /// Return
    ///
    fn read(&mut self) -> Result<u8>;

    fn read_all(&mut self, buff:&mut [u8]) -> Result<()> {
        for i in 0..buff.len() {
            buff[i] = self.read()?
        }
        Ok(())
    }
}

/// The Writer trait allows the insertion of bytes into a destination.
pub trait Writer {
    fn write(&mut self, value: u8) -> Result<()>;

    fn write_all(&mut self, buff: &[u8]) -> Result<()> {
        for b in buff {
            self.write(*b)?;
        }
        Ok(())
    }
}

pub struct ReadReader<'a> {
    source: &'a mut dyn std::io::Read,
}

impl<'a> ReadReader<'a> {

    pub fn new(src: &'a mut dyn std::io::Read) -> ReadReader<'a> {
        ReadReader{
            source: src
        }
    }
}

impl<'a> Reader for ReadReader<'a> {

    fn read(&mut self) -> Result<u8> {
        let mut buff: [u8; 1] = [0;1];
        match self.source.read(&mut buff) {
            Ok(1) => Ok(buff[0]),
            Ok(_) => Err(ErrorKind::UnableToReadData),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<()> {
        match self.source.read(buff) {
            Ok(n) if n == buff.len() => Ok(()),
            Ok(_) => Err(ErrorKind::UnableToReadData),
            Err(e) => Err(ErrorKind::IOError(e)),
        }
    }
}

pub struct WriteWriter<'a> {
    dest: &'a mut dyn std::io::Write,
}

impl<'a> WriteWriter<'a> {
    pub fn new(dst: &'a mut dyn std::io::Write) -> WriteWriter<'a> {
        WriteWriter{
            dest: dst
        }
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
}
