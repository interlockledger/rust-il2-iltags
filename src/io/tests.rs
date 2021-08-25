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

struct DummyReader {
    pub available: usize,
    pub read_count: usize,
}

#[test]
fn test_errorkind_debug() {
    assert_eq!(format!("{:?}", ErrorKind::CorruptedData), "CorruptedData");
    assert_eq!(
        format!("{:?}", ErrorKind::UnableToReadData),
        "UnableToReadData"
    );
}

impl DummyReader {
    pub fn new(available: usize) -> DummyReader {
        DummyReader {
            available: available,
            read_count: 0,
        }
    }

    pub fn get_read_count(&self) -> usize {
        self.read_count
    }
}

impl<'a> Reader for DummyReader {
    fn read(&mut self) -> Result<u8> {
        if self.available > 0 {
            self.available -= 1;
            let r = self.read_count as u8;
            self.read_count += 1;
            Ok(r as u8)
        } else {
            Err(ErrorKind::UnableToReadData)
        }
    }
}

//=============================================================================
// SkipTestReader
//-----------------------------------------------------------------------------
struct SkipTestReader {
    offset: u64,
    size: u64,
}

impl SkipTestReader {
    pub fn new(size: u64) -> Self {
        Self { offset: 0, size }
    }
}

impl Reader for SkipTestReader {
    fn read(&mut self) -> Result<u8> {
        if self.offset < self.size {
            let b = self.offset as u8;
            self.offset += 1;
            Ok(b)
        } else {
            Err(ErrorKind::UnableToReadData)
        }
    }

    fn skip(&mut self, count: usize) -> Result<()> {
        let final_offset = self.offset + count as u64;
        if final_offset <= self.size {
            self.offset = final_offset;
            Ok(())
        } else {
            Err(ErrorKind::UnableToReadData)
        }
    }
}

#[test]
fn test_skiptestreader_skip() {
    let mut r = SkipTestReader::new(16);
    r.skip(15).unwrap();
    r.skip(1).unwrap();
    assert!(matches!(r.skip(1), Err(ErrorKind::UnableToReadData)));

    let mut r = SkipTestReader::new(16);
    assert!(matches!(r.skip(17), Err(ErrorKind::UnableToReadData)));
}

#[test]
fn test_skiptestreader_skip_u64() {
    let mut r = SkipTestReader::new(16);
    r.skip_u64(15).unwrap();
    r.skip_u64(1).unwrap();
    assert!(matches!(r.skip_u64(1), Err(ErrorKind::UnableToReadData)));

    let mut r = SkipTestReader::new(16);
    assert!(matches!(r.skip_u64(17), Err(ErrorKind::UnableToReadData)));

    let mut r = SkipTestReader::new((usize::MAX / 2) as u64 + 2);
    r.skip_u64((usize::MAX / 2) as u64 + 2).unwrap();
    assert!(matches!(r.skip_u64(1), Err(ErrorKind::UnableToReadData)));
}

//=============================================================================
// LimitedReader
//-----------------------------------------------------------------------------
#[test]
fn test_limitedreader_new() {
    let mut reader = DummyReader::new(10);
    let limited = LimitedReader::new(&mut reader, 10);

    assert_eq!(limited.available(), 10)
}

#[test]
fn test_limitedreader_can_read() {
    let mut reader = DummyReader::new(10);
    let limited = LimitedReader::new(&mut reader, 10);

    for i in 0..11 {
        assert!(limited.can_read(i).is_ok());
    }
    match limited.can_read(11) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!("Unexpected response."),
    }
}

#[test]
fn test_limitedreader_available() {
    let mut reader = DummyReader::new(10);

    for i in 0..10 {
        let limited = LimitedReader::new(&mut reader, i);
        assert_eq!(limited.available(), i);
    }
}

#[test]
fn test_limitedreader_empty() {
    let mut reader = DummyReader::new(10);

    let mut limited = LimitedReader::new(&mut reader, 5);
    for _i in 0..5 {
        assert!(!limited.empty());
        match limited.read() {
            Ok(_) => (),
            _ => panic!(),
        }
    }
    assert!(limited.empty());
}

#[test]
fn test_limitedreader_goto_end() {
    for i in 0..10 {
        let mut reader = DummyReader::new(10);
        let mut limited = LimitedReader::new(&mut reader, i);
        assert_eq!(limited.available(), i);
        assert!(limited.goto_end().is_ok());
        assert_eq!(limited.available(), 0);
        assert_eq!(reader.get_read_count(), i);
    }

    let mut reader = DummyReader::new(10);
    let mut limited = LimitedReader::new(&mut reader, 11);
    match limited.goto_end() {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!("Unexpected response."),
    }
    assert_eq!(limited.available(), 11);
    assert_eq!(reader.get_read_count(), 10);
}

#[test]
fn test_limitedreader_read() {
    let mut reader = DummyReader::new(10);

    {
        let mut limited = LimitedReader::new(&mut reader, 5);
        for i in 0..5 {
            match limited.read() {
                Ok(v) => assert_eq!(v, i),
                _ => panic!("Unexpected error"),
            }
        }
        match limited.read() {
            Err(ErrorKind::UnableToReadData) => (),
            _ => panic!("Unexpected error"),
        }
    }
    assert_eq!(reader.get_read_count(), 5);
}

fn assert_sequence(value: &[u8], expected_size: usize) {
    assert_eq!(value.len(), expected_size);
    if value.len() > 0 {
        let mut i = 0;
        for x in value {
            assert_eq!(*x, i as u8);
            i += 1
        }
    }
}

#[test]
fn test_limitedreader_read_all() {
    let mut buff: [u8; 10] = [0; 10];

    let mut reader = DummyReader::new(10);
    let mut limited = LimitedReader::new(&mut reader, 5);
    match limited.read_all(&mut buff[0..0]) {
        Ok(()) => assert_sequence(&buff[0..0], 0),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[0..1]) {
        Ok(()) => assert_sequence(&buff[0..1], 1),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[1..4]) {
        Ok(()) => assert_sequence(&buff[0..4], 4),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[4..6]) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[4..5]) {
        Ok(()) => assert_sequence(&buff[0..5], 5),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[0..0]) {
        Ok(()) => assert_sequence(&buff[0..5], 5),
        _ => panic!("Unexpected error"),
    }
    match limited.read_all(&mut buff[0..1]) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!("Unexpected error"),
    }
}
