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
use crate::io::{ByteArrayReader, VecWriter};

macro_rules! test_int_data_reader_t {
    ($type: ty, $sample : expr, $expected: expr) => {{
        let mut r = ByteArrayReader::new($sample);

        for exp_val in $expected {
            match IntDataReader::<$type>::read_int(&mut r) {
                Ok(v) => assert_eq!(v, *exp_val as $type),
                _ => panic!(),
            }
        }
        assert!(IntDataReader::<$type>::read_int(&mut r).is_err());
    }};
}

#[test]
fn test_int_data_reader_u8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_reader_t!(u8, &sample, &exp);
}

#[test]
fn test_int_data_reader_i8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i8; 8] = [0x01, 0x23, 0x45, 0x67, -0x77, -0x55, -0x33, -0x11];
    test_int_data_reader_t!(i8, &sample, &exp);
}

#[test]
fn test_int_data_reader_u16() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u16; 4] = [0x0123, 0x4567, 0x89AB, 0xCDEF];
    test_int_data_reader_t!(u16, &sample, &exp);
}

#[test]
fn test_int_data_reader_i16() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i16; 4] = [0x0123, 0x4567, -0x7655, -0x3211];
    test_int_data_reader_t!(i16, &sample, &exp);
}

#[test]
fn test_int_data_reader_u32() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u32; 2] = [0x01234567, 0x89ABCDEF];
    test_int_data_reader_t!(u32, &sample, &exp);
}

#[test]
fn test_int_data_reader_i32() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [i32; 2] = [0x01234567, -0x76543211];
    test_int_data_reader_t!(i32, &sample, &exp);
}

#[test]
fn test_int_data_reader_u64() {
    let sample: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let exp: [u64; 2] = [0x0123456789ABCDEF, 0xFEDCBA9876543210];
    test_int_data_reader_t!(u64, &sample, &exp);
}

#[test]
fn test_int_data_reader_i64() {
    let sample: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let exp: [i64; 2] = [0x0123456789ABCDEF, -0x123456789ABCDF0];
    test_int_data_reader_t!(i64, &sample, &exp);
}

#[test]
fn test_ilint_data_reader(){
    let sample: [u8; 46] = [
        0xF7,
        0xF8, 0x00,
        0xF9, 0x01, 0x23,
        0xFA, 0x01, 0x23, 0x45,
        0xFB, 0x01, 0x23, 0x45, 0x67,
        0xFC, 0x01, 0x23, 0x45, 0x67, 0x89,
        0xFD, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 
        0xFE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07, 
        0xFF
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
        0xFFFFFFFFFFFFFFFF
    ];

    let mut r = ByteArrayReader::new(&sample);
    for exp in expected.iter() {
        match ILIntDataReader::read_ilint(&mut r) {
            Ok(v) => assert_eq!(*exp, v),
            _ => panic!()
        };
    }
    assert!(ILIntDataReader::read_ilint(&mut r).is_err());
}

#[test]
fn test_float_data_reader_f32() {
    let sample: [u8; 5] = [
        0x40, 0x49, 0x0f, 0xdb, 
        0x01
    ];
    let mut r = ByteArrayReader::new(&sample);
    match FloatDataReader::<f32>::read_float(&mut r) {
        Ok(v) => assert_eq!(3.14159274101257324, v),
        _ => panic!()
    };
    assert!(FloatDataReader::<f32>::read_float(&mut r).is_err());
}

#[test]
fn test_float_data_reader_f64() {
    let sample: [u8; 9] = [
        0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18,
        0x01
    ];
    let mut r = ByteArrayReader::new(&sample);
    match FloatDataReader::<f64>::read_float(&mut r) {
        Ok(v) => assert_eq!(3.1415926535897932384626433, v),
        _ => panic!()
    };
    assert!(FloatDataReader::<f64>::read_float(&mut r).is_err());
}

#[test]
fn test_string_data_reader() {
    // This frase is attibuted to Machado de Assis. It was
    // choosen because it contains Latin characters that
    // result in a multi-byte characters in UTF-8.
    let sample: [u8; 30] = [
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61,
        0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73,
        0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75,
        0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73
    ];
    let expected = "Lágrimas não são argumentos";
    
    let mut r = ByteArrayReader::new(&sample);


    match StringDataReader::read_string(&mut r, 30) {
        Ok(v) => assert_eq!(expected, v),
        _ => panic!()
    };
    assert!(StringDataReader::read_string(&mut r, 0).is_ok());
    assert!(StringDataReader::read_string(&mut r, 1).is_err());
}

#[test]
fn test_data_reader() {
    // This frase is attibuted to Machado de Assis. It was
    // choosen because it contains Latin characters that
    // result in a multi-byte characters in UTF-8.
    let sample: [u8; 72] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 
        0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
        0x28, 0x29,
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61,
        0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73,
        0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75,
        0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73
    ];

    let mut r = ByteArrayReader::new(&sample);
    let dr: &mut dyn DataReader = &mut r; 

    match dr.read_int() as Result<u8> {
        Ok(v) =>  assert_eq!(0x00,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<u16> {
        Ok(v) =>  assert_eq!(0x0102,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<u32> {
        Ok(v) =>  assert_eq!(0x0304_0506,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<u64> {
        Ok(v) =>  assert_eq!(0x0708_090A_0B0C_0D0E,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<i8> {
        Ok(v) =>  assert_eq!(0x0f,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<i16> {
        Ok(v) =>  assert_eq!(0x1011,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<i32> {
        Ok(v) =>  assert_eq!(0x1213_1415,  v),
        _ => panic!()
    }
    match dr.read_int() as Result<i64> {
        Ok(v) =>  assert_eq!(0x1617_1819_1A1B_1C1D,  v),
        _ => panic!()
    }
    match dr.read_float() as Result<f32> {
        Ok(v) =>  assert_eq!(0.000000000000000000008424034,  v),
        _ => panic!()
    }
    match dr.read_float() as Result<f64> {
        Ok(v) =>  assert_eq!(0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030657805298494026,  v),
        _ => panic!()
    }
    match dr.read_string(30) {
        Ok(v) =>  assert_eq!("Lágrimas não são argumentos", v),
        _ => panic!()
    }
    match dr.read_int() as Result<u8> {
        Ok(_) =>  panic!(),
        _ => ()
    }
}

macro_rules! test_int_data_writer_t {
    ($type: ty, $sample : expr, $expected: expr) => {{
        let mut bin = Vec::<u8>::new();
        let mut w =  VecWriter::new(&mut bin);

        for val in $sample {
            match IntDataWriter::<$type>::write_int(&mut w, *val) {
                Ok(_) => (),
                _ => panic!(),
            }
        }
        assert_eq!($expected.len(), bin.len());
        assert_eq!($expected, bin.as_slice());
    }};
}

#[test]
fn test_int_data_writer_u8() {
    let sample: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u8, &sample, &exp);
}

#[test]
fn test_int_data_writer_i8() {
    let sample: [i8; 8] = [0x01, 0x23, 0x45, 0x67, -0x77, -0x55, -0x33, -0x11];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i8, &sample, &exp);
}

#[test]
fn test_int_data_writer_u16() {
    let sample: [u16; 4] = [0x0123, 0x4567, 0x89AB, 0xCDEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u16, &sample, &exp);
}

#[test]
fn test_int_data_writer_i16() {
    let sample: [i16; 4] = [0x0123, 0x4567, -0x7655, -0x3211];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i16, &sample, &exp);
}

#[test]
fn test_int_data_writer_u32() {
    let sample: [u32; 2] = [0x01234567, 0x89ABCDEF];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u32, &sample, &exp);
}

#[test]
fn test_int_data_writer_i32() {
    let sample: [i32; 2] = [0x01234567, -0x76543211];
    let exp: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i32, &sample, &exp);
}

#[test]
fn test_int_data_writer_u64() {
    let sample: [u64; 2] = [0x0123456789ABCDEF, 0x0123456789ABCDEF];
    let exp: [u8; 16] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
                        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(u64, &sample, &exp);
}

#[test]
fn test_int_data_writer_i64() {
    let sample: [i64; 2] = [0x0123456789ABCDEF, -0x7EDCBA9876543211];
    let exp: [u8; 16] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
                        0x81, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    test_int_data_writer_t!(i64, &sample, &exp);
}

#[test]
fn test_float_data_writer_f32() {
    let mut bin = Vec::<u8>::new();
    let mut w =  VecWriter::new(&mut bin);
    let exp: [u8; 8] = 
                    [0x40, 0x49, 0x0f, 0xdb,
                    0x40, 0x49, 0x0f, 0xdb,];

    match FloatDataWriter::<f32>::write_float(&mut w, 3.14159274101257324 as f32) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match FloatDataWriter::<f32>::write_float(&mut w, 3.14159274101257324 as f32) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    assert_eq!(exp, bin.as_slice());
}

#[test]
fn test_float_data_writer_f64() {
    let mut bin = Vec::<u8>::new();
    let mut w =  VecWriter::new(&mut bin);
    let exp: [u8; 16] = [
        0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18,
        0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];

    match FloatDataWriter::<f64>::write_float(&mut w, 3.1415926535897932384626433 as f64) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match FloatDataWriter::<f64>::write_float(&mut w, 3.1415926535897932384626433 as f64) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    assert_eq!(exp, bin.as_slice());
}

#[test]
fn test_string_data_writer_f64() {
    let mut bin = Vec::<u8>::new();
    let mut w =  VecWriter::new(&mut bin);
    let exp: [u8; 60] = [
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61,
        0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73,
        0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75,
        0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73,
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61,
        0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73,
        0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75,
        0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73];

    match StringDataWriter::write_string(&mut w, "Lágrimas não são argumentos") {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match StringDataWriter::write_string(&mut w, "Lágrimas não são argumentos") {
        Ok(_) => (),
        Err(_) => panic!()
    }
    assert_eq!(exp, bin.as_slice());
}

#[test]
fn test_data_writer() {
    let mut bin = Vec::<u8>::new();
    let mut w =  VecWriter::new(&mut bin);
    let exp: [u8; 72] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 
        0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1e, 0x1f,
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 
        0x28, 0x29,
        0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69, 0x6d, 0x61,
        0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73,
        0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72, 0x67, 0x75,
        0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73];

    let dw : &mut dyn DataWriter = &mut w;

    match dw.write_int(0x01 as u8) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x0203 as u16) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x0405_0607 as u32) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x0809_0A0B_0C0D_0E0F as u64) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x10 as i8) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x1112 as i16) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x1314_1516 as i32) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_int(0x1718_191A_1B1C_1D1E as i64) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_float(0.000000000000000000008424034 as f32) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_float(0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030657805298494026 as f64) {
        Ok(_) => (),
        Err(_) => panic!()
    }
    match dw.write_string("Lágrimas não são argumentos") {
        Ok(_) => (),
        Err(_) => panic!()
    }

    assert_eq!(exp, bin.as_slice());
}