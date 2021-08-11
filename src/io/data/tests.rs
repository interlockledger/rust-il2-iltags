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
use super::test_samples::*;
use super::*;
use crate::io::array::{ByteArrayReader, VecWriter};

macro_rules! test_int_data_reader_t {
    ($type: ty, $sample : expr, $expected: expr, $func: ident) => {{
        let mut reader = ByteArrayReader::new($sample);
        let r: &mut dyn Reader = &mut reader;

        for exp_val in $expected {
            match $func(r) {
                Ok(v) => assert_eq!(v, *exp_val as $type),
                _ => panic!(),
            }
        }
        assert!($func(r).is_err());
    }};
}

#[test]
fn test_int_data_reader_u8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_reader_t!(u8, &sample, &exp, read_u8);
}

#[test]
fn test_int_data_reader_i8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i8; 8] = [0x01, 0x23, 0x45, 0x67, -0x77, -0x55, -0x33, -0x11];
    test_int_data_reader_t!(i8, &sample, &exp, read_i8);
}

#[test]
fn test_int_data_reader_u16() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u16; 4] = [0x0123, 0x4567, 0x89AB, 0xCDEF];
    test_int_data_reader_t!(u16, &sample, &exp, read_u16);
}

#[test]
fn test_int_data_reader_i16() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i16; 4] = [0x0123, 0x4567, -0x7655, -0x3211];
    test_int_data_reader_t!(i16, &sample, &exp, read_i16);
}

#[test]
fn test_int_data_reader_u32() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u32; 2] = [0x01234567, 0x89ABCDEF];
    test_int_data_reader_t!(u32, &sample, &exp, read_u32);
}

#[test]
fn test_int_data_reader_i32() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i32; 2] = [0x01234567, -0x76543211];
    test_int_data_reader_t!(i32, &sample, &exp, read_i32);
}

#[test]
fn test_int_data_reader_u64() {
    let sample: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let exp: [u64; 2] = [0x0123456789ABCDEF, 0xFEDCBA9876543210];
    test_int_data_reader_t!(u64, &sample, &exp, read_u64);
}

#[test]
fn test_int_data_reader_i64() {
    let sample: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let exp: [i64; 2] = [0x0123456789ABCDEF, -0x123456789ABCDF0];
    test_int_data_reader_t!(i64, &sample, &exp, read_i64);
}

#[test]
fn test_ilint_data_reader() {
    let sample: [u8; 46] = [
        0xF7, 0xF8, 0x00, 0xF9, 0x01, 0x23, 0xFA, 0x01, 0x23, 0x45, 0xFB, 0x01, 0x23, 0x45, 0x67,
        0xFC, 0x01, 0x23, 0x45, 0x67, 0x89, 0xFD, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xFE, 0x01,
        0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07,
        0xFF,
    ];
    let expected: [u64; 9] = [
        0xF7,
        0xF8,
        0x021B,
        0x01243D,
        0x0123465F,
        0x0123456881,
        0x012345678AA3,
        0x123456789ACC5,
        0xFFFFFFFFFFFFFFFF,
    ];

    let mut reader = ByteArrayReader::new(&sample);
    let r: &mut dyn Reader = &mut reader;

    for exp in expected.iter() {
        match read_ilint(r) {
            Ok(v) => assert_eq!(*exp, v),
            _ => panic!(),
        };
    }
    assert!(read_ilint(r).is_err());
}

#[test]
fn test_float_data_reader_f32() {
    let sample: [u8; 5] = [0x40, 0x49, 0x0f, 0xdb, 0x01];
    let mut reader = ByteArrayReader::new(&sample);
    let r: &mut dyn Reader = &mut reader;

    match read_f32(r) {
        Ok(v) => assert_eq!(3.14159274101257324, v),
        _ => panic!(),
    };
    assert!(read_f32(r).is_err());
}

#[test]
fn test_float_data_reader_f64() {
    let sample: [u8; 9] = [0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18, 0x01];
    let mut reader = ByteArrayReader::new(&sample);
    let r: &mut dyn Reader = &mut reader;

    match read_f64(r) {
        Ok(v) => assert_eq!(3.1415926535897932384626433, v),
        _ => panic!(),
    };
    assert!(read_f64(r).is_err());
}

#[test]
fn test_string_data_reader() {
    // This frase is attibuted to Machado de Assis. It was
    // choosen because it contains Latin characters that
    // result in a multi-byte characters in UTF-8.
    let sample: [u8; 30] = [
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61, 0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20,
        0x73, 0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73,
    ];
    let expected = "Lágrimas não são argumentos";
    let mut reader = ByteArrayReader::new(&sample);
    let r: &mut dyn Reader = &mut reader;

    match read_string(r, 30) {
        Ok(v) => assert_eq!(expected, v),
        _ => panic!(),
    };
    assert!(read_string(r, 0).is_ok());
    assert!(read_string(r, 1).is_err());
}

#[test]
fn test_data_reader() {
    let mut reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let dr: &mut dyn Reader = &mut reader;

    match read_u8(dr) as Result<u8> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_00_U8, v),
        _ => panic!(),
    }
    match read_u16(dr) as Result<u16> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_01_U16, v),
        _ => panic!(),
    }
    match read_u32(dr) as Result<u32> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_02_U32, v),
        _ => panic!(),
    }
    match read_u64(dr) as Result<u64> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_03_U64, v),
        _ => panic!(),
    }
    match read_i8(dr) as Result<i8> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_04_I8, v),
        _ => panic!(),
    }
    match read_i16(dr) as Result<i16> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_05_I16, v),
        _ => panic!(),
    }
    match read_i32(dr) as Result<i32> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_06_I32, v),
        _ => panic!(),
    }
    match read_i64(dr) as Result<i64> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_07_I64, v),
        _ => panic!(),
    }
    match read_f32(dr) as Result<f32> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_08_F32, v),
        _ => panic!(),
    }
    match read_f64(dr) as Result<f64> {
        Ok(v) => assert_eq!(SAMPLE_VALUES_09_F64, v),
        _ => panic!(),
    }
    match read_string(dr, SAMPLE_VALUES_10_STR_LEN) {
        Ok(v) => assert_eq!(SAMPLE_VALUES_10_STR, v),
        _ => panic!(),
    }
    match read_ilint(dr) {
        Ok(v) => assert_eq!(v, SAMPLE_VALUES_11_ILINT),
        Err(_) => panic!(),
    }

    match read_u8(dr) as Result<u8> {
        Ok(_) => panic!(),
        _ => (),
    }
}

macro_rules! test_int_data_writer_t {
    ($type: ty, $sample : expr, $expected: expr, $func: ident) => {{
        let mut writer = VecWriter::new();
        let w: &mut dyn Writer = &mut writer;

        for val in $sample {
            match $func(*val, w) {
                Ok(_) => (),
                _ => panic!(),
            }
        }
        assert_eq!($expected.len(), writer.as_slice().len());
        assert_eq!($expected, writer.as_slice());
    }};
}

#[test]
fn test_int_data_writer_u8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u8, &sample, &exp, write_u8);
}

#[test]
fn test_int_data_writer_i8() {
    let sample: [i8; 8] = [0x01, 0x23, 0x45, 0x67, -0x77, -0x55, -0x33, -0x11];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i8, &sample, &exp, write_i8);
}

#[test]
fn test_int_data_writer_u16() {
    let sample: [u16; 4] = [0x0123, 0x4567, 0x89AB, 0xCDEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u16, &sample, &exp, write_u16);
}

#[test]
fn test_int_data_writer_i16() {
    let sample: [i16; 4] = [0x0123, 0x4567, -0x7655, -0x3211];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i16, &sample, &exp, write_i16);
}

#[test]
fn test_int_data_writer_u32() {
    let sample: [u32; 2] = [0x01234567, 0x89ABCDEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u32, &sample, &exp, write_u32);
}

#[test]
fn test_int_data_writer_i32() {
    let sample: [i32; 2] = [0x01234567, -0x76543211];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i32, &sample, &exp, write_i32);
}

#[test]
fn test_int_data_writer_u64() {
    let sample: [u64; 2] = [0x0123456789ABCDEF, 0x0123456789ABCDEF];
    let exp: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF,
    ];
    test_int_data_writer_t!(u64, &sample, &exp, write_u64);
}

#[test]
fn test_int_data_writer_i64() {
    let sample: [i64; 2] = [0x0123456789ABCDEF, -0x7EDCBA9876543211];
    let exp: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x81, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF,
    ];
    test_int_data_writer_t!(i64, &sample, &exp, write_i64);
}

#[test]
fn test_float_data_writer_f32() {
    let mut writer = VecWriter::new();
    let w: &mut dyn Writer = &mut writer;

    let exp: [u8; 8] = [0x40, 0x49, 0x0f, 0xdb, 0x40, 0x49, 0x0f, 0xdb];

    match write_f32(3.14159274101257324 as f32, w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_f32(3.14159274101257324 as f32, w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    assert_eq!(exp, writer.as_slice());
}

#[test]
fn test_float_data_writer_f64() {
    let mut writer = VecWriter::new();
    let w: &mut dyn Writer = &mut writer;
    let exp: [u8; 16] = [
        0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18, 0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d,
        0x18,
    ];

    match write_f64(3.1415926535897932384626433 as f64, w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_f64(3.1415926535897932384626433 as f64, w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    assert_eq!(exp, writer.as_slice());
}

#[test]
fn test_string_data_writer_f64() {
    let mut writer = VecWriter::new();
    let w: &mut dyn Writer = &mut writer;

    let exp: [u8; 60] = [
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61, 0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20,
        0x73, 0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73,
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61, 0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20,
        0x73, 0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73,
    ];

    match write_string("Lágrimas não são argumentos", w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_string("Lágrimas não são argumentos", w) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    assert_eq!(exp, writer.as_slice());
}

#[test]
fn test_ilint_data_writer() {
    let expected: [u8; 45] = [
        0xF7, 0xF8, 0x00, 0xF9, 0x01, 0x23, 0xFA, 0x01, 0x23, 0x45, 0xFB, 0x01, 0x23, 0x45, 0x67,
        0xFC, 0x01, 0x23, 0x45, 0x67, 0x89, 0xFD, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xFE, 0x01,
        0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07,
    ];
    let sample: [u64; 9] = [
        0xF7,
        0xF8,
        0x021B,
        0x01243D,
        0x0123465F,
        0x0123456881,
        0x012345678AA3,
        0x123456789ACC5,
        0xFFFFFFFFFFFFFFFF,
    ];
    let mut writer = VecWriter::new();
    let w: &mut dyn Writer = &mut writer;

    for exp in sample.iter() {
        match write_ilint(*exp, w) {
            Ok(_) => (),
            _ => panic!(),
        };
    }
    assert_eq!(expected, writer.as_slice())
}

#[test]
fn test_data_writer() {
    let mut writer = VecWriter::new();

    //let dw: &mut dyn Writer = &mut writer;

    match write_u8(SAMPLE_VALUES_00_U8, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_u16(SAMPLE_VALUES_01_U16, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_u32(SAMPLE_VALUES_02_U32, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_u64(SAMPLE_VALUES_03_U64, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_i8(SAMPLE_VALUES_04_I8, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_i16(SAMPLE_VALUES_05_I16, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_i32(SAMPLE_VALUES_06_I32, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_i64(SAMPLE_VALUES_07_I64, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_f32(SAMPLE_VALUES_08_F32 as f32, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_f64(SAMPLE_VALUES_09_F64, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_string(SAMPLE_VALUES_10_STR, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }
    match write_ilint(SAMPLE_VALUES_11_ILINT, &mut writer) {
        Ok(_) => (),
        Err(_) => panic!(),
    }

    assert_eq!(&SAMPLE_VALUES_BIN, writer.as_slice());
}

//=============================================================================
// Data Reader Traits
//-----------------------------------------------------------------------------
macro_rules! test_valuereader_read_value_item {
    ($reader: ident, $type: ty, $exp: expr) => {
        match $reader.read_value() as Result<$type> {
            Ok(v) => assert_eq!(v, $exp),
            _ => panic!("Unable to read the value"),
        };
    };
}

macro_rules! test_valuereader_read_value_item_error {
    ($reader: ident, $type: ty) => {
        match $reader.read_value() as Result<$type> {
            Err(_) => (),
            _ => panic!("Error expected!"),
        };
    };
}

fn test_datareader_trait_dyn_reader(reader: &mut dyn Reader) {
    // Test Read
    test_valuereader_read_value_item!(reader, u8, SAMPLE_VALUES_00_U8);
    test_valuereader_read_value_item!(reader, u16, SAMPLE_VALUES_01_U16);
    test_valuereader_read_value_item!(reader, u32, SAMPLE_VALUES_02_U32);
    test_valuereader_read_value_item!(reader, u64, SAMPLE_VALUES_03_U64);
    test_valuereader_read_value_item!(reader, i8, SAMPLE_VALUES_04_I8);
    test_valuereader_read_value_item!(reader, i16, SAMPLE_VALUES_05_I16);
    test_valuereader_read_value_item!(reader, i32, SAMPLE_VALUES_06_I32);
    test_valuereader_read_value_item!(reader, i64, SAMPLE_VALUES_07_I64);
    test_valuereader_read_value_item!(reader, f32, SAMPLE_VALUES_08_F32);
    test_valuereader_read_value_item!(reader, f64, SAMPLE_VALUES_09_F64);
    match reader.read_string(SAMPLE_VALUES_10_STR_LEN) {
        Ok(v) => assert_eq!(v, SAMPLE_VALUES_10_STR),
        _ => panic!("Unable to read the value"),
    }
    match reader.read_ilint() {
        Ok(v) => assert_eq!(v, SAMPLE_VALUES_11_ILINT),
        _ => panic!("Unable to read the value"),
    }

    // Test errors
    test_valuereader_read_value_item_error!(reader, u8);
    test_valuereader_read_value_item_error!(reader, u16);
    test_valuereader_read_value_item_error!(reader, u32);
    test_valuereader_read_value_item_error!(reader, u64);
    test_valuereader_read_value_item_error!(reader, i8);
    test_valuereader_read_value_item_error!(reader, i16);
    test_valuereader_read_value_item_error!(reader, i32);
    test_valuereader_read_value_item_error!(reader, i64);
    test_valuereader_read_value_item_error!(reader, f32);
    test_valuereader_read_value_item_error!(reader, f64);
    match reader.read_string(SAMPLE_VALUES_10_STR_LEN) {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }
    match reader.read_ilint() {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }
}

#[test]
fn test_datareader_trait_impl() {
    // Test for dyn Reader
    let mut reader = crate::io::array::ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    test_datareader_trait_dyn_reader(&mut reader);

    // Test for T:Reader
    let mut reader = crate::io::array::ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    test_valuereader_read_value_item!(reader, u8, SAMPLE_VALUES_00_U8);
    test_valuereader_read_value_item!(reader, u16, SAMPLE_VALUES_01_U16);
    test_valuereader_read_value_item!(reader, u32, SAMPLE_VALUES_02_U32);
    test_valuereader_read_value_item!(reader, u64, SAMPLE_VALUES_03_U64);
    test_valuereader_read_value_item!(reader, i8, SAMPLE_VALUES_04_I8);
    test_valuereader_read_value_item!(reader, i16, SAMPLE_VALUES_05_I16);
    test_valuereader_read_value_item!(reader, i32, SAMPLE_VALUES_06_I32);
    test_valuereader_read_value_item!(reader, i64, SAMPLE_VALUES_07_I64);
    test_valuereader_read_value_item!(reader, f32, SAMPLE_VALUES_08_F32);
    test_valuereader_read_value_item!(reader, f64, SAMPLE_VALUES_09_F64);
    match reader.read_string(SAMPLE_VALUES_10_STR_LEN) {
        Ok(v) => assert_eq!(v, SAMPLE_VALUES_10_STR),
        _ => panic!("Unable to read the value"),
    }
    match reader.read_ilint() {
        Ok(v) => assert_eq!(v, SAMPLE_VALUES_11_ILINT),
        _ => panic!("Unable to read the value"),
    }

    // Test errors
    test_valuereader_read_value_item_error!(reader, u8);
    test_valuereader_read_value_item_error!(reader, u16);
    test_valuereader_read_value_item_error!(reader, u32);
    test_valuereader_read_value_item_error!(reader, u64);
    test_valuereader_read_value_item_error!(reader, i8);
    test_valuereader_read_value_item_error!(reader, i16);
    test_valuereader_read_value_item_error!(reader, i32);
    test_valuereader_read_value_item_error!(reader, i64);
    test_valuereader_read_value_item_error!(reader, f32);
    test_valuereader_read_value_item_error!(reader, f64);
    match reader.read_string(SAMPLE_VALUES_10_STR_LEN) {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }
    match reader.read_ilint() {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }
}

macro_rules! test_valuewriter_write_value_item {
    ($writer: ident, $type: ty, $val: expr) => {
        match $writer.write_value($val as $type) {
            Ok(()) => (),
            _ => panic!("Unable to read the value!"),
        };
    };
}

macro_rules! test_valuewriter_write_value_item_err {
    ($writer: ident, $type: ty, $val: expr) => {
        match $writer.write_value($val as $type) {
            Err(_) => (),
            _ => panic!("Error expected!"),
        };
    };
}

fn test_datawriter_trait_dyn_writer(writer: &mut dyn Writer) {
    test_valuewriter_write_value_item!(writer, u8, SAMPLE_VALUES_00_U8);
    test_valuewriter_write_value_item!(writer, u16, SAMPLE_VALUES_01_U16);
    test_valuewriter_write_value_item!(writer, u32, SAMPLE_VALUES_02_U32);
    test_valuewriter_write_value_item!(writer, u64, SAMPLE_VALUES_03_U64);
    test_valuewriter_write_value_item!(writer, i8, SAMPLE_VALUES_04_I8);
    test_valuewriter_write_value_item!(writer, i16, SAMPLE_VALUES_05_I16);
    test_valuewriter_write_value_item!(writer, i32, SAMPLE_VALUES_06_I32);
    test_valuewriter_write_value_item!(writer, i64, SAMPLE_VALUES_07_I64);
    test_valuewriter_write_value_item!(writer, f32, SAMPLE_VALUES_08_F32);
    test_valuewriter_write_value_item!(writer, f64, SAMPLE_VALUES_09_F64);
    test_valuewriter_write_value_item!(writer, &str, SAMPLE_VALUES_10_STR);
    match writer.write_ilint(SAMPLE_VALUES_11_ILINT) {
        Ok(()) => (),
        _ => panic!("Unable to write the value."),
    }
}

fn test_datawriter_trait_dyn_writer_err(writer: &mut dyn Writer) {
    test_valuewriter_write_value_item_err!(writer, u8, SAMPLE_VALUES_00_U8);
    test_valuewriter_write_value_item_err!(writer, u16, SAMPLE_VALUES_01_U16);
    test_valuewriter_write_value_item_err!(writer, u32, SAMPLE_VALUES_02_U32);
    test_valuewriter_write_value_item_err!(writer, u64, SAMPLE_VALUES_03_U64);
    test_valuewriter_write_value_item_err!(writer, i8, SAMPLE_VALUES_04_I8);
    test_valuewriter_write_value_item_err!(writer, i16, SAMPLE_VALUES_05_I16);
    test_valuewriter_write_value_item_err!(writer, i32, SAMPLE_VALUES_06_I32);
    test_valuewriter_write_value_item_err!(writer, i64, SAMPLE_VALUES_07_I64);
    test_valuewriter_write_value_item_err!(writer, f32, SAMPLE_VALUES_08_F32);
    test_valuewriter_write_value_item_err!(writer, f64, SAMPLE_VALUES_09_F64);
    test_valuewriter_write_value_item_err!(writer, &str, SAMPLE_VALUES_10_STR);
    match writer.write_ilint(SAMPLE_VALUES_11_ILINT) {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }
}

#[test]
fn test_datawriter_trait_impl() {
    // Test for dyn Reader
    let mut vec: Vec<u8> = Vec::new();
    let mut writer = crate::io::array::BorrowedVecWriter::new(&mut vec);
    test_datawriter_trait_dyn_writer(&mut writer);
    writer.set_read_only(true);
    test_datawriter_trait_dyn_writer_err(&mut writer);
    assert_eq!(vec.as_slice(), &SAMPLE_VALUES_BIN);

    // Test for T:Reader
    let mut vec: Vec<u8> = Vec::new();
    let mut writer = crate::io::array::BorrowedVecWriter::new(&mut vec);
    test_valuewriter_write_value_item!(writer, u8, SAMPLE_VALUES_00_U8);
    test_valuewriter_write_value_item!(writer, u16, SAMPLE_VALUES_01_U16);
    test_valuewriter_write_value_item!(writer, u32, SAMPLE_VALUES_02_U32);
    test_valuewriter_write_value_item!(writer, u64, SAMPLE_VALUES_03_U64);
    test_valuewriter_write_value_item!(writer, i8, SAMPLE_VALUES_04_I8);
    test_valuewriter_write_value_item!(writer, i16, SAMPLE_VALUES_05_I16);
    test_valuewriter_write_value_item!(writer, i32, SAMPLE_VALUES_06_I32);
    test_valuewriter_write_value_item!(writer, i64, SAMPLE_VALUES_07_I64);
    test_valuewriter_write_value_item!(writer, f32, SAMPLE_VALUES_08_F32);
    test_valuewriter_write_value_item!(writer, f64, SAMPLE_VALUES_09_F64);
    test_valuewriter_write_value_item!(writer, &str, SAMPLE_VALUES_10_STR);
    match writer.write_ilint(SAMPLE_VALUES_11_ILINT) {
        Ok(()) => (),
        _ => panic!("Unable to write the value."),
    }
    writer.set_read_only(true);
    test_valuewriter_write_value_item_err!(writer, u8, SAMPLE_VALUES_00_U8);
    test_valuewriter_write_value_item_err!(writer, u16, SAMPLE_VALUES_01_U16);
    test_valuewriter_write_value_item_err!(writer, u32, SAMPLE_VALUES_02_U32);
    test_valuewriter_write_value_item_err!(writer, u64, SAMPLE_VALUES_03_U64);
    test_valuewriter_write_value_item_err!(writer, i8, SAMPLE_VALUES_04_I8);
    test_valuewriter_write_value_item_err!(writer, i16, SAMPLE_VALUES_05_I16);
    test_valuewriter_write_value_item_err!(writer, i32, SAMPLE_VALUES_06_I32);
    test_valuewriter_write_value_item_err!(writer, i64, SAMPLE_VALUES_07_I64);
    test_valuewriter_write_value_item_err!(writer, f32, SAMPLE_VALUES_08_F32);
    test_valuewriter_write_value_item_err!(writer, f64, SAMPLE_VALUES_09_F64);
    test_valuewriter_write_value_item_err!(writer, &str, SAMPLE_VALUES_10_STR);
    match writer.write_ilint(SAMPLE_VALUES_11_ILINT) {
        Err(_) => (),
        _ => panic!("Error expected!"),
    }

    assert_eq!(vec.as_slice(), &SAMPLE_VALUES_BIN);
}

//=============================================================================
// Signed ILInt tests
//-----------------------------------------------------------------------------
#[test]
fn test_read_signed_ilint() {
    for s in SIGNED_SAMPLES {
        let mut writer = VecWriter::new();
        assert!(crate::ilint::signed_encode(s, &mut writer).is_ok());
        let mut reader = ByteArrayReader::new(writer.as_slice());
        let r = match read_signed_ilint(&mut reader) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read the value."),
        };
        assert_eq!(s, r);

        let mut reader = ByteArrayReader::new(&writer.as_slice()[0..writer.as_slice().len() - 1]);
        match read_signed_ilint(&mut reader) {
            Err(_) => (),
            _ => panic!("Error expected."),
        };
    }
}

#[test]
fn test_write_signed_ilint() {
    for s in SIGNED_SAMPLES {
        let mut writer = VecWriter::new();
        assert!(write_signed_ilint(s, &mut writer).is_ok());

        let mut reader = ByteArrayReader::new(writer.as_slice());
        let r = match crate::ilint::signed_decode(&mut reader) {
            Ok(v) => v,
            Err(_) => panic!("Unable to read the value."),
        };
        assert_eq!(s, r);

        writer.set_read_only(true);
        match write_signed_ilint(s, &mut writer) {
            Err(_) => (),
            _ => panic!("Error expected."),
        };
    }
}
