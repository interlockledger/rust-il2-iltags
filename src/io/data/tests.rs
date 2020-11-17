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
use crate::io::ByteArrayReader;

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