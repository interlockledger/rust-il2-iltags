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
use crate::io::array::{ByteArrayReader, VecWriter};
use crate::io::Writer;

#[test]
fn test_constants() {
    assert_eq!(IMPLICIT_ID_MAX, 15);
    assert_eq!(RESERVED_ID_MAX, 31);
}

#[test]
fn test_is_implicit_tag() {
    for i in 0..16 {
        assert!(is_implicit_tag(i as u64))
    }
    assert!(!is_implicit_tag(16))
}

#[test]
fn test_is_reserved_tag() {
    for i in 0..32 {
        assert!(is_reserved_tag(i as u64))
    }
    assert!(!is_reserved_tag(32))
}

#[test]
fn test_tag_size_to_usize() {
    assert_eq!(MAX_TAG_SIZE, 512 * 1024 * 1024);

    let s = match tag_size_to_usize(0) {
        Ok(v) => v,
        _ => panic!(),
    };
    assert_eq!(s, 0);

    let s = match tag_size_to_usize(MAX_TAG_SIZE) {
        Ok(v) => v,
        _ => panic!(),
    };
    assert_eq!(s as u64, MAX_TAG_SIZE);

    match tag_size_to_usize(MAX_TAG_SIZE + 1) {
        Err(ErrorKind::TagTooLarge) => (),
        _ => panic!(),
    };
}

const ILINT_SAMPLE: u64 = 0xcb3a_208d_5c13_69e4;
const ILINT_SAMPLE_BIN: [u8; 9] = [0xFF, 0xCB, 0x3A, 0x20, 0x8D, 0x5C, 0x13, 0x68, 0xEC];

#[test]
fn test_serialize_ilint() {
    let mut writer = VecWriter::new();

    match serialize_ilint(ILINT_SAMPLE, &mut writer) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(writer.as_slice(), &ILINT_SAMPLE_BIN);

    writer.set_read_only(true);
    match serialize_ilint(ILINT_SAMPLE, &mut writer) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_deserialize_ilint() {
    let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN);
    let v = match deserialize_ilint(&mut reader) {
        Ok(v) => v,
        _ => panic!(),
    };
    assert_eq!(v, ILINT_SAMPLE);

    let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN[..ILINT_SAMPLE_BIN.len() - 1]);
    match deserialize_ilint(&mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_serialize_bytes() {
    let mut writer = VecWriter::new();
    match serialize_bytes(&ILINT_SAMPLE_BIN, &mut writer) {
        Ok(v) => v,
        _ => panic!(),
    };
    assert_eq!(writer.as_slice(), &ILINT_SAMPLE_BIN);

    writer.set_read_only(true);
    match serialize_bytes(&ILINT_SAMPLE_BIN, &mut writer) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_deserialize_bytes() {
    for size in 0..ILINT_SAMPLE_BIN.len() {
        let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN);
        let ret = match deserialize_bytes(size, &mut reader) {
            Ok(v) => v,
            _ => panic!(),
        };
        assert_eq!(ret.as_slice(), &ILINT_SAMPLE_BIN[0..size]);
    }

    let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN);
    match deserialize_bytes(ILINT_SAMPLE_BIN.len() + 1, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_deserialize_bytes_into_vec() {
    for size in 0..ILINT_SAMPLE_BIN.len() {
        let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN);
        let mut vec: Vec<u8> = Vec::new();
        vec.resize(1234, 1); //
        match deserialize_bytes_into_vec(size, &mut reader, &mut vec) {
            Ok(()) => (),
            _ => panic!(),
        };
        assert_eq!(vec.as_slice(), &ILINT_SAMPLE_BIN[0..size]);
    }

    let mut reader = ByteArrayReader::new(&ILINT_SAMPLE_BIN);
    let mut vec: Vec<u8> = Vec::new();
    match deserialize_bytes_into_vec(ILINT_SAMPLE_BIN.len() + 1, &mut reader, &mut vec) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    };
}

//=============================================================================
// DummyTag
//-----------------------------------------------------------------------------

const DEFAULT_DUMMY_TAG_ID: u64 = 0xFACADA;

struct DummyTag {
    id: u64,
    size: u64,
    dummy: usize,
}

impl DummyTag {
    pub fn new(id: u64, size: u64) -> DummyTag {
        DummyTag {
            id: id,
            size: size,
            dummy: 0,
        }
    }

    pub fn get_dummy(&self) -> usize {
        self.dummy
    }

    pub fn set_dummy(&mut self, value: usize) {
        self.dummy = value
    }
}

impl ILTag for DummyTag {
    fn id(&self) -> u64 {
        self.id
    }
    fn value_size(&self) -> u64 {
        self.size
    }
    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        for i in 0..self.size {
            match writer.write((i & 0xFF) as u8) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            }
        }
        Ok(())
    }
    fn deserialize_value(
        &mut self,
        _factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        if value_size != 4 {
            Err(ErrorKind::CorruptedData)
        } else {
            let mut buff: [u8; 4] = [0; 4];
            match reader.read_all(&mut buff) {
                Ok(()) => Ok(()),
                Err(e) => Err(ErrorKind::IOError(e)),
            }
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Default for DummyTag {
    fn default() -> Self {
        Self::new(DEFAULT_DUMMY_TAG_ID, 0)
    }
}

impl DefaultWithId for DummyTag {
    fn default_with_id(id: u64) -> Self {
        Self::new(id, 0)
    }
}

//=============================================================================
// ILTag
//-----------------------------------------------------------------------------
#[test]
fn test_iltag_is_implicity() {
    for i in 0..16 {
        let tag = DummyTag::new(i, 1);
        assert!(tag.is_implicity());
    }
    let tag = DummyTag::new(16, 1);
    assert!(!tag.is_implicity());
}

#[test]
fn test_iltag_is_reserved() {
    for i in 0..32 {
        let tag = DummyTag::new(i, 1);
        assert!(tag.is_reserved());
    }
    let tag = DummyTag::new(32, 1);
    assert!(!tag.is_reserved());
}

#[test]
fn test_iltag_size() {
    let tag = DummyTag::new(15, 3);
    assert_eq!(tag.size(), 1 + 0 + 3);

    let tag = DummyTag::new(31, 3);
    assert_eq!(tag.size(), 1 + 1 + 3);

    let tag = DummyTag::new(255, 255);
    assert_eq!(tag.size(), 2 + 2 + 255);
}

#[test]
fn test_iltag_serialize() {
    let mut writer = VecWriter::new();

    let tag = DummyTag::new(15, 4);
    assert!(tag.serialize(writer.as_writer()).is_ok());
    assert_eq!(writer.as_slice(), &[0x0F, 0x00, 0x01, 0x02, 0x03]);

    // Explicity tag
    let mut writer = VecWriter::new();

    let tag = DummyTag::new(16, 4);
    assert!(tag.serialize(writer.as_writer()).is_ok());
    assert_eq!(writer.as_slice(), &[0x10, 0x04, 0x00, 0x01, 0x02, 0x03]);

    // Explicity tag - long
    let mut writer = VecWriter::new();

    let tag = DummyTag::new(255, 256);
    assert!(tag.serialize(writer.as_writer()).is_ok());

    let mut exp: Vec<u8> = Vec::new();
    exp.extend_from_slice(&[248, 7, 248, 8]);
    for i in 0..256 {
        exp.push(i as u8);
    }
    assert_eq!(writer.as_slice(), exp);
}

#[test]
fn test_tag_downcast_ref() {
    let tag = DummyTag::new(132, 1);
    let t: &dyn ILTag = &tag;

    match tag_downcast_ref::<DummyTag>(t) {
        Some(v) => assert_eq!(0, v.get_dummy()),
        None => panic!(),
    }
}

#[test]
fn test_tag_downcast_mut() {
    let mut tag = DummyTag::new(132, 1);
    let t: &mut dyn ILTag = &mut tag;

    match tag_downcast_mut::<DummyTag>(t) {
        Some(v) => {
            v.set_dummy(1234);
            assert_eq!(1234, v.get_dummy())
        }
        None => panic!(),
    }
}

#[test]
fn test_tag_id_downcast_ref() {
    let tag = DummyTag::new(132, 1);
    let t: &dyn ILTag = &tag;

    match tag_id_downcast_ref::<DummyTag>(132, t) {
        Ok(v) => assert_eq!(0, v.get_dummy()),
        _ => panic!(),
    }
    match tag_id_downcast_ref::<DummyTag>(12, t) {
        Err(ErrorKind::UnexpectedTagType) => (),
        _ => panic!(),
    }
}

#[test]
fn test_tag_id_downcast_mut() {
    let mut tag = DummyTag::new(132, 1);
    let t: &mut dyn ILTag = &mut tag;

    match tag_id_downcast_mut::<DummyTag>(132, t) {
        Ok(v) => {
            v.set_dummy(1234);
            assert_eq!(1234, v.get_dummy())
        }
        _ => panic!(),
    }

    match tag_id_downcast_mut::<DummyTag>(12, t) {
        Err(ErrorKind::UnexpectedTagType) => (),
        _ => panic!(),
    }
}

//=============================================================================
// UntouchbleTagFactory
//-----------------------------------------------------------------------------
pub struct UntouchbleTagFactory {}

impl ILTagFactory for UntouchbleTagFactory {
    fn create_tag(&self, _tag_id: u64) -> Option<Box<dyn ILTag>> {
        panic!();
    }

    fn deserialize(&self, _reader: &mut dyn Reader) -> Result<Box<dyn ILTag>> {
        panic!();
    }
}

//=============================================================================
// ILDummyTagCreator
//-----------------------------------------------------------------------------
struct DummyTagCreator {
    id: u64,
}

impl DummyTagCreator {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl ILTagCreator for DummyTagCreator {
    fn create_empty_tag(&self, tag_id: u64) -> Box<dyn ILTag> {
        assert!(tag_id == self.id);
        Box::new(DummyTag::new(self.id, 1))
    }
}

#[test]
fn test_iltagcreator_for_dummytagcreator() {
    let c: DummyTagCreator = DummyTagCreator::new(16);

    let t = c.create_empty_tag(16);
    assert_eq!(t.id(), 16);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<DummyTag>());
}

#[test]
#[should_panic]
fn test_iltagcreator_for_dummytagcreator_error() {
    let c: DummyTagCreator = DummyTagCreator::new(16);
    c.create_empty_tag(17);
}

//=============================================================================
// ILDefaultTagCreator
//-----------------------------------------------------------------------------

#[test]
fn test_ildefaulttagcreator() {
    let creator = ILDefaultTagCreator::<DummyTag>::new();

    let t = creator.create_empty_tag(DEFAULT_DUMMY_TAG_ID);
    assert_eq!(t.id(), DEFAULT_DUMMY_TAG_ID);
}

#[test]
#[should_panic]
fn test_ildefaulttagcreator_bad_id() {
    let creator = ILDefaultTagCreator::<DummyTag>::new();

    let t = creator.create_empty_tag(0);
    assert_eq!(t.id(), DEFAULT_DUMMY_TAG_ID);
}

//=============================================================================
// ILDefaultWithIdTagCreator
//-----------------------------------------------------------------------------

#[test]
fn test_ildefaultwithidtagcreator() {
    let creator = ILDefaultWithIdTagCreator::<DummyTag>::new();

    let t = creator.create_empty_tag(0);
    assert_eq!(t.id(), 0);

    let t = creator.create_empty_tag(DEFAULT_DUMMY_TAG_ID);
    assert_eq!(t.id(), DEFAULT_DUMMY_TAG_ID);
}

//=============================================================================
// ILTagCreatorEngine
//-----------------------------------------------------------------------------

#[test]
fn test_iltagcreatorengine_new() {
    let e: ILTagCreatorEngine = ILTagCreatorEngine::new(false);
    assert!(!e.strict());

    let e: ILTagCreatorEngine = ILTagCreatorEngine::new(true);
    assert!(e.strict());
}

#[test]
fn test_iltagcreatorengine_normal() {
    let mut e: ILTagCreatorEngine = ILTagCreatorEngine::new(false);

    e.register(16, Box::new(DummyTagCreator::new(16)));
    e.register(17, Box::new(DummyTagCreator::new(17)));

    let t = match e.create_tag(16) {
        Some(v) => v,
        _ => panic!(),
    };
    assert_eq!(t.id(), 16);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<DummyTag>());

    let t = match e.create_tag(17) {
        Some(v) => v,
        _ => panic!(),
    };
    assert_eq!(t.id(), 17);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<DummyTag>());

    let t = match e.create_tag(18) {
        Some(v) => v,
        _ => panic!(),
    };
    assert_eq!(t.id(), 18);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<ILRawTag>());

    for id in 0..16 {
        match e.create_tag(id as u64) {
            None => (),
            _ => panic!(),
        };
    }
}

#[test]
fn test_iltagcreatorengine_strict() {
    let mut e: ILTagCreatorEngine = ILTagCreatorEngine::new(true);

    e.register(16, Box::new(DummyTagCreator::new(16)));
    e.register(17, Box::new(DummyTagCreator::new(17)));

    let t = match e.create_tag(16) {
        Some(v) => v,
        _ => panic!(),
    };
    assert_eq!(t.id(), 16);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<DummyTag>());

    let t = match e.create_tag(17) {
        Some(v) => v,
        _ => panic!(),
    };
    assert_eq!(t.id(), 17);
    assert_eq!(t.as_any().type_id(), ::std::any::TypeId::of::<DummyTag>());

    match e.create_tag(18) {
        None => (),
        _ => panic!(),
    };

    for id in 0..16 {
        match e.create_tag(id as u64) {
            None => (),
            _ => panic!(),
        };
    }
}

#[test]
fn test_iltagcreatorengine_deregister() {
    let mut e: ILTagCreatorEngine = ILTagCreatorEngine::new(true);

    e.register(16, Box::new(DummyTagCreator::new(16)));
    e.register(17, Box::new(DummyTagCreator::new(17)));

    let prev = match e.deregister(16) {
        Some(v) => v,
        None => panic!(),
    };
    let t = prev.create_empty_tag(16);
    assert_eq!(t.id(), 16);

    match e.deregister(16) {
        None => (),
        _ => panic!(),
    }

    match e.deregister(18) {
        None => (),
        _ => panic!(),
    }
}

//=============================================================================
// ILRawTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilrawtag_new() {
    let t = ILRawTag::new(16);
    assert_eq!(t.id(), 16);
    assert_eq!(t.value().len(), 0);
}

#[test]
#[should_panic]
fn test_ilrawtag_new_bad_id() {
    ILRawTag::new(15);
}

#[test]
fn test_ilrawtag_with_capacity() {
    let t = ILRawTag::with_capacity(16, 17);
    assert_eq!(t.id(), 16);
    assert_eq!(t.value().len(), 0);
    assert_eq!(t.value().capacity(), 17);
}

#[test]
#[should_panic]
fn test_ilrawtag_with_capacity_bad_id() {
    ILRawTag::with_capacity(15, 16);
}

#[test]
fn test_ilrawtag_with_value() {
    const SAMPLE: [u8; 5] = [0, 1, 2, 3, 4];
    let t = ILRawTag::with_value(16, &SAMPLE);
    assert_eq!(t.id(), 16);
    assert_eq!(t.value().len(), SAMPLE.len());
    assert_eq!(t.value().as_slice(), &SAMPLE);
}

#[test]
#[should_panic]
fn test_ilrawtag_with_value_bad_id() {
    const SAMPLE: [u8; 5] = [0, 1, 2, 3, 4];
    ILRawTag::with_value(15, &SAMPLE);
}

#[test]
fn test_ilrawtag_mut_value() {
    const SAMPLE: [u8; 5] = [0, 1, 2, 3, 4];
    let mut t = ILRawTag::new(16);
    t.mut_value().extend_from_slice(&SAMPLE);
    assert_eq!(t.value().as_slice(), &SAMPLE);
}

#[test]
fn test_ilrawtag_iltag_base() {
    let mut t = ILRawTag::new(16);

    assert_eq!(16, t.id());
    let _a: &dyn Any = t.as_any();
    let _b: &mut dyn Any = t.as_mut_any();
}

#[test]
fn test_ilrawtag_iltag_value_size() {
    const SAMPLE: [u8; 5] = [0, 1, 2, 3, 4];
    let mut t = ILRawTag::new(16);

    assert_eq!(t.value_size(), 0);
    t.mut_value().extend_from_slice(&SAMPLE);
    assert_eq!(t.value_size(), SAMPLE.len() as u64);
    assert_eq!(t.value().as_slice(), &SAMPLE);
}

#[test]
fn test_ilrawtag_iltag_deserialize_value() {
    const SAMPLE: [u8; 5] = [0, 1, 2, 3, 4];
    let mut reader = ByteArrayReader::new(&SAMPLE);
    let factory = UntouchbleTagFactory {};
    let mut t = ILRawTag::new(16);

    match t.deserialize_value(&factory, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(t.value_size(), 0);

    match t.deserialize_value(&factory, SAMPLE.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(t.value_size(), SAMPLE.len() as u64);
    assert_eq!(t.value().as_slice(), &SAMPLE);

    match t.deserialize_value(&factory, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(t.value_size(), 0);

    match t.deserialize_value(&factory, 1, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_ilrawtag_default_with_id() {
    let c = ILDefaultWithIdTagCreator::<ILRawTag>::new();
    let t = c.create_empty_tag(16);
    assert_eq!(t.id(), 16);
}
