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
use crate::tests::fill_sample;

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

//=============================================================================
// ReadReader
//-----------------------------------------------------------------------------
#[test]
fn test_readreader_new() {
    let mut sample: [u8; 10] = [0; 10];
    fill_sample(&mut sample);
    let mut mem_reader = std::io::Cursor::new(&sample);
    let mut reader = ReadReader::new(&mut mem_reader);
    match reader.read() {
        Ok(v) => assert_eq!(v, 0),
        _ => panic!("Unexpected error!"),
    }
}

#[test]
fn test_readreader_read() {
    let mut sample: [u8; 10] = [0; 10];
    fill_sample(&mut sample);
    let mut mem_reader = std::io::Cursor::new(&sample);
    let mut reader = ReadReader::new(&mut mem_reader);
    for i in 0..10 {
        match reader.read() {
            Ok(v) => assert_eq!(v, i as u8),
            _ => panic!("Unexpected error!"),
        }
    }
    match reader.read() {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unexpected error!"),
    }
}

#[test]
fn test_readreader_read_all() {
    let mut sample: [u8; 10] = [0; 10];
    fill_sample(&mut sample);
    let mut mem_reader = std::io::Cursor::new(&sample);
    let mut read_buff: [u8; 15] = [0; 15];
    let mut reader = ReadReader::new(&mut mem_reader);
    match reader.read_all(&mut read_buff[0..0]) {
        Ok(()) => (),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[0..5]) {
        Ok(()) => assert_sequence(&read_buff[0..5], 5),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[5..9]) {
        Ok(()) => assert_sequence(&read_buff[0..9], 9),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[9..11]) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[9..10]) {
        Ok(()) => assert_sequence(&read_buff[0..10], 10),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[0..0]) {
        Ok(()) => (),
        _ => panic!("Unexpected error!"),
    }
    match reader.read_all(&mut read_buff[0..1]) {
        Err(ErrorKind::IOError(_)) => (),
        _ => panic!("Unexpected error!"),
    }
}

//=============================================================================
// WriteWriter
//-----------------------------------------------------------------------------
#[test]
fn test_writewriter_new() {
    let mut buff: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buff);
    let mut writer = WriteWriter::new(&mut cursor);

    match writer.write(0) {
        Ok(()) => (),
        _ => panic!("Unexpected error."),
    }
    assert_eq!(&buff, &[0 as u8])
}

#[test]
fn test_writewriter_write() {
    let mut sample: [u8; 10] = [0; 10];
    fill_sample(&mut sample);
    let mut buff: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buff);
    let mut writer = WriteWriter::new(&mut cursor);

    for x in &sample {
        match writer.write(*x) {
            Ok(()) => (),
            _ => panic!("Unexpected error."),
        }
    }
    assert_eq!(&buff, &sample)
}

#[test]
fn test_writewriter_write_all() {
    let mut sample: [u8; 10] = [0; 10];
    fill_sample(&mut sample);
    let mut buff: Vec<u8> = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buff);
    let mut writer = WriteWriter::new(&mut cursor);

    match writer.write_all(&sample[0..1]) {
        Ok(()) => (),
        _ => panic!("Unexpected error."),
    }
    match writer.write_all(&sample[1..9]) {
        Ok(()) => (),
        _ => panic!("Unexpected error."),
    }
    match writer.write_all(&sample[9..10]) {
        Ok(()) => (),
        _ => panic!("Unexpected error."),
    }
    assert_eq!(&buff, &sample)
}

//=============================================================================
// Reader for std::io::Read + std::io::Seek
//-----------------------------------------------------------------------------
#[test]
fn test_reader_for_read_seek_read() {
    let buff: [u8; 4] = [1, 2, 3, 4];
    let mut read = std::io::Cursor::new(&buff);

    assert_eq!(read.read().unwrap(), 1);
    assert_eq!(read.read().unwrap(), 2);
    assert_eq!(read.read().unwrap(), 3);
    assert_eq!(read.read().unwrap(), 4);
    assert!(matches!(read.read(), Err(ErrorKind::UnableToReadData)));
}

#[test]
fn test_reader_for_read_seek_read_all() {
    let buff: [u8; 4] = [1, 2, 3, 4];
    let mut r: [u8; 4] = [0; 4];

    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;

    read.read_all(&mut r).unwrap();
    assert_eq!(buff, r);
    read.read_all(&mut r[0..0]).unwrap();
    assert!(matches!(
        read.read_all(&mut r[0..1]),
        Err(ErrorKind::UnableToReadData)
    ));

    let mut r: [u8; 4] = [0; 4];
    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    read.read_all(&mut r[0..2]).unwrap();
    read.read_all(&mut r[2..4]).unwrap();
    assert_eq!(buff, r);
    assert!(matches!(
        read.read_all(&mut r[0..1]),
        Err(ErrorKind::UnableToReadData)
    ));

    let mut r: [u8; 5] = [0; 5];
    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    assert!(matches!(
        read.read_all(&mut r),
        Err(ErrorKind::UnableToReadData)
    ));
}

#[test]
fn test_reader_for_read_seek_skip() {
    let buff: [u8; 4] = [1, 2, 3, 4];

    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    read.skip(2).unwrap();
    assert_eq!(read.read().unwrap(), 3);
    read.skip(1).unwrap();
    assert!(matches!(read.skip(1), Err(ErrorKind::EndOfData)));

    let buff: [u8; 1] = [1];
    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    assert!(matches!(read.skip(2), Err(ErrorKind::UnableToReadData)));
}

#[test]
fn test_reader_for_read_seek_skip_u64() {
    let buff: [u8; 4] = [1, 2, 3, 4];

    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    read.skip_u64(2).unwrap();
    assert_eq!(read.read().unwrap(), 3);
    read.skip_u64(1).unwrap();
    assert!(matches!(read.skip_u64(1), Err(ErrorKind::EndOfData)));

    let buff: [u8; 1] = [1];
    let mut inner_read = std::io::Cursor::new(&buff);
    let read = &mut inner_read;
    assert!(matches!(read.skip_u64(2), Err(ErrorKind::UnableToReadData)));
}

//=============================================================================
// Writer for std::io::Write + std::io::Seek
//-----------------------------------------------------------------------------
#[test]
fn test_writer_write_seek_write() {
    let exp: [u8; 4] = [1, 2, 3, 4];
    let mut buff = Vec::<u8>::new();

    let mut inner_write = std::io::Cursor::new(&mut buff);
    let write = &mut inner_write;

    write.write(1).unwrap();
    write.write(2).unwrap();
    write.write(3).unwrap();
    write.write(4).unwrap();

    assert_eq!(buff.as_slice(), &exp);
}

#[test]
fn test_writer_write_seek_write_all() {
    let exp: [u8; 4] = [1, 2, 3, 4];
    let mut buff = Vec::<u8>::new();

    let mut inner_write = std::io::Cursor::new(&mut buff);
    let write = &mut inner_write;
    write.write_all(&exp).unwrap();
    assert_eq!(buff.as_slice(), &exp);
}

#[test]
fn test_writer_write_seek_as_writer() {
    let exp: [u8; 4] = [1, 2, 3, 4];
    let mut buff = Vec::<u8>::new();

    let mut inner_write = std::io::Cursor::new(&mut buff);
    let write = &mut inner_write;
    let w = write.as_writer();
    w.write_all(&exp).unwrap();
    assert_eq!(buff.as_slice(), &exp);
}
