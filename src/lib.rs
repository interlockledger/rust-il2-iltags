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
pub mod ilint;
pub mod io;
pub mod standard;

#[cfg(test)]
mod tests;

use io::{DataReader,DataWriter};

pub const IMPLICITY_ID_MAX: u64 = 0x0F;
pub const RESERVED_ID_MAX: u64 = 0x1F;

pub fn is_implicity(id: u64) -> bool {
    id < IMPLICITY_ID_MAX
}

pub fn is_reserved(id: u64) -> bool {
    id < RESERVED_ID_MAX
}

pub trait ILTag {
    /// Returns the ID of the tag.
    fn id(&self) -> u64;

    /// Verifies if this tag is implicity
    fn is_implicity(&self) -> bool {
        is_implicity(self.id())
    }

    /// Verifies if this tag is reserved.
    fn is_reserved(&self) -> bool {
        is_reserved(self.id())
    }

    /// Retuns the size of the payload in bytes.
    fn payload_size(&self) -> usize;

    /// Returns the total size of the tag in bytes.
    fn size(&self) -> usize {
        let mut size: usize = ilint::encoded_size(self.id());
        if  !self.is_implicity() {
            size += ilint::encoded_size(self.payload_size() as u64);
        }
        size + self.payload_size()
    }

    fn serialize_value(&self, writer: &mut dyn DataWriter) -> Result<(),()>;

    fn serialize(&self, writer: &mut dyn DataWriter) -> Result<(),()> {
        writer.write_ilint(self.id())?;
        if  !self.is_implicity() {
            writer.write_ilint(self.payload_size() as u64)?;
        }
        self.serialize_value(writer)
    }

    fn deserialize_value(&mut self, factory: &dyn ILTagFactory, reader: &mut dyn DataReader) -> Result<(),()>;
}

pub trait ILTagExplicity: ILTag {

    fn deserialize_value(&mut self, factory: &dyn ILTagFactory, reader: &mut dyn DataReader) -> Result<(),()> {
        if self.is_implicity() {
            return Err(())
        }
        let size = reader.read_ilint()?;
        self.deserialize_value_core(factory, size as usize, reader)
    }

    fn deserialize_value_core(&mut self, factory: &dyn ILTagFactory, payload_size: usize, reader: &mut dyn DataReader) -> Result<(),()>;
}

pub struct RawTag {
    id: u64,
    payload: Box<Vec<u8>>,
}

impl RawTag {
    pub fn new(id: u64) -> RawTag {
        RawTag{
            id: id,
            payload: Box::new(Vec::new()),
        }
    }
}

///
/// 
impl ILTag for RawTag {
    
    fn id(&self) -> u64 {
        self.id
    }
    
    fn payload_size(&self) -> usize {
        self.payload.len()
    }

    fn serialize_value(&self, writer: &mut dyn DataWriter) -> Result<(),()> {
        writer.write_all(self.payload.as_slice())
    }

    fn deserialize_value(&mut self, _factory: &dyn ILTagFactory, reader: &mut dyn DataReader) -> Result<(),()> {
        if self.is_implicity() {
            return Err(())
        }
        reader.read_all(self.payload.as_mut_slice())
    }
}

pub trait ILTagFactory {

    fn as_ref(&self) ->  &dyn ILTagFactory;

    fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>>;

    fn deserialize(&self, reader: &mut dyn DataReader) -> Result<Box<dyn ILTag>, ()> {
        
        let mut tag = match self.create_tag(reader.read_ilint()?) {
            Some(v) => v,
            _ => return Err(())
        };
        match tag.deserialize_value(self.as_ref(), reader) {
            Ok(()) => Ok(tag),
            _ => Err(())
        }
    }
}
