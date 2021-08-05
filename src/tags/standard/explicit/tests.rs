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
use crate::io::data::*;
use crate::io::Writer;
use crate::tags::standard::factory::ILStandardTagFactory;
use crate::tags::standard::implicit::*;
use crate::tags::tests::UntouchbleTagFactory;
use crate::tags::util::*;

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

macro_rules! write_value_for_testing {
    ($write_func: ident, $value: expr, $writer: expr) => {
        match $write_func($value, $writer) {
            Ok(()) => (),
            _ => panic!("Unable to write the value."),
        }
    };
}

/// Serializes a tag for testing purposes. It panics in case of error.
fn serialize_tag_for_testing(tag: &dyn ILTag, writer: &mut dyn Writer) {
    match tag.serialize(writer) {
        Ok(()) => (),
        _ => panic!(""),
    }
}

/// Computes the total size of the tags inside a slice.
fn compute_box_tag_slice_size(values: &[Box<dyn ILTag>]) -> u64 {
    values.iter().map(|t| t.size()).sum::<u64>()
}

/// Serializes all tags in the given slice.
fn serialize_box_tag_slice_size(values: &[Box<dyn ILTag>], writer: &mut dyn Writer) {
    values
        .iter()
        .for_each(|t| serialize_tag_for_testing(t.as_ref(), writer));
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
    serialize_tag_for_testing(&t, &mut exp);
    let mut writer = VecWriter::new();
    match serialize_string_tag_from_value("", &mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
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
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(ret, "");

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(ret, SAMPLE_STRING);

    // Incomplete
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(&exp.as_slice()[0..SAMPLE_STRING.len() - 1]);
    match deserialize_string_tag_from_value_into(&mut reader, &mut ret) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    }

    // Corrupted UTF-8
    let t = ILRawTag::with_value(IL_STRING_TAG_ID, &SAMPLE_STRING_BIN[..8]);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
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
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let ret = match deserialize_string_tag_from_value(&mut reader) {
        Ok(ret) => ret,
        _ => panic!("Unable to write the tag."),
    };
    assert_eq!(ret, "");

    // With content
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let ret = match deserialize_string_tag_from_value(&mut reader) {
        Ok(ret) => ret,
        _ => panic!("Unable to write the tag."),
    };
    assert_eq!(ret, SAMPLE_STRING);

    // Incomplete
    let t = ILStringTag::with_value(SAMPLE_STRING);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
    let mut reader = ByteArrayReader::new(&exp.as_slice()[0..SAMPLE_STRING.len() - 1]);
    match deserialize_string_tag_from_value(&mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    };

    // Corrupted UTF-8
    let t = ILRawTag::with_value(IL_STRING_TAG_ID, &SAMPLE_STRING_BIN[..8]);
    let mut exp = VecWriter::new();
    serialize_tag_for_testing(&t, &mut exp);
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

static SAMPLE_ILINT: [u64; 9] = [
    0xf7,
    0xf8,
    0xFEDC,
    0xFEDCBA,
    0xFEDCBA98,
    0xFEDCBA76,
    0xFEDCBA7654,
    0xFEDCBA765432,
    0xFEDCBA76543210,
];

#[test]
fn test_ililintarraytag_impl() {
    // new
    let t = ILILIntArrayTag::new();
    assert_eq!(t.id(), IL_ILINTARRAY_TAG_ID);
    assert_eq!(t.value().len(), 0);

    // with_id
    let t = ILILIntArrayTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value().len(), 0);

    // with_value
    let t = ILILIntArrayTag::with_value(&SAMPLE_ILINT);
    assert_eq!(t.id(), IL_ILINTARRAY_TAG_ID);
    assert_eq!(t.value().as_slice(), &SAMPLE_ILINT);

    // with_id_value
    let t = ILILIntArrayTag::with_id_value(1234, &SAMPLE_ILINT);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value().as_slice(), &SAMPLE_ILINT);

    // mut
    let mut t = ILILIntArrayTag::default();
    t.mut_value().extend_from_slice(&SAMPLE_ILINT);
    assert_eq!(t.value().as_slice(), &SAMPLE_ILINT);
}

#[test]
fn test_ililintarraytag_iltag_value_size() {
    // Empty
    let t = ILILIntArrayTag::new();
    let exp = crate::ilint::encoded_size(0);
    assert_eq!(t.value_size(), exp as u64);

    // With value
    let t = ILILIntArrayTag::with_value(&SAMPLE_ILINT);
    let mut exp = crate::ilint::encoded_size(SAMPLE_ILINT.len() as u64);
    for v in SAMPLE_ILINT {
        exp += crate::ilint::encoded_size(v);
    }
    assert_eq!(t.value_size(), exp as u64);
}

#[test]
fn test_ililintarraytag_iltag_serialize_value() {
    // Empty
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, 0, &mut exp);
    let t = ILILIntArrayTag::new();
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.vec().as_slice());

    // With value
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, SAMPLE_ILINT.len() as u64, &mut exp);
    SAMPLE_ILINT.iter().for_each(|x| {
        write_value_for_testing!(write_ilint, *x, &mut exp);
    });

    //for v in SAMPLE_ILINT {}
    let t = ILILIntArrayTag::with_value(&SAMPLE_ILINT);
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.vec().as_slice());
}

#[test]
fn test_ililintarraytag_iltag_deserialize_value() {
    let f = UntouchbleTagFactory::new();

    // Empty
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, 0, &mut exp);
    let mut t = ILILIntArrayTag::with_value(&SAMPLE_ILINT);
    let mut reader = ByteArrayReader::new(exp.vec().as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(t.value().len(), 0);

    // With value
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, SAMPLE_ILINT.len() as u64, &mut exp);
    SAMPLE_ILINT
        .iter()
        .for_each(|x| write_value_for_testing!(write_ilint, *x, &mut exp));
    let mut t = ILILIntArrayTag::default();
    let mut reader = ByteArrayReader::new(exp.vec().as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to write the tag."),
    }
    assert_eq!(t.value().as_slice(), &SAMPLE_ILINT);

    // Incomplete
    let mut t = ILILIntArrayTag::default();
    let mut reader = ByteArrayReader::new(exp.vec().as_slice());
    match t.deserialize_value(&f, exp.vec().len() - 1, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    }

    let mut t = ILILIntArrayTag::default();
    let mut reader = ByteArrayReader::new(&exp.vec().as_slice()[0..exp.vec().len() - 1]);
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    }

    // Corrupted
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, (SAMPLE_ILINT.len() - 1) as u64, &mut exp);
    SAMPLE_ILINT
        .iter()
        .for_each(|x| write_value_for_testing!(write_ilint, *x, &mut exp));

    let mut t = ILILIntArrayTag::default();
    let mut reader = ByteArrayReader::new(exp.vec().as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error not detected."),
    }

    // Broken ILInt size
    let sample: [u8; 1] = [0xF8];
    let mut t = ILILIntArrayTag::default();
    let mut reader = ByteArrayReader::new(&sample);
    match t.deserialize_value(&f, 1, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error not detected."),
    }
}

//=============================================================================
// ILTagSeqTag
//-----------------------------------------------------------------------------
#[test]
fn test_iltagseqtag_impl() {
    // new
    let t = ILTagSeqTag::new();
    assert_eq!(t.id(), IL_ILTAGSEQ_TAG_ID);
    assert_eq!(t.value().len(), 0);

    // with_id
    let t = ILTagSeqTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value().len(), 0);

    // mut
    let mut t = ILTagSeqTag::default();
    t.mut_value().push(Box::new(ILStringTag::default()));
    assert_eq!(t.value().len(), 1);
}

#[test]
fn test_iltagseqtag_iltag_value_size() {
    // empty
    let t = ILTagSeqTag::new();
    assert_eq!(t.value_size(), 0);

    // with one
    let mut t = ILTagSeqTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));
    assert_eq!(
        t.value_size(),
        compute_box_tag_slice_size(t.value().as_slice())
    );

    // with two
    t.mut_value()
        .push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    assert_eq!(
        t.value_size(),
        compute_box_tag_slice_size(t.value().as_slice())
    );
}

#[test]
fn test_iltagseqtag_iltag_serialize_value() {
    // empty
    let exp: [u8; 0] = [];
    let t = ILTagSeqTag::new();
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), &exp);

    // with one
    let mut t = ILTagSeqTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));
    let mut exp = VecWriter::default();
    serialize_box_tag_slice_size(t.value().as_slice(), &mut exp);
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.as_slice());

    // with two
    t.mut_value()
        .push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    serialize_box_tag_slice_size(t.value().as_slice(), &mut exp);
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.as_slice());
}

#[test]
fn test_iltagseqtag_iltag_deserialize_value_non_strict() {
    let f = ILStandardTagFactory::new(false);

    // empty
    let exp: [u8; 0] = [];
    let mut t = ILTagSeqTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));

    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.value().len(), 0);

    let mut sample: Vec<Box<dyn ILTag>> = Vec::new();

    // with one tag
    sample.push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagSeqTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // with multiple tags
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILILInt64Tag::with_id_value(1234, 12345678)));
    sample.push(Box::new(ILNullTag::default()));
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagSeqTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // Errors
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len() - 2, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
}

#[test]
fn test_iltagseqtag_iltag_deserialize_value_strict() {
    let f = ILStandardTagFactory::new(true);

    // empty
    let exp: [u8; 0] = [];
    let mut t = ILTagSeqTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));

    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), 0);

    let mut sample: Vec<Box<dyn ILTag>> = Vec::new();

    // with one tag
    sample.push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagSeqTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // with multiple tags
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILNullTag::default()));
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagSeqTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // Errors
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len() - 2, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // with unknown tag
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILILInt64Tag::with_id_value(1234, 12345678)));
    sample.push(Box::new(ILNullTag::default()));
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagSeqTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
}

//=============================================================================
// ILTagArrayTag
//-----------------------------------------------------------------------------
#[test]
fn test_iltagarraytag_impl() {
    // new
    let t = ILTagArrayTag::new();
    assert_eq!(t.id(), IL_ILTAGARRAY_TAG_ID);
    assert_eq!(t.value().len(), 0);

    // with_id
    let t = ILTagArrayTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value().len(), 0);

    // mut
    let mut t = ILTagArrayTag::default();
    t.mut_value().push(Box::new(ILStringTag::default()));
    assert_eq!(t.value().len(), 1);
}

#[test]
fn test_iltagarraytag_iltag_value_size() {
    // empty
    let t = ILTagArrayTag::new();
    assert_eq!(t.value_size(), 1);

    // with one
    let mut t = ILTagArrayTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));
    assert_eq!(
        t.value_size(),
        ((crate::ilint::encoded_size(t.value().len() as u64) as u64)
            + compute_box_tag_slice_size(t.value().as_slice()))
    );

    // with two
    t.mut_value()
        .push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    assert_eq!(
        t.value_size(),
        ((crate::ilint::encoded_size(t.value().len() as u64) as u64)
            + compute_box_tag_slice_size(t.value().as_slice()))
    );
}

#[test]
fn test_iltagarraytag_iltag_serialize_value() {
    // empty
    let exp: [u8; 1] = [0];
    let t = ILTagArrayTag::new();
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), &exp);

    // with one
    let mut t = ILTagArrayTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, t.value().len() as u64, &mut exp);
    serialize_box_tag_slice_size(t.value().as_slice(), &mut exp);
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.as_slice());

    // with two
    t.mut_value()
        .push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, t.value().len() as u64, &mut exp);
    serialize_box_tag_slice_size(t.value().as_slice(), &mut exp);
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.vec().as_slice(), exp.as_slice());
}

#[test]
fn test_iltagarraytag_iltag_deserialize_value_non_strict() {
    let f = ILStandardTagFactory::new(false);

    // empty
    let exp: [u8; 1] = [0];
    let mut t = ILTagArrayTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));
    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, 1, &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.value().len(), 0);

    let mut sample: Vec<Box<dyn ILTag>> = Vec::new();

    // with one tag
    sample.push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagArrayTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // with multiple tags
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILILInt64Tag::with_id_value(1234, 12345678)));
    sample.push(Box::new(ILNullTag::default()));
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagArrayTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // Error - last is corrupted
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len() - 2, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Counter is corrupted
    match t.deserialize_value(&f, exp.vec().len() - 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Unable to serialize the tag."),
    }
}

#[test]
fn test_iltagarraytag_iltag_deserialize_value_strict() {
    let f = ILStandardTagFactory::new(true);

    // empty
    let exp: [u8; 1] = [0];
    let mut t = ILTagArrayTag::new();
    t.mut_value().push(Box::new(ILStringTag::default()));

    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, 1, &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), 0);

    let mut sample: Vec<Box<dyn ILTag>> = Vec::new();

    // with one tag
    sample.push(Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagArrayTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // with multiple tags
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILNullTag::default()));
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagArrayTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
    assert_eq!(t.value().len(), sample.len());
    for i in 0..sample.len() {
        assert!(iltag_are_equal(t.value()[i].as_ref(), sample[i].as_ref()));
    }

    // Error - last is corrupted
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len() - 2, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Counter is corrupted
    match t.deserialize_value(&f, exp.vec().len() - 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // with unknown tag
    let mut exp = VecWriter::default();
    sample.push(Box::new(ILILInt64Tag::with_value(12345678)));
    sample.push(Box::new(ILILInt64Tag::with_id_value(1234, 12345678)));
    sample.push(Box::new(ILNullTag::default()));
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    serialize_box_tag_slice_size(sample.as_slice(), &mut exp);
    let mut t = ILTagArrayTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.vec().len(), &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to deserialize the tag."),
    }
}

//=============================================================================
// ILRangeTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilrangetag_impl() {
    // new
    let t = ILRangeTag::new();
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 0);
    assert_eq!(t.count(), 0);

    // with_id
    let t = ILRangeTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.start(), 0);
    assert_eq!(t.count(), 0);

    // with_value
    let t = ILRangeTag::with_value(124, 123);
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 124);
    assert_eq!(t.count(), 123);

    // with_value
    let t = ILRangeTag::with_id_value(1234, 124, 123);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.start(), 124);
    assert_eq!(t.count(), 123);

    // mut
    let mut t = ILRangeTag::default();
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 0);
    assert_eq!(t.count(), 0);

    t.set_start(123);
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 123);
    assert_eq!(t.count(), 0);

    t.set_count(124);
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 123);
    assert_eq!(t.count(), 124);

    t.set_value(321, 322);
    assert_eq!(t.id(), IL_RANGE_TAG_ID);
    assert_eq!(t.start(), 321);
    assert_eq!(t.count(), 322);
}

#[test]
fn test_ilrangetag_iltag_value_size() {
    let t = ILRangeTag::new();
    assert_eq!(
        t.value_size(),
        (crate::ilint::encoded_size(t.start()) + 2) as u64
    );

    let t = ILRangeTag::with_value(123123, 65535);
    assert_eq!(
        t.value_size(),
        (crate::ilint::encoded_size(t.start()) + 2) as u64
    );

    let t = ILRangeTag::with_value(0xFFFF_FFFF_FFFF_FFFF, 65535);
    assert_eq!(
        t.value_size(),
        (crate::ilint::encoded_size(t.start()) + 2) as u64
    );
}

#[test]
fn test_ilrangetag_iltag_serialize_value() {
    let start: u64 = 1;
    let count: u16 = 2;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let t = ILRangeTag::with_value(start, count);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());

    let start: u64 = 123123;
    let count: u16 = 65535;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let t = ILRangeTag::with_value(start, count);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());

    let start: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let count: u16 = 65535;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let t = ILRangeTag::with_value(start, count);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(exp.as_slice(), writer.as_slice());
}

#[test]
fn test_ilrangetag_iltag_deserialize_value() {
    let f = UntouchbleTagFactory::new();

    let start: u64 = 1;
    let count: u16 = 2;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let mut t = ILRangeTag::default();
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(t.start(), start);
    assert_eq!(t.count(), count);

    let start: u64 = 123123;
    let count: u16 = 65535;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let mut t = ILRangeTag::default();
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(t.start(), start);
    assert_eq!(t.count(), count);

    let start: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let count: u16 = 65535;
    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, start, &mut exp);
    write_value_for_testing!(write_u16, count, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let mut t = ILRangeTag::default();
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Serialization failed."),
    }
    assert_eq!(t.start(), start);
    assert_eq!(t.count(), count);

    // Broken
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let mut t = ILRangeTag::default();
    match t.deserialize_value(&f, exp.as_slice().len() - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Serialization failed."),
    }

    // Too much bytes
    write_value_for_testing!(write_u8, 0, &mut exp);
    let mut reader = ByteArrayReader::new(exp.as_slice());
    let mut t = ILRangeTag::default();
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Err(_) => (),
        _ => panic!("Serialization failed."),
    }
}

//=============================================================================
// ILVersionTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilversiontag_impl() {
    let sample: [i32; 4] = [1, 2, 3, 4];

    let t = ILVersionTag::new();
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 0);
    assert_eq!(t.minor(), 0);
    assert_eq!(t.revision(), 0);
    assert_eq!(t.build(), 0);

    let t = ILVersionTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.major(), 0);
    assert_eq!(t.minor(), 0);
    assert_eq!(t.revision(), 0);
    assert_eq!(t.build(), 0);

    let t = ILVersionTag::with_value(1, 2, 3, 4);
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let t = ILVersionTag::with_id_value(1234, 1, 2, 3, 4);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let t = ILVersionTag::with_value_from_slice(&sample);
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let t = ILVersionTag::with_id_value_from_slice(1234, &sample);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let mut t = ILVersionTag::default();
    t.set_major(1);
    t.set_minor(2);
    t.set_revision(3);
    t.set_build(4);
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let mut t = ILVersionTag::default();
    t.set_value(1, 2, 3, 4);
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);

    let mut t = ILVersionTag::default();
    t.set_value_from_slice(&sample);
    assert_eq!(t.id(), IL_VERSION_TAG_ID);
    assert_eq!(t.major(), 1);
    assert_eq!(t.minor(), 2);
    assert_eq!(t.revision(), 3);
    assert_eq!(t.build(), 4);
}

#[test]
fn test_ilversiontag_iltag_value_size() {
    let t = ILVersionTag::new();
    assert_eq!(t.value_size(), 16);

    let t = ILVersionTag::with_value(1, 2, 3, 4);
    assert_eq!(t.value_size(), 16);
}

#[test]
fn test_ilversiontag_iltag_serialize_value() {
    let sample: [i32; 4] = [1, 2, 3, 4];

    let exp: [u8; 16] = [0; 16];
    let t = ILVersionTag::new();
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize."),
    }
    assert_eq!(writer.as_slice(), &exp);

    let exp: [u8; 16] = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4];
    let t = ILVersionTag::with_value_from_slice(&sample);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize."),
    }
    assert_eq!(writer.as_slice(), &exp);
}

#[test]
fn test_ilversiontag_iltag_deserialize_value() {
    let f = UntouchbleTagFactory::new();

    let sample: [i32; 4] = [0; 4];
    let exp: [u8; 16] = [0; 16];
    let mut reader = ByteArrayReader::new(&exp);
    let mut t = ILVersionTag::new();
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize."),
    }
    assert_eq!(t.value(), &sample);

    let sample: [i32; 4] = [1, 2, 3, 4];
    let exp: [u8; 16] = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4];
    let mut reader = ByteArrayReader::new(&exp);
    let mut t = ILVersionTag::new();
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize."),
    }
    assert_eq!(t.value(), &sample);

    // Failed
    let mut reader = ByteArrayReader::new(&exp[0..15]);
    let mut t = ILVersionTag::new();
    match t.deserialize_value(&f, 16, &mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unable to serialize."),
    }

    let mut reader = ByteArrayReader::new(&exp[0..15]);
    let mut t = ILVersionTag::new();
    match t.deserialize_value(&f, 15, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Unable to serialize."),
    }

    let mut reader = ByteArrayReader::new(&exp);
    let mut t = ILVersionTag::new();
    match t.deserialize_value(&f, 17, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Unable to serialize."),
    }
}

//=============================================================================
// ILOIDTag
//-----------------------------------------------------------------------------
#[test]
fn test_iloidtag_impl() {
    let sample: [u64; 4] = [0, 0xFF, 0xFFFF, 0xFFFF_FFFF_FFFF_FFFF];

    let t = ILOIDTag::new();
    assert_eq!(t.id(), IL_OID_TAG_ID);
    assert_eq!(t.value().len(), 0);

    let t = ILOIDTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value().len(), 0);

    let t = ILOIDTag::with_value(&sample);
    assert_eq!(t.id(), IL_OID_TAG_ID);
    assert_eq!(t.value(), &sample);

    let t = ILOIDTag::with_id_value(1234, &sample);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.value(), &sample);

    // mutable
    let mut t = ILOIDTag::new();
    t.set_value(&sample);
    assert_eq!(t.id(), IL_OID_TAG_ID);
    assert_eq!(t.value(), &sample);

    t.set_value(&sample[..2]);
    assert_eq!(t.id(), IL_OID_TAG_ID);
    assert_eq!(t.value(), &sample[..2]);

    t.mut_value().clear();
    assert_eq!(t.id(), IL_OID_TAG_ID);
    assert_eq!(t.value().len(), 0);
}

#[test]
fn test_iloidtag_iltag_value_size() {
    let sample: [u64; 4] = [0, 0xFF, 0xFFFF, 0xFFFF_FFFF_FFFF_FFFF];

    let t = ILOIDTag::new();
    assert_eq!(t.value_size(), 1);

    let t = ILOIDTag::with_value(&sample);
    let mut exp = crate::ilint::encoded_size(sample.len() as u64);
    exp += sample
        .iter()
        .map(|x| crate::ilint::encoded_size(*x))
        .sum::<usize>();
    assert_eq!(t.value_size(), exp as u64);
}

#[test]
fn test_iloidtag_iltag_serialize_value() {
    let sample: [u64; 4] = [0, 0xFF, 0xFFFF, 0xFFFF_FFFF_FFFF_FFFF];

    let exp: [u8; 1] = [0];
    let t = ILOIDTag::new();
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize!"),
    }
    assert_eq!(writer.as_slice(), &exp);

    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    sample
        .iter()
        .for_each(|x| write_value_for_testing!(write_ilint, *x, &mut exp));
    let t = ILOIDTag::with_value(&sample);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize!"),
    }
    assert_eq!(writer.as_slice(), exp.as_slice());
}

#[test]
fn test_iloidtag_iltag_deserialize_value() {
    let f = UntouchbleTagFactory::new();
    let sample: [u64; 4] = [0xFF, 0xFFFF, 0xFFFF_FFFF_FFFF_FFFF, 0];

    let exp: [u8; 1] = [0];
    let mut t = ILOIDTag::with_value(&sample);
    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize!"),
    }
    assert_eq!(t.value().len(), 0);

    let mut exp = VecWriter::new();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    sample
        .iter()
        .for_each(|x| write_value_for_testing!(write_ilint, *x, &mut exp));
    let mut t = ILOIDTag::default();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize!"),
    }
    assert_eq!(t.value().as_slice(), &sample);

    // Broken
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize!"),
    }

    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() + 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize!"),
    }

    let mut reader = ByteArrayReader::new(&exp.as_slice()[..14]);
    match t.deserialize_value(&f, exp.as_slice().len() + 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize!"),
    }
}

//=============================================================================
// ILDictTag
//-----------------------------------------------------------------------------

#[test]
fn test_ildicttag_impl() {
    let t = ILDictTag::new();
    assert_eq!(t.id(), IL_DICTIONARY_TAG_ID);
    assert_eq!(t.len(), 0);

    let t = ILDictTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.len(), 0);

    let mut t = ILDictTag::default();
    assert_eq!(t.id(), IL_DICTIONARY_TAG_ID);
    assert_eq!(t.len(), 0);
    match t.insert("test", Box::new(ILNullTag::default())) {
        None => (),
        _ => panic!("Should be empty."),
    }
    let t1 = match t.get("test") {
        Some(t) => t,
        _ => panic!("Value not found!"),
    };
    assert_eq!(t1.id(), IL_NULL_TAG_ID);
}

const SAMPLE_SORTED_KEYS: [&str; 3] = ["a", "b", "c"];

fn create_sample_ildicttag() -> ILDictTag {
    let mut t = ILDictTag::new();
    t.insert("c", Box::new(ILNullTag::default()));
    t.insert("a", Box::new(ILStringTag::with_value(SAMPLE_STRING)));
    t.insert("b", Box::new(ILInt64Tag::with_value(0xFACADA)));
    t
}

#[test]
fn test_ildicttag_iltag_value_size() {
    // Empty
    let t = ILDictTag::new();
    assert_eq!(t.value_size(), 1);

    // With multiple tags
    let t = create_sample_ildicttag();
    let exp = (crate::ilint::encoded_size(t.len() as u64) as u64)
        + t.value()
            .iter()
            .map(|(k, v)| (string_tag_size_from_value(k) + v.size()) as u64)
            .sum::<u64>();
    assert_eq!(t.value_size(), exp);
}

#[test]
fn test_ildicttag_iltag_serialze_value() {
    // Empty
    let exp: [u8; 1] = [0];
    let t = ILDictTag::new();
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.as_slice(), &exp);

    // With multiple tags - check the order
    let t = create_sample_ildicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, t.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(t.get(k).unwrap(), &mut exp);
    }

    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.as_slice(), exp.as_slice());
}

#[test]
fn test_ildicttag_iltag_deserialze_value_non_strict() {
    let f = ILStandardTagFactory::new(false);

    // Empty
    let exp: [u8; 1] = [0];
    let mut t = ILDictTag::new();
    t.insert("x", Box::new(ILNullTag::default()));
    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), 0);

    // With multiple tags - check the order
    let sample = create_sample_ildicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(sample.get(k).unwrap(), &mut exp);
    }

    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), sample.len());
    for k in SAMPLE_SORTED_KEYS {
        let t1 = sample.get(k).unwrap();
        let t2 = t.get(k).unwrap();
        assert!(iltag_are_equal(t1, t2));
    }

    // Error - Missing tag
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Missing pair
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 3 - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Too much data
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() + 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // With multiple tags - check custom
    let mut sample = create_sample_ildicttag();
    sample
        .mut_value()
        .insert(String::from("c"), Box::new(ILUInt8Tag::with_id(1234)));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(sample.get(k).unwrap(), &mut exp);
    }

    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), sample.len());
    for k in SAMPLE_SORTED_KEYS {
        let t1 = sample.get(k).unwrap();
        let t2 = t.get(k).unwrap();
        assert!(iltag_are_equal(t1, t2));
    }
}

#[test]
fn test_ildicttag_iltag_deserialze_value_strict() {
    let f = ILStandardTagFactory::new(true);

    // Empty
    let exp: [u8; 1] = [0];
    let mut t = ILDictTag::new();
    t.mut_value()
        .insert(String::from("x"), Box::new(ILNullTag::default()));
    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), 0);

    // With multiple tags - check the order
    let sample = create_sample_ildicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(sample.get(k).unwrap(), &mut exp);
    }

    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), sample.len());
    for k in SAMPLE_SORTED_KEYS {
        let t1 = sample.get(k).unwrap();
        let t2 = t.get(k).unwrap();
        assert!(iltag_are_equal(t1, t2));
    }

    // Error - Missing tag
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Missing pair
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 3 - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Too much data
    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() + 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // With multiple tags - check custom
    let mut sample = create_sample_ildicttag();
    sample.insert("c", Box::new(ILUInt8Tag::with_id(1234)));
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(sample.get(k).unwrap(), &mut exp);
    }

    let mut t = ILDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }
}

//=============================================================================
// ILDictTag
//-----------------------------------------------------------------------------
#[test]
fn test_ilstrdicttag_impl() {
    let t = ILStrDictTag::new();
    assert_eq!(t.id(), IL_STRING_DICTIONARY_TAG_ID);
    assert_eq!(t.len(), 0);

    let t = ILStrDictTag::with_id(1234);
    assert_eq!(t.id(), 1234);
    assert_eq!(t.len(), 0);

    let mut t = ILStrDictTag::default();
    assert_eq!(t.id(), IL_STRING_DICTIONARY_TAG_ID);
    assert_eq!(t.len(), 0);
    match t.insert("test1", "test2") {
        None => (),
        _ => panic!("Should be empty."),
    }
    let v = match t.get("test1") {
        Some(v) => v,
        _ => panic!("Value not found!"),
    };
    assert_eq!(v, "test2");
}

fn create_sample_ilstrdicttag() -> ILStrDictTag {
    let mut t = ILStrDictTag::new();
    t.insert("c", "c");
    t.insert("a", "A");
    t.insert("b", "b");
    t
}

#[test]
fn test_ilstrdicttag_iltag_value_size() {
    // Empty
    let t = ILStrDictTag::new();
    assert_eq!(t.value_size(), 1);

    // With multiple tags
    let t = create_sample_ilstrdicttag();
    let exp = (crate::ilint::encoded_size(t.len() as u64) as u64)
        + t.value()
            .iter()
            .map(|(k, v)| (string_tag_size_from_value(k) + string_tag_size_from_value(v)) as u64)
            .sum::<u64>();
    assert_eq!(t.value_size(), exp);
}

#[test]
fn test_ilstrdicttag_iltag_serialze_value() {
    // Empty
    let exp: [u8; 1] = [0];
    let t = ILStrDictTag::new();
    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.as_slice(), &exp);

    // With multiple tags - check the order
    let t = create_sample_ilstrdicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, t.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        write_value_for_testing!(serialize_string_tag_from_value, t.get(k).unwrap(), &mut exp);
    }

    let mut writer = VecWriter::default();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(writer.as_slice(), exp.as_slice());
}

#[test]
fn test_ilstrdicttag_iltag_deserialze_value() {
    let f = UntouchbleTagFactory::new();

    // Empty
    let exp: [u8; 1] = [0];
    let mut t = ILStrDictTag::new();
    t.insert("x", "y");
    let mut reader = ByteArrayReader::new(&exp);
    match t.deserialize_value(&f, exp.len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), 0);

    // With multiple tags - check the order
    let sample = create_sample_ilstrdicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        write_value_for_testing!(
            serialize_string_tag_from_value,
            sample.get(k).unwrap(),
            &mut exp
        );
    }

    let mut t = ILStrDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Ok(()) => (),
        _ => panic!("Unable to serialize the tag."),
    }
    assert_eq!(t.len(), sample.len());
    for k in SAMPLE_SORTED_KEYS {
        assert_eq!(sample.get(k).unwrap(), t.get(k).unwrap());
    }

    // Error - Missing tag
    let mut t = ILStrDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Missing pair
    let mut t = ILStrDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() - 3 - 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // Error - Too much data
    let mut t = ILStrDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len() + 1, &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to serialize the tag."),
    }

    // With multiple tags - check the order
    let sample = create_sample_ildicttag();
    let mut exp = VecWriter::default();
    write_value_for_testing!(write_ilint, sample.len() as u64, &mut exp);
    for k in SAMPLE_SORTED_KEYS {
        write_value_for_testing!(serialize_string_tag_from_value, k, &mut exp);
        serialize_tag_for_testing(sample.get(k).unwrap(), &mut exp);
    }
    let mut t = ILStrDictTag::new();
    let mut reader = ByteArrayReader::new(exp.as_slice());
    match t.deserialize_value(&f, exp.as_slice().len(), &mut reader) {
        Err(_) => (),
        _ => panic!("Unable to detect issues."),
    }
}
