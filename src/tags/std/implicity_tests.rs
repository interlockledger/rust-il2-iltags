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
use super::implicity::*;
use super::*;
use crate::io::{ByteArrayReader, VecReader, VecWriter};
use crate::tags::tests::UntouchbleTagFactory;

//=============================================================================
// Common tests
//-----------------------------------------------------------------------------
/// Test constructors
macro_rules! test_new_with_id_default_func_impl {
    ($tag_type: ty, $default_id: expr) => {
        let t = <$tag_type>::new();
        assert_eq!(
            t.id(),
            $default_id,
            "new() failed because it does not match the default id."
        );

        let t = <$tag_type>::default();
        assert_eq!(
            t.id(),
            $default_id,
            "default() should be equivalent to new() failed because it does not match the default id."
        );

        let t = <$tag_type>::with_id(123);
        assert_eq!(t.id(), 123, "with_id() is not setting the custom ID correctly.");
    };
}

macro_rules! test_simple_value_tag_struct_impl_func_impl {
    ($tag_type: ty, $default_id: expr, $value_type: ty, $sample_value: expr) => {
        let sample_value: $value_type = $sample_value as $value_type;
        let def_value: $value_type = <$value_type>::default();

        let t = <$tag_type>::new();
        assert_eq!(
            t.id(),
            $default_id,
            "new() failed because it does not match the default id."
        );
        assert_eq!(t.value(), def_value, "new() should initialize value with default.");

        let t = <$tag_type>::default();
        assert_eq!(
            t.id(),
            $default_id,
            "default() should be equivalent to new() failed because it does not match the default id."
        );
        assert_eq!(t.value(), def_value, "default() should initialize value with default.");

        let t = <$tag_type>::with_id(123);
        assert_eq!(t.id(), 123, "with_id() is not setting the custom ID correctly.");
        assert_eq!(t.value(), def_value, "with_id() should initialize value with default.");

        let t = <$tag_type>::with_value(sample_value);
        assert_eq!(t.id(), $default_id, "with_value() should use the default id");
        assert_eq!(t.value(), sample_value, "with_value() should initialize the value with {}.", sample_value);

        let t = <$tag_type>::with_id_value(123, sample_value);
        assert_eq!(t.id(), 123, "with_id_value() is not setting the custom ID correctly.");
        assert_eq!(t.value(), sample_value, "with_id_value() should initialize the value with {}.", sample_value);

        let mut t = <$tag_type>::new();
        assert_eq!(t.value(), def_value, "Failed to start with default.");
        t.set_value(sample_value);
        assert_eq!(t.id(), $default_id, "Set value should not change the id.");
        assert_eq!(t.value(), sample_value, "set_value() failed.");
    };
}

macro_rules! test_simple_value_iltag_value_size_impl {
    ($tag_type: ty, $value_size: expr) => {
        let t = <$tag_type>::default();
        assert_eq!(t.value_size(), $value_size as u64, "Bad value size.");
    };
}

macro_rules! test_simple_value_iltag_serialize_impl {
    ($tag_type: ty, $value_type: ty, $value_size: expr, $samples: expr) => {
        let exp_size: u64 = $value_size;
        let sample: [$value_type; 2] = $samples;

        // Serialize
        for s in &sample {
            let t = <$tag_type>::with_value(*s);
            let mut writer = VecWriter::new();
            match t.serialize_value(&mut writer) {
                Ok(()) => (),
                _ => panic!("serialize_value failed."),
            }
            assert_eq!(writer.as_slice().len() as u64, exp_size);
            assert_eq!(writer.as_slice(), (*s).to_be_bytes());
        }

        let t = <$tag_type>::default();
        let mut writer = VecWriter::new();
        writer.set_read_only(true);
        match t.serialize_value(&mut writer) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!(),
        }
    };
}

macro_rules! test_simple_value_iltag_deserialize_impl {
    ($tag_type: ty, $value_type: ty, $value_size: expr, $samples: expr) => {
        let exp_size: usize = $value_size;
        let factory = UntouchbleTagFactory {};
        const SAMPLE: [$value_type; 2] = $samples;

        // Unserialize
        for s in &SAMPLE {
            let mut t = <$tag_type>::new();
            let mut reader = VecReader::new(&((*s).to_be_bytes()));
            match t.deserialize_value(&factory, exp_size, &mut reader) {
                Ok(()) => (),
                _ => panic!("deserialize_value() failed."),
            }
            assert_eq!(t.value(), *s);
            match t.deserialize_value(&factory, exp_size, &mut reader) {
                Err(ErrorKind::IOError(_)) => (),
                _ => panic!("deserialize_value() should fail due to IO error."),
            }
        }

        let tmp: [u8; 16] = [0; 16];
        let mut t = <$tag_type>::new();
        let mut reader = ByteArrayReader::new(&tmp);
        match t.deserialize_value(&factory, exp_size + 1, &mut reader) {
            Err(ErrorKind::CorruptedData) => (),
            _ => panic!("deserialize_value() should fail because size tag_size is wrong."),
        }
    };
}

//=============================================================================
// ILNullTag
//-----------------------------------------------------------------------------

#[test]
fn test_ilnulltag_new() {
    test_new_with_id_default_func_impl!(ILNullTag, IL_NULL_TAG_ID);
}

#[test]
fn test_ilnulltag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILNullTag, 0);
}

#[test]
fn test_ilnulltag_iltag_serialize() {
    // Serialize
    let t = ILNullTag::new();
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(writer.as_slice().len() as u64, 0);
}

#[test]
fn test_ilnulltag_iltag_deserialize() {
    let factory = UntouchbleTagFactory {};
    const SAMPLE: [u8; 0] = [0; 0];

    // Unserialize
    let mut t = ILNullTag::new();
    let mut reader = ByteArrayReader::new(&SAMPLE);
    match t.deserialize_value(&factory, 0, &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    match t.deserialize_value(&factory, 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!(),
    }
}

//=============================================================================
// ILBoolTag
//-----------------------------------------------------------------------------

#[test]
fn test_ilbooltag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILBoolTag, IL_BOOL_TAG_ID, bool, true);
}

#[test]
fn test_ilbooltag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILBoolTag, 1);
}

#[test]
fn test_ilbooltag_iltag_serialize() {
    // Size
    let exp_size: u64 = 1;

    // Serialize
    let t = ILBoolTag::with_value(false);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(writer.as_slice().len() as u64, exp_size);
    assert_eq!(writer.as_slice()[0], 0 as u8);

    let t = ILBoolTag::with_value(true);
    let mut writer = VecWriter::new();
    match t.serialize_value(&mut writer) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(writer.as_slice().len() as u64, exp_size);
    assert_eq!(writer.as_slice()[0], 1 as u8);

    writer.set_read_only(true);
    match t.serialize_value(&mut writer) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_ilbooltag_iltag_deserialize() {
    let factory = UntouchbleTagFactory {};
    const SAMPLE: [u8; 3] = [1, 0, 2];

    // Unserialize
    let mut t = ILBoolTag::new();
    let mut reader = ByteArrayReader::new(&SAMPLE);
    match t.deserialize_value(&factory, 1, &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(t.value(), true);

    let mut t = ILBoolTag::new();
    match t.deserialize_value(&factory, 1, &mut reader) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(t.value(), false);

    let mut t = ILBoolTag::new();
    match t.deserialize_value(&factory, 1, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!(),
    }

    match t.deserialize_value(&factory, 2, &mut reader) {
        Err(ErrorKind::CorruptedData) => (),
        _ => panic!(),
    }
}

//=============================================================================
// ILInt8Tag
//-----------------------------------------------------------------------------

#[test]
fn test_ilint8tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILInt8Tag, IL_INT8_TAG_ID, i8, 126);
}

#[test]
fn test_ilint8tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILInt8Tag, 1);
}

#[test]
fn test_ilint8tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILInt8Tag, i8, 1, [-123, 123]);
}

#[test]
fn test_ilint8tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILInt8Tag, i8, 1, [-123, 123]);
}

//=============================================================================
// ILUInt8Tag
//-----------------------------------------------------------------------------
#[test]
fn test_iluint8tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILUInt8Tag, IL_UINT8_TAG_ID, u8, 126);
}

#[test]
fn test_iluint8tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILUInt8Tag, 1);
}

#[test]
fn test_iluint8tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILUInt8Tag, u8, 1, [0x12, 0xFE]);
}

#[test]
fn test_iulint8tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILUInt8Tag, u8, 1, [0x12, 0xFE]);
}

//=============================================================================
// ILInt16Tag
//-----------------------------------------------------------------------------
#[test]
fn test_ilint16tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILInt16Tag, IL_INT16_TAG_ID, i16, 126);
}

#[test]
fn test_ilint16tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILInt16Tag, 2);
}

#[test]
fn test_ilint16tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILInt16Tag, i16, 2, [-0x1234, 0x1234]);
}

#[test]
fn test_ilint16tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILInt16Tag, i16, 2, [-0x1234, 0x1234]);
}

//=============================================================================
// ILUInt16Tag
//-----------------------------------------------------------------------------
#[test]
fn test_iluint16tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILUInt16Tag, IL_UINT16_TAG_ID, u16, 126);
}

#[test]
fn test_iluint16tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILUInt16Tag, 2);
}

#[test]
fn test_iluint16tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILUInt16Tag, u16, 2, [0x1234, 0xFEDC]);
}

#[test]
fn test_iulint16tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILUInt16Tag, u16, 2, [0x1234, 0xFEDC]);
}
//=============================================================================
// ILInt32Tag
//-----------------------------------------------------------------------------

#[test]
fn test_ilint32tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILInt32Tag, IL_INT32_TAG_ID, i32, 126);
}

#[test]
fn test_ilint32tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILInt32Tag, 4);
}

#[test]
fn test_ilint32tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILInt32Tag, i32, 4, [-0x1234_5678, 0x1234_5678]);
}

#[test]
fn test_ilint32tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILInt32Tag, i32, 4, [-0x1234_5678, 0x1234_5678]);
}

//=============================================================================
// ILUInt32Tag
//-----------------------------------------------------------------------------
#[test]
fn test_iluint32tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILUInt32Tag, IL_UINT32_TAG_ID, u32, 126);
}

#[test]
fn test_iluint32tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILUInt32Tag, 4);
}

#[test]
fn test_iluint32tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILUInt32Tag, u32, 4, [0x1234_5678, 0xFEDC_BA98]);
}

#[test]
fn test_iulint32tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILUInt32Tag, u32, 4, [0x1234_5678, 0xFEDC_BA98]);
}

//=============================================================================
// ILInt64Tag
//-----------------------------------------------------------------------------

#[test]
fn test_ilint64tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILInt64Tag, IL_INT64_TAG_ID, i64, 126);
}

#[test]
fn test_ilint64tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILInt64Tag, 8);
}

#[test]
fn test_ilint64tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(
        ILInt64Tag,
        i64,
        8,
        [-0x1234_5678_90AB_CDEF, 0x1234_5678_90AB_CDEF]
    );
}

#[test]
fn test_ilint64tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(
        ILInt64Tag,
        i64,
        8,
        [-0x1234_5678_90AB_CDEF, 0x1234_5678_90AB_CDEF]
    );
}

//=============================================================================
// ILUInt64Tag
//-----------------------------------------------------------------------------
#[test]
fn test_iluint64tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILUInt64Tag, IL_UINT64_TAG_ID, u64, 126);
}

#[test]
fn test_iluint64tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILUInt64Tag, 8);
}

#[test]
fn test_iluint64tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(
        ILUInt64Tag,
        u64,
        8,
        [0x1234_5678_90AB_CDEF, 0xFEDC_BA98_7654_3210]
    );
}

#[test]
fn test_iulint64tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(
        ILUInt64Tag,
        u64,
        8,
        [0x1234_5678_90AB_CDEF, 0xFEDC_BA98_7654_3210]
    );
}

//=============================================================================
// ILILInt64Tag
//-----------------------------------------------------------------------------
#[test]
fn test_ililint64tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILILInt64Tag, IL_ILINT_TAG_ID, u64, 1234);
}

#[test]
fn test_ililint64tag_iltag_size() {
    // TODO
}

#[test]
fn test_ililint64tag_iltag_serialize() {
    // TODO
}

#[test]
fn test_ililint64tag_iltag_deserialize() {
    // TODO
}

//=============================================================================
// ILBin32Tag
//-----------------------------------------------------------------------------
#[test]
fn test_ilbin32tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILBin32Tag, IL_BIN32_TAG_ID, f32, 1.2345);
}

#[test]
fn test_ilbin32tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILBin32Tag, 4);
}

#[test]
fn test_ilbin32tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILBin32Tag, f32, 4, [-1.2345678, 9.87654]);
}

#[test]
fn test_ilbin32tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILBin32Tag, f32, 4, [-1.2345678, 9.87654]);
}

//=============================================================================
// ILBin64Tag
//-----------------------------------------------------------------------------
#[test]
fn test_ilbin64tag_new() {
    test_simple_value_tag_struct_impl_func_impl!(ILBin64Tag, IL_BIN64_TAG_ID, f64, 1.2345);
}

#[test]
fn test_ilbin64tag_iltag_size() {
    test_simple_value_iltag_value_size_impl!(ILBin64Tag, 8);
}

#[test]
fn test_ilbin64tag_iltag_serialize() {
    test_simple_value_iltag_serialize_impl!(ILBin64Tag, f64, 8, [-1.2345678, 9.87654]);
}

#[test]
fn test_ilbin64tag_iltag_deserialize() {
    test_simple_value_iltag_deserialize_impl!(ILBin64Tag, f64, 8, [-1.2345678, 9.87654]);
}
