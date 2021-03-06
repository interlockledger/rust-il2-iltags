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
use crate::io::array::{ByteArrayReader, MemoryReader, VecWriter};

#[test]
fn test_errorkind_debug() {
    assert_eq!(format!("{:?}", ErrorKind::InvalidFormat), "InvalidFormat");
    assert_eq!(format!("{:?}", ErrorKind::ValueOverflow), "ValueOverflow");
    assert_eq!(
        format!(
            "{:?}",
            ErrorKind::IOError(crate::io::ErrorKind::CorruptedData)
        ),
        "IOError(CorruptedData)"
    );
}

pub struct SampleILInt {
    pub value: u64,
    pub encoded_size: usize,
    pub encoded: [u8; 10],
}

const FILLER: u8 = 0xA5;

const SAMPLE_VALUES: [SampleILInt; 10] = [
    SampleILInt {
        value: 0xF7,
        encoded_size: 1,
        encoded: [
            0xF7, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0xF8,
        encoded_size: 2,
        encoded: [
            0xF8, 0x00, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x021B,
        encoded_size: 3,
        encoded: [
            0xF9, 0x01, 0x23, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x01243D,
        encoded_size: 4,
        encoded: [
            0xFA, 0x01, 0x23, 0x45, FILLER, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x0123465F,
        encoded_size: 5,
        encoded: [
            0xFB, 0x01, 0x23, 0x45, 0x67, FILLER, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x0123456881,
        encoded_size: 6,
        encoded: [
            0xFC, 0x01, 0x23, 0x45, 0x67, 0x89, FILLER, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x012345678AA3,
        encoded_size: 7,
        encoded: [
            0xFD, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, FILLER, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x123456789ACC5,
        encoded_size: 8,
        encoded: [
            0xFE, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, FILLER, FILLER,
        ],
    },
    SampleILInt {
        value: 0x123456789ABCEE7,
        encoded_size: 9,
        encoded: [0xFF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, FILLER],
    },
    SampleILInt {
        value: 0xFFFFFFFFFFFFFFFF,
        encoded_size: 9,
        encoded: [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x07, FILLER],
    },
];

#[test]
fn test_constants() {
    assert_eq!(ILINT_BASE, 0xF8);
}

#[test]
fn test_encoded_size() {
    assert_eq!(encoded_size(0), 1);
    assert_eq!(encoded_size(ILINT_BASE_U64 - 1), 1);
    assert_eq!(encoded_size(ILINT_BASE_U64), 2);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFF), 2);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100), 3);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFF), 3);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x10000), 4);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFF), 4);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1000000), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFF), 5);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100000000), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFF), 6);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x10000000000), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFFFF), 7);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x1000000000000), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0xFFFFFFFFFFFFFF), 8);
    assert_eq!(encoded_size(ILINT_BASE_U64 + 0x100000000000000), 9);
    assert_eq!(encoded_size(0xFFFFFFFFFFFFFFFF), 9);
}

#[test]
fn test_encode() {
    for i in 0..0xF8 {
        let mut writer = VecWriter::new();
        match encode(i as u64, &mut writer) {
            Ok(()) => (),
            Err(_) => panic!("Success expected!"),
        }
        assert_eq!(writer.as_slice()[0], i as u8);

        let mut writer = VecWriter::new();
        writer.set_read_only(true);
        assert!(encode(i as u64, &mut writer).is_err());
    }

    for sample in &SAMPLE_VALUES {
        let enc_size = sample.encoded_size;

        let mut writer = VecWriter::new();
        match encode(sample.value, &mut writer) {
            Ok(()) => (),
            Err(_) => panic!("Success expected!"),
        }
        assert_eq!(enc_size, writer.get_offset());
        assert_eq!(writer.as_slice()[0..enc_size], sample.encoded[0..enc_size]);

        let mut writer = VecWriter::new();
        writer.set_read_only(true);
        assert!(encode(sample.value as u64, &mut writer).is_err());
    }
}

#[test]
fn test_decoded_size() {
    for i in 0..0xF8 {
        assert_eq!(decoded_size(i), 1);
    }
    assert_eq!(decoded_size(0xF8), 2);
    assert_eq!(decoded_size(0xF9), 3);
    assert_eq!(decoded_size(0xFA), 4);
    assert_eq!(decoded_size(0xFB), 5);
    assert_eq!(decoded_size(0xFC), 6);
    assert_eq!(decoded_size(0xFD), 7);
    assert_eq!(decoded_size(0xFE), 8);
    assert_eq!(decoded_size(0xFF), 9);
}

#[test]
fn test_decode_body() {
    for sample in &SAMPLE_VALUES {
        let enc_size = sample.encoded_size;
        if enc_size > 1 {
            match decode_body(&sample.encoded[1..enc_size]) {
                Ok(v) => {
                    assert_eq!(v, sample.value);
                }
                _ => panic!(),
            }
        }
    }

    let sample: [u8; 9] = [0; 9];
    match decode_body(&sample[0..0]) {
        Err(ErrorKind::InvalidFormat) => (),
        _ => panic!(),
    }
    match decode_body(&sample) {
        Err(ErrorKind::InvalidFormat) => (),
        _ => panic!(),
    }
}

#[test]
fn test_decode_from_bytes() {
    for sample in &SAMPLE_VALUES {
        match decode_from_bytes(&sample.encoded) {
            Ok((v, size)) => {
                assert_eq!(v, sample.value);
                assert_eq!(size, sample.encoded_size);
            }
            _ => panic!(),
        }
        match decode_from_bytes(&sample.encoded[0..sample.encoded_size]) {
            Ok((v, size)) => {
                assert_eq!(v, sample.value);
                assert_eq!(size, sample.encoded_size);
            }
            _ => panic!(),
        }
        if sample.encoded_size > 1 {
            match decode_from_bytes(&sample.encoded[0..sample.encoded_size - 1]) {
                Err(ErrorKind::InvalidFormat) => (),
                _ => panic!(),
            }
        }
    }

    let sample: [u8; 0] = [0; 0];
    match decode_from_bytes(&sample) {
        Err(ErrorKind::InvalidFormat) => (),
        _ => panic!(),
    }
}

#[test]
fn test_decode() {
    // All with 1 byte
    for i in 0..0xF8 {
        let mut buff: [u8; 1] = [0];
        buff[0] = i as u8;

        let mut reader = ByteArrayReader::new(&buff);
        match decode(&mut reader) {
            Ok(v) => {
                assert_eq!(v, i as u64);
                assert_eq!(reader.offset(), 1);
            }
            _ => panic!(),
        }
    }

    // From samples
    for sample in &SAMPLE_VALUES {
        let enc_size = sample.encoded_size;
        let mut reader = ByteArrayReader::new(&sample.encoded[0..enc_size]);
        match decode(&mut reader) {
            Ok(v) => {
                assert_eq!(v, sample.value);
                assert_eq!(reader.offset(), enc_size);
            }
            _ => panic!(),
        }
    }

    // Corrupted due to size
    let mut encoded: [u8; 9] = [0; 9];
    for size in 2..10 {
        encoded[0] = ILINT_BASE + (size - 2) as u8;
        for bad_size in 0..size {
            let mut reader = ByteArrayReader::new(&encoded[0..bad_size]);
            match decode(&mut reader) {
                Err(super::ErrorKind::IOError(_)) => {}
                _ => panic!("Corrupted data expected."),
            }
        }
    }

    // Overflow!
    let mut encoded: [u8; 9] = [0xFF; 9];
    for last in 0x08..0x100 {
        encoded[8] = last as u8;
        let mut reader = ByteArrayReader::new(&encoded);
        match decode(&mut reader) {
            Err(super::ErrorKind::ValueOverflow) => {}
            _ => panic!("Overflow expected."),
        }
    }
}

#[test]
fn test_encode_sign() {
    // Unsigned
    assert_eq!(encode_sign(0), 0);
    assert_eq!(encode_sign(1), 2);
    assert_eq!(encode_sign(0x7FFF_FFFF_FFFF_FFFF), 0xFFFF_FFFF_FFFF_FFFE);
    // Signed
    assert_eq!(encode_sign(-1), 1);
    assert_eq!(encode_sign(-2), 3);
    assert_eq!(
        encode_sign(-9_223_372_036_854_775_808),
        0xFFFF_FFFF_FFFF_FFFF
    );
}

#[test]
fn test_decode_sign() {
    // Unsigned
    assert_eq!(decode_sign(0), 0);
    assert_eq!(decode_sign(2), 1);
    assert_eq!(decode_sign(0xFFFF_FFFF_FFFF_FFFE), 0x7FFF_FFFF_FFFF_FFFF);
    // Signed
    assert_eq!(decode_sign(1), -1);
    assert_eq!(decode_sign(3), -2);
    assert_eq!(
        decode_sign(0xFFFF_FFFF_FFFF_FFFF),
        -9_223_372_036_854_775_808
    );
}

const SIGNED_SAMPLES: [i64; 9] = [
    9_223_372_036_854_775_807,
    4_294_967_295,
    2,
    1,
    0,
    -1,
    -2,
    -4_294_967_295,
    -9_223_372_036_854_775_808,
];

#[test]
fn test_encode_decode_sign() {
    for s in SIGNED_SAMPLES {
        let e = encode_sign(s);
        assert_eq!(s, decode_sign(e));
    }
}

#[test]
fn test_signed_encoded_size() {
    // Unsigned
    assert_eq!(signed_encoded_size(0), encoded_size(encode_sign(0)));
    assert_eq!(signed_encoded_size(1), encoded_size(encode_sign(1)));
    assert_eq!(
        signed_encoded_size(9_223_372_036_854_775_807),
        encoded_size(encode_sign(9_223_372_036_854_775_807))
    );
    assert_eq!(signed_encoded_size(-1), encoded_size(encode_sign(-1)));
    assert_eq!(signed_encoded_size(-2), encoded_size(encode_sign(-2)));
    assert_eq!(
        signed_encoded_size(-9_223_372_036_854_775_808),
        encoded_size(encode_sign(-9_223_372_036_854_775_808))
    );
}

#[test]
fn test_signed_encode() {
    for s in SIGNED_SAMPLES {
        let mut writer = VecWriter::new();
        assert!(signed_encode(s, &mut writer).is_ok());
        let mut exp = VecWriter::new();
        assert!(encode(encode_sign(s), &mut exp).is_ok());
        assert_eq!(exp.as_slice(), writer.as_slice());
    }
}

#[test]
fn test_signed_decode() {
    for s in SIGNED_SAMPLES {
        let mut exp = VecWriter::new();
        assert!(encode(encode_sign(s), &mut exp).is_ok());

        let mut reader = ByteArrayReader::new(exp.as_slice());
        let r: i64 = match signed_decode(&mut reader) {
            Ok(v) => v,
            _ => panic!("Reading error"),
        };
        assert_eq!(s, r);

        let mut reader = ByteArrayReader::new(&exp.as_slice()[..exp.as_slice().len() - 1]);
        match signed_decode(&mut reader) {
            Err(ErrorKind::IOError(_)) => (),
            _ => panic!("Reading error"),
        };
    }
}
