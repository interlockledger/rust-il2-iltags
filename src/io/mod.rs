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

pub mod array;
pub mod vec;

use super::ilint::{encode, decode};

/// The Reader trait is allows the extraction of bytes from a source.
pub trait Reader {
    
    /// Reads a single byte from the source.
    /// 
    /// Return
    ///
    fn read(&mut self) -> Result<u8, ()>;

    fn read_all(&mut self, buff:&mut [u8]) -> Result<(), ()> {
        for i in 0..buff.len() {
            match self.read() {
                Ok(v) => buff[i] = v,
                Err(()) => return Err(())
            }
        }
        Ok(())
    }
}

/// The Writer trait allows the insertion of bytes into a destination.
pub trait Writer {
    fn write(&mut self, value: u8) -> Result<(), ()>;

    fn write_all(&mut self, buff: &[u8]) -> Result<(), ()> {
        for b in buff {
            if self.write(*b).is_err(){
                return Err(())
            }
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

    fn read(&mut self) -> Result<u8, ()> {
        let mut buff: [u8; 1] = [0;1];
        match self.source.read(&mut buff) {
            Ok(1) => Ok(buff[0]),
            _ => Err(())
        }
    }

    fn read_all(&mut self, buff: &mut [u8]) -> Result<(), ()> {
        match self.source.read(buff) {
            Ok(n) => {
                if n == buff.len() {
                    Ok(())
                } else {
                    Err(())
                }
            },
            _ => Err(())
        }
    }
}

macro_rules! data_reader_read_be_bytes {
    ($self: ident, $type: ident, $tmp: expr) => {
        match $self.read_all(&mut $tmp) {
            Ok(()) => Ok($type::from_be_bytes($tmp)),
            _ => Err(())
        }
    };
}

/// The DataReader is a Reader that implements some
/// data functions.
pub trait DataReader: Reader {

    fn as_reader(&mut self) -> &mut dyn Reader;

    fn read_u16(&mut self) -> Result<u16, ()> {
        let mut tmp: [u8; 2] = [0; 2];
        data_reader_read_be_bytes!(self, u16, tmp)
    }

    fn read_u32(&mut self) -> Result<u32, ()> {
        let mut tmp: [u8; 4] = [0; 4];
        data_reader_read_be_bytes!(self, u32, tmp)
    }

    fn read_u64(&mut self) -> Result<u64, ()> {
        let mut tmp: [u8; 8] = [0; 8];
        data_reader_read_be_bytes!(self, u64, tmp)
    }

    fn read_ilint(&mut self) -> Result<u64, ()> {
        match decode(self.as_reader()) {
            Ok(value) => Ok(value),
            _ => Err(())
        }
    }

    fn read_f32(&mut self) -> Result<f32, ()> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    fn read_f64(&mut self) -> Result<f64, ()> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    fn read_i16(&mut self) -> Result<i16, ()> {
        Ok(self.read_u16()? as i16)
    }

    fn read_i32(&mut self) -> Result<i32, ()> {
        Ok(self.read_u32()? as i32)
    }

    fn read_i64(&mut self) -> Result<i64, ()> {
        Ok(self.read_u64()? as i64)
    }

    fn read_string(&mut self, size: usize) -> Result<String, ()> {
        let mut tmp: Vec<u8> = Vec::with_capacity(size);
        tmp.resize(size, 0);
        self.read_all(tmp.as_mut_slice())?;
        match String::from_utf8(tmp) {
            Ok(s) => Ok(s),
            _ => Err(())
        }
    }
}

macro_rules! data_writer_write_be_bytes {
    ($self: ident, $value: expr) => {
        $self.write_all(&$value.to_be_bytes())
    };
}

pub trait DataWriter: Writer {
    
    fn as_writer(&mut self) -> &mut dyn Writer;

    fn write_u16(&mut self, value: u16) -> Result<(), ()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_u32(&mut self, value: u32) -> Result<(), ()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_u64(&mut self, value: u64) -> Result<(), ()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_ilint(&mut self, value: u64) -> Result<(), ()> {
        encode(value, self.as_writer())
    }

    fn write_f32(&mut self, value: f32) -> Result<(), ()> {
        data_writer_write_be_bytes!(self, value)
    }
    
    fn write_f64(&mut self, value: f64) -> Result<(), ()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_i16(&mut self, value: i16) -> Result<(), ()> {
        self.write_u16(value as u16)
    }

    fn write_i32(&mut self, value: i32) -> Result<(), ()> {
        self.write_u32(value as u32)
    }

    fn write_i64(&mut self, value: i64) -> Result<(), ()> {
        self.write_u64(value as u64)
    }

    fn write_string(&mut self, value: &String) -> Result<(), ()> {
        self.write_all(value.as_bytes())
    }    
}
