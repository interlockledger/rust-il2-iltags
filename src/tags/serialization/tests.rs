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
use crate::io::array::ByteArrayReader;
use crate::io::data::test_samples::*;

macro_rules! test_deserializer_core {
    ($reader: expr) => {
        let r: u8 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_00_U8);
        let r: u16 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_01_U16);
        let r: u32 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_02_U32);
        let r: u64 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_03_U64);
        let r: i8 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_04_I8);
        let r: i16 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_05_I16);
        let r: i32 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_06_I32);
        let r: i64 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_07_I64);
        let r: f32 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_08_F32);
        let r: f64 = $reader.deserialize_value()?;
        assert_eq!(r, SAMPLE_VALUES_09_F64);
        let r = $reader.deserialize_string(SAMPLE_VALUES_10_STR_LEN)?;
        assert_eq!(r, SAMPLE_VALUES_10_STR);
        let r: u64 = $reader.deserialize_ilint()?;
        assert_eq!(r, SAMPLE_VALUES_11_ILINT);

        match $reader.deserialize_value() as Result<u8> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u16> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u32> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u64> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i8> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i16> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i32> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i64> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<f32> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<f64> {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_string(SAMPLE_VALUES_10_STR_LEN) {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_ilint() {
            Err(_) => (),
            _ => panic!("Error expected."),
        }
    };
}

//=============================================================================
// ValueDeserializer
//-----------------------------------------------------------------------------
#[test]
fn test_deserializer_struct_impl() -> Result<()> {
    let mut reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    test_deserializer_core!(reader);

    let mut reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let r = reader.deserialize_bytes(SAMPLE_VALUES_BIN_SIZE)?;
    assert_eq!(r.as_slice(), &SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes(SAMPLE_VALUES_BIN_SIZE) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }

    let mut reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let mut r: [u8; SAMPLE_VALUES_BIN_SIZE] = [0; SAMPLE_VALUES_BIN_SIZE];
    reader.deserialize_bytes_into_slice(&mut r)?;
    assert_eq!(r, SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes_into_slice(&mut r) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }

    let mut reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let mut r: Vec<u8> = Vec::new();
    reader.deserialize_bytes_into_vec(SAMPLE_VALUES_BIN_SIZE, &mut r)?;
    assert_eq!(r.as_slice(), &SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes_into_vec(SAMPLE_VALUES_BIN_SIZE, &mut r) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }

    Ok(())
}

#[test]
fn test_deserializer_dyn_reader_impl() -> Result<()> {
    let mut actual_reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let reader: &mut dyn Reader = &mut actual_reader;

    test_deserializer_core!(reader);

    let mut actual_reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let reader: &mut dyn Reader = &mut actual_reader;
    let r = reader.deserialize_bytes(SAMPLE_VALUES_BIN_SIZE)?;
    assert_eq!(r.as_slice(), &SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes(SAMPLE_VALUES_BIN_SIZE) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }

    let mut actual_reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let reader: &mut dyn Reader = &mut actual_reader;
    let mut r: [u8; SAMPLE_VALUES_BIN_SIZE] = [0; SAMPLE_VALUES_BIN_SIZE];
    reader.deserialize_bytes_into_slice(&mut r)?;
    assert_eq!(r, SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes_into_slice(&mut r) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }

    let mut actual_reader = ByteArrayReader::new(&SAMPLE_VALUES_BIN);
    let reader: &mut dyn Reader = &mut actual_reader;
    let mut r: Vec<u8> = Vec::new();
    reader.deserialize_bytes_into_vec(SAMPLE_VALUES_BIN_SIZE, &mut r)?;
    assert_eq!(r.as_slice(), &SAMPLE_VALUES_BIN);
    match reader.deserialize_bytes_into_vec(SAMPLE_VALUES_BIN_SIZE, &mut r) {
        Err(_) => (),
        _ => panic!("Error expected."),
    }
    Ok(())
}
