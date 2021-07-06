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
use crate::tags::tests::UntouchbleTagFactory;

/// Test the functions defined by std_byte_array_tag_func_impl macro.
macro_rules! test_std_byte_array_tag_func_impl {
    ($tag_type: ty, $tag_id: expr) => {
        let sample: [u8; 4] = [1, 2, 3, 4];

        // new
        let t = <$tag_type>::new();
        assert_eq!(t.id(), $tag_id);
        assert_eq!(t.value().len(), 0);

        // with_value
        let t = <$tag_type>::with_value(&sample);
        assert_eq!(t.id(), $tag_id);
        assert_eq!(t.value().len(), sample.len());
        assert_eq!(t.value().as_slice(), &sample);

        let mut t = <$tag_type>::new();
        assert_eq!(t.id(), $tag_id);
        assert_eq!(t.value().len(), 0);
        t.mut_value().extend_from_slice(&sample);
        assert_eq!(t.value().len(), sample.len());
        assert_eq!(t.value().as_slice(), &sample);
    };
}

macro_rules! test_inner_iltag_default_impl_value_size {
    ($tag_type: ty, $tag_id: expr) => {
        let sample: [u8; 4] = [1, 2, 3, 4];

        let t = <$tag_type>::new();
        assert_eq!(t.value_size(), 0);

        let t = <$tag_type>::with_value(&sample);
        assert_eq!(t.value_size(), sample.len() as u64);

        let mut t = <$tag_type>::new();
        assert_eq!(t.value_size(), 0);
        t.mut_value().extend_from_slice(&sample);
        assert_eq!(t.value_size(), sample.len() as u64);
    };
}

macro_rules! test_inner_iltag_default_impl_serialize {
    ($tag_type: ty) => {
        let sample: [u8; 4] = [1, 2, 3, 4];

        let t = <$tag_type>::new();
        let mut writer = VecWriter::new();
        match t.serialize_value(&mut writer) {
            Ok(()) => (),
            _ => panic!("Unable to serialize."),
        }
        assert_eq!(writer.as_slice().len(), 0);

        let t = <$tag_type>::with_value(&sample);
        let mut writer = VecWriter::new();
        match t.serialize_value(&mut writer) {
            Ok(()) => (),
            _ => panic!("Unable to serialize."),
        }
        assert_eq!(writer.as_slice().len(), sample.len());
        assert_eq!(writer.as_slice(), &sample);
    };
}

macro_rules! test_inner_iltag_default_impl_deserialize {
    ($tag_type: ty) => {
        let f = UntouchbleTagFactory::new();
        let sample: [u8; 4] = [1, 2, 3, 4];

        let mut reader = ByteArrayReader::new(&sample);
        let mut t = <$tag_type>::new();
        match t.deserialize_value(&f, 0, &mut reader) {
            Ok(()) => (),
            _ => panic!("Unable to serialize."),
        }
        assert_eq!(t.value().len(), 0);

        let mut reader = ByteArrayReader::new(&sample);
        let mut t = <$tag_type>::new();
        match t.deserialize_value(&f, sample.len(), &mut reader) {
            Ok(()) => (),
            _ => panic!("Unable to serialize."),
        }
        assert_eq!(t.value().len(), sample.len());
        assert_eq!(t.value().as_slice(), &sample);

        let mut reader = ByteArrayReader::new(&sample);
        let mut t = <$tag_type>::new();
        match t.deserialize_value(&f, sample.len() + 1, &mut reader) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("I should not be able to deserialize."),
        }
    };
}

macro_rules! test_default_impl {
    ($tag_type: ty, $tag_id: expr) => {
        let t = <$tag_type>::default();
        assert_eq!(t.id(), $tag_id);
    };
}

//=============================================================================
// ILByteArrayTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilbytearraytag_impl() {
    test_std_byte_array_tag_func_impl!(ILByteArrayTag, IL_BYTES_TAG_ID);

    // with_capacity
    let t = ILByteArrayTag::with_capacity(10);
    assert_eq!(t.id(), IL_BYTES_TAG_ID);
    assert_eq!(t.value().len(), 0);
    assert_eq!(t.value().capacity(), 10);
}

#[test]
fn test_ilbytearraytag_iltag_value_size() {
    test_inner_iltag_default_impl_value_size!(ILByteArrayTag, IL_BYTES_TAG_ID);
}

#[test]
fn test_ilbytearraytag_iltag_serialize() {
    test_inner_iltag_default_impl_serialize!(ILByteArrayTag);
}

#[test]
fn test_ilbytearraytag_iltag_deserialize() {
    test_inner_iltag_default_impl_deserialize!(ILByteArrayTag);
}

#[test]
fn test_ilbytearraytag_default() {
    test_default_impl!(ILByteArrayTag, IL_BYTES_TAG_ID);
}

//=============================================================================
// ILStringTag
//-----------------------------------------------------------------------------
/// By Ruy Barbosa de Oliveira - Good because of the 2 byte UTF-8 characters
static SAMPLE_STRING: &str = "A justiça, cega para um dos dois lados, já não é justiça.";
static SAMPLE_STRING_BIN: [u8; 62] = [
    0x41, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x69, 0xc3, 0xa7, 0x61, 0x2c, 0x20, 0x63, 0x65, 0x67, 0x61,
    0x20, 0x70, 0x61, 0x72, 0x61, 0x20, 0x75, 0x6d, 0x20, 0x64, 0x6f, 0x73, 0x20, 0x64, 0x6f, 0x69,
    0x73, 0x20, 0x6c, 0x61, 0x64, 0x6f, 0x73, 0x2c, 0x20, 0x6a, 0xc3, 0xa1, 0x20, 0x6e, 0xc3, 0xa3,
    0x6f, 0x20, 0xc3, 0xa9, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x69, 0xc3, 0xa7, 0x61, 0x2e,
];

#[test]
fn test_ilstringtag_impl() {
    // new
    let t = ILStringTag::new();
    assert_eq!(t.id(), IL_STRING_TAG_ID);
    assert_eq!(t.value(), "");

    // with_id
    let t = ILStringTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value(), "");

    // with_value
    let t = ILStringTag::with_value(SAMPLE_STRING);
    assert_eq!(t.id(), IL_STRING_TAG_ID);
    assert_eq!(t.value(), SAMPLE_STRING);

    // with_value
    let t = ILStringTag::with_id_value(1234, SAMPLE_STRING);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value(), SAMPLE_STRING);

    // value
    let mut t = ILStringTag::new();
    t.mut_value().insert_str(0, SAMPLE_STRING);
    assert_eq!(t.value(), SAMPLE_STRING);

    // value
    let mut t = ILStringTag::new();
    t.set_value(SAMPLE_STRING);
    assert_eq!(t.value(), SAMPLE_STRING);
}

#[test]
fn test_ilstringtag_iltag_value_size() {
    let t = ILStringTag::new();
    assert_eq!(t.value_size(), 0);

    let t = ILStringTag::with_value(SAMPLE_STRING);
    assert_eq!(t.value_size(), SAMPLE_STRING.len() as u64);
}

#[test]
fn test_ilstringtag_iltag_serialize() {
    let t = ILStringTag::new();
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the string."),
    }
    assert_eq!(writer.as_slice().len(), 0);

    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the string."),
    }
    assert_eq!(writer.as_slice().len(), SAMPLE_STRING_BIN.len());
    assert_eq!(writer.as_slice(), &SAMPLE_STRING_BIN);
}

#[test]
fn test_ilstringtag_iltag_deserialize() {
    let f = UntouchbleTagFactory::new();

    let mut reader = ByteArrayReader::new(&SAMPLE_STRING_BIN);
    let mut t = ILStringTag::new();
    match t.deserialize_value(&f, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the string."),
    }
    assert_eq!(t.value().len(), 0);

    let mut reader = ByteArrayReader::new(&SAMPLE_STRING_BIN);
    let mut t = ILStringTag::new();
    match t.deserialize_value(&f, SAMPLE_STRING_BIN.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the string."),
    }
    assert_eq!(t.value().len(), SAMPLE_STRING.len());
    assert_eq!(t.value(), SAMPLE_STRING);

    let mut reader = ByteArrayReader::new(&SAMPLE_STRING_BIN);
    let mut t = ILStringTag::new();
    match t.deserialize_value(&f, SAMPLE_STRING_BIN.len() + 1, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Should fail."),
    }

    let mut reader = ByteArrayReader::new(&SAMPLE_STRING_BIN);
    let mut t = ILStringTag::new();
    match t.deserialize_value(&f, 8, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Should fail."),
    }
}

#[test]
fn test_string_tag_size_from_value() {
    let mut exp = crate::ilint::encoded_size(IL_STRING_TAG_ID);
    exp += crate::ilint::encoded_size(0);
    assert_eq!(string_tag_size_from_value(""), exp as u64);

    let mut exp = crate::ilint::encoded_size(IL_STRING_TAG_ID);
    exp += crate::ilint::encoded_size(SAMPLE_STRING.len() as u64);
    exp += SAMPLE_STRING.len();
    assert_eq!(string_tag_size_from_value(SAMPLE_STRING), exp as u64);
}

#[test]
fn test_serialize_string_tag_from_value() {
    // Empty
    let t = ILStringTag::new();
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut writer = VecWriter::new();
    match serialize_string_tag_from_value("", &mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut writer = VecWriter::new();
    match serialize_string_tag_from_value(SAMPLE_STRING, &mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());
}

#[test]
fn test_deserialize_string_tag_from_value_into() {
    let mut ret = String::default();

    // Empty
    let t = ILStringTag::new();
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(ret, "");

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(ret, SAMPLE_STRING);

    // Incomplete
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(&exp.as_slice()[0..SAMPLE_STRING.len() - 1]);
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    }

    // Corrupted UTF-8
    let t = ILRawTag::with_value(IL_STRING_TAG_ID, &SAMPLE_STRING_BIN[..8]);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error not detected."),
    }
}

#[test]
fn test_deserialize_string_tag_from_value() {
    // Empty
    let t = ILStringTag::new();
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let ret = match deserialize_string_tag_from_value(&mut reader) {
        Ok(ret) => ret,
        _ => panic!("Unable to write the tag."),
    };
    assert_eq!(ret, "");

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let ret = match deserialize_string_tag_from_value(&mut reader) {
        Ok(ret) => ret,
        _ => panic!("Unable to write the tag."),
    };
    assert_eq!(ret, SAMPLE_STRING);

    // Incomplete
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(&exp.as_slice()[0..SAMPLE_STRING.len() - 1]);
    match deserialize_string_tag_from_value(&mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    };

    // Corrupted UTF-8
    let t = ILRawTag::with_value(IL_STRING_TAG_ID, &SAMPLE_STRING_BIN[..8]);
    let mut exp = VecWriter::new();
    match t.serialize(&mut exp) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value(&mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error not detected."),
    };
}

//=============================================================================
// ILBigIntTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilbiginttag_impl() {
    test_std_byte_array_tag_func_impl!(ILBigIntTag, IL_BINT_TAG_ID);
}

#[test]
fn test_ilbiginttag_iltag_value_size() {
    test_inner_iltag_default_impl_value_size!(ILBigIntTag, IL_BINT_TAG_ID);
}

#[test]
fn test_ilbiginttag_iltag_serialize() {
    test_inner_iltag_default_impl_serialize!(ILBigIntTag);
}

#[test]
fn test_ilbiginttag_iltag_deserialize() {
    test_inner_iltag_default_impl_deserialize!(ILBigIntTag);
}

#[test]
fn test_ilbiginttag_default() {
    test_default_impl!(ILBigIntTag, IL_BINT_TAG_ID);
}

//=============================================================================
// ILBigDecTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilbigdectag_impl() {
    // new
    let t = ILBigDecTag::new();
    assert_eq!(t.id(), IL_BDEC_TAG_ID);
    assert_eq!(t.scale(), 0);
    assert_eq!(t.value().len(), 0);

    // with_id
    let t = ILBigDecTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.scale(), 0);
    assert_eq!(t.value().len(), 0);

    // with_value
    let t = ILBigDecTag::with_value(-1, &SAMPLE_STRING_BIN);
    assert_eq!(t.id(), IL_BDEC_TAG_ID);
    assert_eq!(t.scale(), -1);
    assert_eq!(t.value().as_slice(), &SAMPLE_STRING_BIN);

    // with_id_value
    let t = ILBigDecTag::with_id_value(1234, -1, &SAMPLE_STRING_BIN);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.scale(), -1);
    assert_eq!(t.value().as_slice(), &SAMPLE_STRING_BIN);

    // Mut
    let mut t = ILBigDecTag::new();
    t.set_scale(-123);
    assert_eq!(t.scale(), -123);
    t.mut_value().extend_from_slice(&SAMPLE_STRING_BIN);
    assert_eq!(t.value().as_slice(), &SAMPLE_STRING_BIN);
}

#[test]
fn test_ilbigdectag_iltag_value_size() {
    // new
    let t = ILBigDecTag::new();
    assert_eq!(t.value_size(), 4 + 0);

    // with_value
    let t = ILBigDecTag::with_value(-1, &SAMPLE_STRING_BIN);
    assert_eq!(t.value_size(), (4 + SAMPLE_STRING_BIN.len()) as u64);
}

#[test]
fn test_ilbigdectag_iltag_serialize_value() {
    // Empty
    let exp: [u8; 4] = [0; 4];
    let t = ILBigDecTag::new();
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(writer.vec().as_slice(), &exp);

    // with_value
    let exp: [u8; 10] = [0x12, 0x34, 0x56, 0x78, 0x41, 0x20, 0x6a, 0x75, 0x73, 0x74];
    let t = ILBigDecTag::with_value(0x12345678, &SAMPLE_STRING_BIN[0..6]);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(writer.vec().as_slice(), &exp);
}

#[test]
fn test_ilbigdectag_iltag_deserialize_value() {
    let f = UntouchbleTagFactory::new();

    // Empty
    let exp: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
    let mut reader = ByteArrayReader::new(&exp);
    let mut t = ILBigDecTag::with_value(0x12345678, &SAMPLE_STRING_BIN[0..6]);
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(t.scale(), -1);
    assert_eq!(t.value().len(), 0);

    // with_value
    let exp: [u8; 10] = [0x12, 0x34, 0x56, 0x78, 0x41, 0x20, 0x6a, 0x75, 0x73, 0x74];
    let mut reader = ByteArrayReader::new(&exp);
    let mut t = ILBigDecTag::default();
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(t.scale(), 0x12345678);
    assert_eq!(t.value().as_slice(), &SAMPLE_STRING_BIN[0..6]);

    for size in 0..3 {
        let mut reader = ByteArrayReader::new(&exp);
        let mut t = ILBigDecTag::default();
        match t.deserialize_value(&f, size, &mut reader) {
            Err(ErrorKind::CorruptedData) => (),
            _ => panic!("Unable to detect the data corruption."),
        }
    }
}

//=============================================================================
// ILILIntArrayTag
//-----------------------------------------------------------------------------
