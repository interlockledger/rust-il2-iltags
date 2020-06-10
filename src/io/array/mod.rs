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
use super::{Reader, Writer,Result,ErrorKind};

#[cfg(test)]
mod tests;

pub struct ByteArrayReader<'a> {
    array: &'a[u8],
    offset: usize
}

impl<'a> ByteArrayReader<'a> {

    pub fn new(buff: &'a [u8]) -> ByteArrayReader<'a> {
        ByteArrayReader{
            array: buff,
            offset: 0
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        if offset < self.array.len() {
            self.offset = offset
        } else {
            self.offset = self.array.len()
        }
    }

    pub fn get_array(&self) -> &[u8] {
        self.array
    }
}

impl<'a> Reader for ByteArrayReader<'a> {
    
    fn read(&mut self) -> Result<u8> {
        if self.offset < self.array.len() {
            let r = self.array[self.offset];
            self.offset += 1;
            Ok(r)
        } else {
            Err(ErrorKind::UnableToReadData)
        }
    }

    fn read_all(&mut self, buff:&mut [u8]) -> Result<(), ()> {
        for i in 0..buff.len() {
            match self.read() {
                Ok(v) => buff[i] = v,
                Err(()) => return Err(())
            }
        }
        Ok(())
    }

    // TODO It is possible to implement a better read_all
}

pub struct ByteArrayWriter<'a> {
    array: &'a mut [u8],
    offset: usize,
}

impl<'a> ByteArrayWriter<'a> {
    pub fn new(buff: &'a mut [u8]) -> ByteArrayWriter {
        ByteArrayWriter {
            array: buff,
            offset: 0
        }
    }

    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        if offset < self.array.len() {
            self.offset = offset
        } else {
            self.offset = self.array.len()
        }
    }

    pub fn get_array(&mut self) -> &mut [u8] {
        self.array
    }
}

impl<'a> Writer for ByteArrayWriter<'a> {

    fn write(&mut self, value: u8) -> Result<()> {
        if self.offset < self.array.len() {
            self.array[self.offset] = value;
            self.offset += 1;
            Ok(())
        } else {
            Err(ErrorKind::UnableToWriteData)
        }
    }

    fn write_all(&mut self, buff: &[u8]) -> Result<(), ()> {
        for b in buff {
            if self.write(*b).is_err(){
                return Err(())
            }
        }
        Ok(())
    }

    // TODO Add a better implementation for write_all()
}
