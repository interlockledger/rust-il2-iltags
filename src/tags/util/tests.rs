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
use crate::tags::standard::constants::IL_STRING_TAG_ID;
use crate::tags::standard::explicit::ILStringTag;
use crate::tags::{ILDefaultWithIdTagCreator, ILRawTag};

#[test]
fn test_iltag_are_equal() {
    let sample = "abcde";
    let a = ILStringTag::with_value(sample);
    let b = ILRawTag::with_value(IL_STRING_TAG_ID, sample.as_bytes());
    let c = ILRawTag::with_value(1234, sample.as_bytes());
    let d = ILStringTag::with_value("abcd");
    let e = ILStringTag::with_value("bcdef");

    assert!(iltag_are_equal(&a, &a));
    assert!(iltag_are_equal(&a, &b));

    assert!(!iltag_are_equal(&a, &c));
    assert!(!iltag_are_equal(&a, &d));
    assert!(!iltag_are_equal(&a, &e));
}

#[test]
fn test_iltag_clone_with_factory() {
    let mut factory = ILStandardTagFactory::new(true);

    factory.engine().register(
        1234 as u64,
        Box::new(ILDefaultWithIdTagCreator::<ILStringTag>::new()),
    );
    let sample = "abcde";

    // Cloning a string
    let a = ILStringTag::with_value(sample);
    let clone = match iltag_clone_with_factory(&factory, &a) {
        Ok(c) => c,
        _ => panic!("Clone failed!"),
    };
    assert!(iltag_are_equal(&a, clone.as_ref()));

    // Cloning a string but with an unknown id
    let a = ILStringTag::with_id_value(1234, sample);
    let clone = match iltag_clone_with_factory(&factory, &a) {
        Ok(c) => c,
        _ => panic!("Clone failed!"),
    };
    // This will result in a ILRawTag instead of ILStringTag.
    assert_eq!(
        clone.as_ref().as_any().type_id(),
        std::any::TypeId::of::<ILStringTag>()
    );
    assert!(iltag_are_equal(&a, clone.as_ref()));

    // Cloning a string but with an unknown id
    let a = ILStringTag::with_id_value(1235, sample);
    match iltag_clone_with_factory(&factory, &a) {
        Err(_) => (),
        _ => panic!("Clone should have failed."),
    };
}

#[test]
fn test_iltag_clone() {
    let sample = "abcde";

    // Cloning a string
    let a = ILStringTag::with_value(sample);
    let clone = match iltag_clone(&a) {
        Ok(c) => c,
        _ => panic!("Clone failed!"),
    };
    assert!(iltag_are_equal(&a, clone.as_ref()));

    // Cloning a string but with an unknown id
    let a = ILStringTag::with_id_value(1234, sample);
    let clone = match iltag_clone(&a) {
        Ok(c) => c,
        _ => panic!("Clone failed!"),
    };
    // This will result in a ILRawTag instead of ILStringTag.
    assert_eq!(
        clone.as_ref().as_any().type_id(),
        std::any::TypeId::of::<ILRawTag>()
    );
    assert!(iltag_are_equal(&a, clone.as_ref()));
}

#[test]
fn test_limited_reader_ensure_empty() {
    let sample: [u8; 1] = [0];
    let mut reader = ByteArrayReader::new(&sample);
    let lreader = LimitedReader::new(&mut reader, 1);

    match limited_reader_ensure_empty(&lreader, ErrorKind::TagTooLarge) {
        Err(ErrorKind::TagTooLarge) => (),
        _ => panic!("Error expected."),
    }
    match limited_reader_ensure_empty(&lreader, ErrorKind::CorruptedData) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!("Error expected."),
    }

    let lreader = LimitedReader::new(&mut reader, 0);
    match limited_reader_ensure_empty(&lreader, ErrorKind::CorruptedData) {
        Ok(()) => (),
        _ => panic!("Error not expected."),
    }
}

//=============================================================================
// UntouchbleTagFactory
//-----------------------------------------------------------------------------

#[test]
#[should_panic(expected = "UntouchbleTagFactory touched.")]
fn test_untouchbletagfactory_create_tag() {
    let f = UntouchbleTagFactory::new();

    f.create_tag(0);
}

#[test]
#[should_panic(expected = "UntouchbleTagFactory touched.")]
#[allow(unused_must_use)]
fn test_untouchbletagfactory_deserialize() {
    let f = UntouchbleTagFactory::new();
    let empty: [u8; 0] = [];

    let mut reader = ByteArrayReader::new(&empty);
    f.deserialize(&mut reader);
}

#[test]
#[should_panic(expected = "UntouchbleTagFactory touched.")]
#[allow(unused_must_use)]
fn test_untouchbletagfactory_deserialize_into() {
    let f = UntouchbleTagFactory::new();
    let empty: [u8; 0] = [];

    let mut reader = ByteArrayReader::new(&empty);
    let mut tag = ILRawTag::new(1234);
    f.deserialize_into(&mut reader, &mut tag);
}
