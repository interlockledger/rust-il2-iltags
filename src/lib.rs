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

use io::{Reader,Writer};

pub const IMPLICITY_ID_MAX: u64 = 0x0F;
pub const RESERVED_ID_MAX: u64 = 0x1F;

pub trait ILTag {
    fn get_id(&self) -> u64;
    
    fn is_implicity(&self) -> bool {
        self.get_id() < IMPLICITY_ID_MAX
    }

    fn is_reserved(&self) -> bool {
        self.get_id() < RESERVED_ID_MAX
    }

    fn payload_size(&self) -> usize;

    fn size(&self) -> usize;

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<(),()>;

    fn serialize(&self, writer: &mut dyn Writer) -> Result<(),()>;

    fn deserialize_value(&mut self, factory: &dyn ILTagFactory, reader: &mut dyn Reader) -> Result<(),()>;
}

pub trait ILTagFactory {

    fn create_tag(&self, tag_id: u64) -> Box<dyn ILTag>;
}
