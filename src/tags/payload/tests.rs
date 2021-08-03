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
use super::*;
use crate::io::array::{VecReader, VecWriter};
use crate::io::data::{ValueReader, ValueWriter};
use crate::tags::tests::UntouchbleTagFactory;
use crate::tags::{ErrorKind, Result};

//=============================================================================
// TestTagPayload
//-----------------------------------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
struct TestTagPayload {
    a: u16,
    b: u32,
}

impl TestTagPayload {
    pub fn new() -> Self {
        Self { a: 0, b: 0 }
    }

    pub fn a(&self) -> u16 {
        self.a
    }

    pub fn set_a(&mut self, a: u16) {
        self.a = a
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn set_b(&mut self, b: u32) {
        self.b = b
    }
}

impl Default for TestTagPayload {
    fn default() -> Self {
        Self::new()
    }
}

impl ILTagPayload for TestTagPayload {
    fn serialized_size(&self) -> usize {
        std::mem::size_of::<u16>() + std::mem::size_of::<u32>()
    }

    fn serialize(&self, writer: &mut dyn Writer) -> Result<()> {
        match writer.write_value(self.a) {
            Ok(()) => (),
            Err(x) => return Err(ErrorKind::IOError(x)),
        }
        match writer.write_value(self.b) {
            Ok(()) => Ok(()),
            Err(x) => Err(ErrorKind::IOError(x)),
        }
    }

    fn deserialize(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        if value_size != self.serialized_size() {
            return Err(ErrorKind::CorruptedData);
        }
        self.a = match reader.read_value() {
            Ok(v) => v,
            Err(x) => return Err(ErrorKind::IOError(x)),
        };
        self.b = match reader.read_value() {
            Ok(v) => v,
            Err(x) => return Err(ErrorKind::IOError(x)),
        };
        Ok(())
    }
}

#[test]
fn test_testtagpayload_default() {
    let p = TestTagPayload::default();
    assert_eq!(p.a(), 0);
    assert_eq!(p.b(), 0);
}

#[test]
fn test_testtagpayload_iltagpayload_serialized_size() {
    let p = TestTagPayload::default();
    assert_eq!(p.serialized_size(), 2 + 4);
}

static TEST_TAG_PAYLOAD_BYTES: [u8; 6] = [0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45];
const TEST_TAG_PAYLOAD_A: u16 = 0xABCD;
const TEST_TAG_PAYLOAD_B: u32 = 0xEF012345;

#[test]
fn test_testtagpayload_iltagpayload_serialize() -> Result<()> {
    let mut p = TestTagPayload::default();
    p.set_a(TEST_TAG_PAYLOAD_A);
    p.set_b(TEST_TAG_PAYLOAD_B);

    let mut writer = VecWriter::new();
    p.serialize(&mut writer)?;
    assert_eq!(writer.as_slice(), &TEST_TAG_PAYLOAD_BYTES);
    Ok(())
}

#[test]
fn test_testtagpayload_iltagpayload_deserialize() -> Result<()> {
    let factory = UntouchbleTagFactory {};
    let mut p = TestTagPayload::default();

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);
    p.deserialize(&factory, TEST_TAG_PAYLOAD_BYTES.len(), &mut reader)?;
    assert_eq!(p.a(), TEST_TAG_PAYLOAD_A);
    assert_eq!(p.b(), TEST_TAG_PAYLOAD_B);

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);
    match p.deserialize(&factory, TEST_TAG_PAYLOAD_BYTES.len() - 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error expected."),
    }

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);
    match p.deserialize(&factory, TEST_TAG_PAYLOAD_BYTES.len() + 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error expected."),
    }

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES[1..]);
    match p.deserialize(&factory, TEST_TAG_PAYLOAD_BYTES.len(), &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error expected."),
    }
    Ok(())
}

//=============================================================================
// ILGenericPayloadTag
//-----------------------------------------------------------------------------
type TestTagPayloadTag = ILGenericPayloadTag<TestTagPayload>;

#[test]
fn test_testtagpayloadtag_impl() {
    let t = TestTagPayloadTag::new(12345);
    let default_payload = TestTagPayload::default();

    assert_eq!(t.id(), 12345);
    assert_eq!(t.payload(), &default_payload);

    let mut t = TestTagPayloadTag::new(12345);
    t.mut_payload().set_a(1);
    t.mut_payload().set_b(2);
    assert_eq!(t.id(), 12345);
    assert_eq!(t.payload().a(), 1);
    assert_eq!(t.payload().b(), 2);
}

#[test]
fn test_testtagpayloadtag_iltag_value_size() {
    let t = TestTagPayloadTag::default_with_id(1234);
    assert_eq!(t.value_size(), t.payload().serialized_size() as u64);
}

#[test]
fn test_testtagpayloadtag_iltag_serialize_value() -> Result<()> {
    let mut t = TestTagPayloadTag::default_with_id(1234);

    t.mut_payload().set_a(TEST_TAG_PAYLOAD_A);
    t.mut_payload().set_b(TEST_TAG_PAYLOAD_B);

    let mut writer = VecWriter::new();
    t.serialize_value(&mut writer)?;
    assert_eq!(writer.as_slice(), &TEST_TAG_PAYLOAD_BYTES);
    Ok(())
}

#[test]
fn test_testtagpayloadtag_iltag_deserialize_value() -> Result<()> {
    let factory = UntouchbleTagFactory {};

    let mut t = TestTagPayloadTag::default_with_id(1234);
    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);

    t.deserialize_value(&factory, TEST_TAG_PAYLOAD_BYTES.len(), &mut reader)?;
    assert_eq!(t.id(), 1234);
    assert_eq!(t.mut_payload().a(), TEST_TAG_PAYLOAD_A);
    assert_eq!(t.mut_payload().b(), TEST_TAG_PAYLOAD_B);

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);
    match t.deserialize_value(&factory, TEST_TAG_PAYLOAD_BYTES.len() - 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error expected!"),
    }

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES);
    match t.deserialize_value(&factory, TEST_TAG_PAYLOAD_BYTES.len() + 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error expected!"),
    }

    let mut reader = VecReader::new(&TEST_TAG_PAYLOAD_BYTES[1..]);
    match t.deserialize_value(&factory, TEST_TAG_PAYLOAD_BYTES.len(), &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error expected!"),
    }
    Ok(())
}

#[test]
fn test_testtagpayloadtag_defaultwithid() {
    let t = TestTagPayloadTag::default_with_id(1234);

    assert_eq!(t.id(), 1234);
}
