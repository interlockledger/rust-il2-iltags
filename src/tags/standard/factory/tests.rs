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
use crate::io::array::*;
use crate::tags::ErrorKind;
use crate::tags::ILRawTag;

macro_rules! test_tag_type {
    ($tag: expr, $tag_id: expr, $tag_type: ty) => {
        assert_eq!($tag.id(), $tag_id);
        assert_eq!(std::any::TypeId::of::<$tag_type>(), $tag.as_any().type_id());
    };
}

macro_rules! test_engine_created_tag {
    ($engine: expr, $tag_id: expr, $tag_type: ty) => {
        let t = match $engine.create_tag($tag_id) {
            Some(boxed) => boxed,
            None => panic!("Tag expected tag {:?} not created.", $tag_id),
        };
        test_tag_type!(t, $tag_id, $tag_type);
    };
}

macro_rules! test_engine_created_tag_expect_none {
    ($engine: expr, $tag_id: expr) => {
        match $engine.create_tag($tag_id) {
            None => (),
            Some(_) => panic!("This should be None for tag id {:?}", $tag_id),
        };
    };
}

macro_rules! test_engine_standard_registration {
    ($engine: expr) => {
        // Implicit
        test_engine_created_tag!($engine, IL_NULL_TAG_ID, ILNullTag);
        test_engine_created_tag!($engine, IL_BOOL_TAG_ID, ILBoolTag);
        test_engine_created_tag!($engine, IL_INT8_TAG_ID, ILInt8Tag);
        test_engine_created_tag!($engine, IL_UINT8_TAG_ID, ILUInt8Tag);
        test_engine_created_tag!($engine, IL_INT16_TAG_ID, ILInt16Tag);
        test_engine_created_tag!($engine, IL_UINT16_TAG_ID, ILUInt16Tag);
        test_engine_created_tag!($engine, IL_INT32_TAG_ID, ILInt32Tag);
        test_engine_created_tag!($engine, IL_UINT32_TAG_ID, ILUInt32Tag);
        test_engine_created_tag!($engine, IL_INT64_TAG_ID, ILInt64Tag);
        test_engine_created_tag!($engine, IL_UINT64_TAG_ID, ILUInt64Tag);
        test_engine_created_tag!($engine, IL_ILINT_TAG_ID, ILILInt64Tag);
        test_engine_created_tag!($engine, IL_BIN32_TAG_ID, ILBin32Tag);
        test_engine_created_tag!($engine, IL_BIN64_TAG_ID, ILBin64Tag);
        test_engine_created_tag!($engine, IL_BIN128_TAG_ID, ILBin128Tag);
        test_engine_created_tag!($engine, IL_SIGNED_ILINT_TAG_ID, ILSignedILInt64Tag);

        // Explict
        test_engine_created_tag!($engine, IL_BYTES_TAG_ID, ILByteArrayTag);
        test_engine_created_tag!($engine, IL_STRING_TAG_ID, ILStringTag);
        test_engine_created_tag!($engine, IL_BINT_TAG_ID, ILBigIntTag);
        test_engine_created_tag!($engine, IL_BDEC_TAG_ID, ILBigDecTag);
        test_engine_created_tag!($engine, IL_ILINTARRAY_TAG_ID, ILILIntArrayTag);
        test_engine_created_tag!($engine, IL_ILTAGARRAY_TAG_ID, ILTagArrayTag);
        test_engine_created_tag!($engine, IL_ILTAGSEQ_TAG_ID, ILTagSeqTag);
        test_engine_created_tag!($engine, IL_RANGE_TAG_ID, ILRangeTag);
        test_engine_created_tag!($engine, IL_VERSION_TAG_ID, ILVersionTag);
        test_engine_created_tag!($engine, IL_OID_TAG_ID, ILOIDTag);
        test_engine_created_tag!($engine, IL_DICTIONARY_TAG_ID, ILDictTag);
        test_engine_created_tag!($engine, IL_STRING_DICTIONARY_TAG_ID, ILStrDictTag);
    };
}

macro_rules! test_engine_standard_registration_non_strict {
    ($engine: expr) => {
        test_engine_created_tag_expect_none!($engine, 15);
        test_engine_created_tag!($engine, 26, ILRawTag);
        test_engine_created_tag!($engine, 27, ILRawTag);
        test_engine_created_tag!($engine, 28, ILRawTag);
        test_engine_created_tag!($engine, 29, ILRawTag);
        test_engine_created_tag!($engine, 32, ILRawTag);
        test_engine_created_tag!($engine, 31313, ILRawTag);
    };
}

macro_rules! test_engine_standard_registration_strict {
    ($engine: expr) => {
        test_engine_created_tag_expect_none!($engine, 15);
        test_engine_created_tag_expect_none!($engine, 26);
        test_engine_created_tag_expect_none!($engine, 27);
        test_engine_created_tag_expect_none!($engine, 28);
        test_engine_created_tag_expect_none!($engine, 29);
        test_engine_created_tag_expect_none!($engine, 32);
        test_engine_created_tag_expect_none!($engine, 31313);
    };
}

#[test]
fn test_create_std_engine_non_strict() {
    let e = create_std_engine(false);
    assert!(!e.strict());
    test_engine_standard_registration!(e);
    test_engine_standard_registration_non_strict!(e);
}

#[test]
fn test_create_std_engine_strict() {
    let e = create_std_engine(true);
    assert!(e.strict());
    test_engine_standard_registration!(e);
    test_engine_standard_registration_strict!(e);
}

//=============================================================================
// ILStandardTagFactory
//-----------------------------------------------------------------------------
#[test]
fn test_ilstandardtagfactory_struct() {
    let mut f = ILStandardTagFactory::new(false);
    assert!(!f.engine().strict());
    test_engine_standard_registration!(f.engine());
    test_engine_standard_registration_non_strict!(f.engine());

    let mut f = ILStandardTagFactory::new(true);
    assert!(f.engine().strict());
    test_engine_standard_registration!(f.engine());
    test_engine_standard_registration_strict!(f.engine());
}

#[test]
fn test_ilstandardtagfactory_iltagfactory_create_tag_non_strict() {
    let mut f = ILStandardTagFactory::new(false);
    assert!(!f.engine().strict());
    test_engine_standard_registration!(f);
    test_engine_standard_registration_non_strict!(f);
}

#[test]
fn test_ilstandardtagfactory_iltagfactory_create_tag_strict() {
    let mut f = ILStandardTagFactory::new(true);
    assert!(f.engine().strict());
    test_engine_standard_registration!(f);
    test_engine_standard_registration_strict!(f);
}

fn create_serialized(tag: &dyn ILTag) -> VecReader {
    let mut writer = VecWriter::new();
    match tag.serialize(&mut writer) {
        Ok(()) => VecReader::new(writer.as_slice()),
        _ => panic!("Unable to serialize the tag."),
    }
}

macro_rules! test_deserialize_tag {
    ($factory: expr, $tag: expr, $tag_type: ty) => {
        let mut reader = create_serialized(&$tag);
        let t = match $factory.deserialize(&mut reader) {
            Ok(t) => t,
            _ => panic!("Unable to read the tag."),
        };
        test_tag_type!(t, $tag.id(), $tag_type);
        assert_eq!($tag.value_size(), t.value_size());
        match reader.read() {
            Err(_) => (),
            _ => panic!("All bytes should have been consumed."),
        };
    };
}

macro_rules! test_deserialize_tag_expect_none {
    ($factory: expr, $tag: expr) => {
        let mut reader = create_serialized(&$tag);
        match $factory.deserialize(&mut reader) {
            Err(ErrorKind::UnknownTag) => (),
            _ => panic!("Unable to read the tag."),
        };
    };
}

#[test]
fn test_ilstandardtagfactory_iltagfactory_deserialize_non_strict() {
    let f = ILStandardTagFactory::new(false);

    // Implicit tags
    test_deserialize_tag!(f, ILNullTag::new(), ILNullTag);
    test_deserialize_tag!(f, ILInt16Tag::new(), ILInt16Tag);
    test_deserialize_tag!(f, ILILInt64Tag::with_value(127), ILILInt64Tag);
    test_deserialize_tag!(f, ILILInt64Tag::with_value(12345), ILILInt64Tag);
    test_deserialize_tag!(f, ILSignedILInt64Tag::with_value(1), ILSignedILInt64Tag);
    test_deserialize_tag!(f, ILSignedILInt64Tag::with_value(-1), ILSignedILInt64Tag);
    test_deserialize_tag!(
        f,
        ILSignedILInt64Tag::with_value(9_223_372_036_854_775_807),
        ILSignedILInt64Tag
    );
    test_deserialize_tag!(
        f,
        ILSignedILInt64Tag::with_value(-9_223_372_036_854_775_808),
        ILSignedILInt64Tag
    );

    // Invalid implicit tag
    test_deserialize_tag_expect_none!(f, ILNullTag::with_id(15));

    // Explicit tag
    let mut t = ILByteArrayTag::new();
    t.mut_value().extend_from_slice(&[0; 32]);
    test_deserialize_tag!(f, t, ILByteArrayTag);

    // Deserialization of unknown tags
    test_deserialize_tag!(f, ILILInt64Tag::with_id_value(12345, 12345), ILRawTag);
    test_deserialize_tag!(f, ILInt16Tag::with_id_value(123123323, 12345), ILRawTag);

    // Simulate an error with an incomplete tag - This tag was expected to have 3 bytes
    let mut reader = VecReader::new(&[4, 2]);
    match f.deserialize(&mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("IO Error expected"),
    }
}

#[test]
fn test_ilstandardtagfactory_iltagfactory_deserialize_strict() {
    let f = ILStandardTagFactory::new(true);

    // Implicit tags
    test_deserialize_tag!(f, ILNullTag::new(), ILNullTag);
    test_deserialize_tag!(f, ILInt16Tag::new(), ILInt16Tag);
    test_deserialize_tag!(f, ILILInt64Tag::with_value(127), ILILInt64Tag);
    test_deserialize_tag!(f, ILILInt64Tag::with_value(12345), ILILInt64Tag);
    test_deserialize_tag!(f, ILSignedILInt64Tag::with_value(1), ILSignedILInt64Tag);
    test_deserialize_tag!(f, ILSignedILInt64Tag::with_value(-1), ILSignedILInt64Tag);
    test_deserialize_tag!(
        f,
        ILSignedILInt64Tag::with_value(9_223_372_036_854_775_807),
        ILSignedILInt64Tag
    );
    test_deserialize_tag!(
        f,
        ILSignedILInt64Tag::with_value(-9_223_372_036_854_775_808),
        ILSignedILInt64Tag
    );

    // Invalid implicit tag
    test_deserialize_tag_expect_none!(f, ILNullTag::with_id(15));

    // Explicit tag
    let mut t = ILByteArrayTag::new();
    t.mut_value().extend_from_slice(&[0; 32]);
    test_deserialize_tag!(f, t, ILByteArrayTag);

    // Deserialization of unknown tags
    test_deserialize_tag_expect_none!(f, ILILInt64Tag::with_id_value(12345, 12345));
    test_deserialize_tag_expect_none!(f, ILInt16Tag::with_id_value(123123323, 12345));

    // Simulate an error with an incomplete tag - This tag was expected to have 3 bytes
    let mut reader = VecReader::new(&[4, 2]);
    match f.deserialize(&mut reader) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("IO Error expected"),
    }
}
