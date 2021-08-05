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
use crate::io::data::test_samples::*;
use crate::tags::ErrorKind;

//=============================================================================
// Deserializer Tests
//-----------------------------------------------------------------------------
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
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u16> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u32> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<u64> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i8> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i16> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i32> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<i64> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<f32> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_value() as Result<f64> {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_string(SAMPLE_VALUES_10_STR_LEN) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $reader.deserialize_ilint() {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
    };
}

#[test]
fn test_deserializer_struct() -> Result<()> {
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
fn test_deserializer_dyn_reader() -> Result<()> {
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

//=============================================================================
// Serializer Tests
//-----------------------------------------------------------------------------

macro_rules! test_serializer_core1 {
    ($writer: expr) => {
        $writer.serialize_value(SAMPLE_VALUES_00_U8)?;
        $writer.serialize_value(SAMPLE_VALUES_01_U16)?;
        $writer.serialize_value(SAMPLE_VALUES_02_U32)?;
        $writer.serialize_value(SAMPLE_VALUES_03_U64)?;
        $writer.serialize_value(SAMPLE_VALUES_04_I8)?;
        $writer.serialize_value(SAMPLE_VALUES_05_I16)?;
        $writer.serialize_value(SAMPLE_VALUES_06_I32)?;
        $writer.serialize_value(SAMPLE_VALUES_07_I64)?;
        $writer.serialize_value(SAMPLE_VALUES_08_F32)?;
        $writer.serialize_value(SAMPLE_VALUES_09_F64)?;
        $writer.serialize_value(SAMPLE_VALUES_10_STR)?;
        $writer.serialize_ilint(SAMPLE_VALUES_11_ILINT)?;
    };
}

macro_rules! test_serializer_core2 {
    ($writer: expr) => {
        match $writer.serialize_value(SAMPLE_VALUES_00_U8) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_01_U16) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_02_U32) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_03_U64) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_04_I8) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_05_I16) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_06_I32) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_07_I64) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_08_F32) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_09_F64) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_value(SAMPLE_VALUES_10_STR) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
        match $writer.serialize_ilint(SAMPLE_VALUES_11_ILINT) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
    };
}

#[test]
fn test_serializer_struct() -> Result<()> {
    let mut writer = VecWriter::new();

    test_serializer_core1!(writer);
    assert_eq!(writer.as_slice(), &SAMPLE_VALUES_BIN);

    writer.set_read_only(true);
    test_serializer_core2!(writer);

    let mut writer = VecWriter::new();
    writer.serialize_bytes(&SAMPLE_VALUES_BIN)?;
    assert_eq!(writer.as_slice(), &SAMPLE_VALUES_BIN);
    writer.set_read_only(true);
    match writer.serialize_bytes(&SAMPLE_VALUES_BIN) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error expected."),
    }

    let mut writer = VecWriter::new();
    let mut v: Vec<u8> = Vec::with_capacity(SAMPLE_VALUES_BIN_SIZE);
    v.extend_from_slice(&SAMPLE_VALUES_BIN);
    writer.serialize_bytes(&v)?;
    assert_eq!(writer.as_slice(), &SAMPLE_VALUES_BIN);
    writer.set_read_only(true);
    match writer.serialize_bytes(&v) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Error expected."),
    }
    Ok(())
}

#[test]
fn test_serializer_dyn_writer() -> Result<()> {
    let mut actual_writer = VecWriter::new();
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        test_serializer_core1!(writer);
    }
    assert_eq!(actual_writer.as_slice(), &SAMPLE_VALUES_BIN);
    actual_writer.set_read_only(true);
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        test_serializer_core2!(writer);
    }

    let mut actual_writer = VecWriter::new();
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        writer.serialize_bytes(&SAMPLE_VALUES_BIN)?;
    }
    assert_eq!(actual_writer.as_slice(), &SAMPLE_VALUES_BIN);
    actual_writer.set_read_only(true);
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        match writer.serialize_bytes(&SAMPLE_VALUES_BIN) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
    }

    let mut actual_writer = VecWriter::new();
    let mut v: Vec<u8> = Vec::with_capacity(SAMPLE_VALUES_BIN_SIZE);
    v.extend_from_slice(&SAMPLE_VALUES_BIN);
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        writer.serialize_bytes(&v)?;
    }
    assert_eq!(actual_writer.as_slice(), &SAMPLE_VALUES_BIN);
    actual_writer.set_read_only(true);
    {
        let writer: &mut dyn Writer = &mut actual_writer;
        match writer.serialize_bytes(&v) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Error expected."),
        }
    }
    Ok(())
}
