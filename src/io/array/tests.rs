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

//=============================================================================
// DummyMemoryReader
//-----------------------------------------------------------------------------
struct DummyMemoryReader {
    offset: usize,
    len: usize,
}

impl DummyMemoryReader {
    pub fn new(offset: usize, len: usize) -> Self {
        Self { offset, len }
    }
}

impl Reader for DummyMemoryReader {
    fn read(&mut self) -> Result<u8> {
        self.assert_can_read(1)?;
        let r = self.offset as u8;
        self.offset += 1;
        Ok(r)
    }
}

impl MemoryReader for DummyMemoryReader {
    fn len(&self) -> usize {
        self.len
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) {
        self.offset = std::cmp::min(self.len, offset);
    }
}

//=============================================================================
// MemoryReader
//-----------------------------------------------------------------------------
#[test]
fn test_memoryreader() {
    let r = DummyMemoryReader::new(0, 0);
    assert!(r.is_empty());
    assert_eq!(r.available(), 0);
    r.assert_can_read(0).unwrap();
    assert!(matches!(r.assert_can_read(3), Err(ErrorKind::EndOfData)));

    let r = DummyMemoryReader::new(0, 2);
    assert!(!r.is_empty());
    assert_eq!(r.available(), 2);
    r.assert_can_read(0).unwrap();
    r.assert_can_read(1).unwrap();
    r.assert_can_read(2).unwrap();
    assert!(matches!(
        r.assert_can_read(3),
        Err(ErrorKind::UnableToReadData)
    ));

    let r = DummyMemoryReader::new(2, 2);
    assert!(!r.is_empty());
    assert_eq!(r.available(), 0);
    r.assert_can_read(0).unwrap();
    assert!(matches!(r.assert_can_read(3), Err(ErrorKind::EndOfData)));
}

//=============================================================================
// ByteArrayReader
//-----------------------------------------------------------------------------
//Tests for ByteArrayReader
#[test]
fn test_bytearrayreader_new() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    assert_eq!(ba.offset(), 0);
    assert_eq!(ba.as_slice(), src);

    ba = ByteArrayReader::new(&src[0..1]);
    assert_eq!(ba.offset(), 0);
    assert_eq!(ba.as_slice(), &src[0..1]);
}

#[test]
fn test_bytearrayreader_get_set_offset() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    for i in 0..src.len() + 1 {
        ba.set_offset(i);
        assert_eq!(ba.offset(), i);
    }

    ba.set_offset(ba.as_slice().len() + 1);
    assert_eq!(ba.offset(), ba.as_slice().len());

    ba.set_offset(usize::MAX);
    assert_eq!(ba.offset(), ba.as_slice().len());
}

#[test]
fn test_bytearrayreader_as_slice() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let ba = ByteArrayReader::new(&src);
    assert_eq!(ba.as_slice(), &src);
}

#[test]
fn test_bytearrayreader_available() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    for i in 0..src.len() {
        assert_eq!(ba.available(), src.len() - i);
        assert!(ba.read().is_ok());
        assert_eq!(ba.available(), src.len() - i - 1);
    }
}

#[test]
fn test_bytearrayreader_can_read() {
    let src = [0u8; 8];
    let ba = ByteArrayReader::new(&src);

    for size in 0..src.len() + 1 {
        assert!(ba.can_read(size).is_ok());
    }
    match ba.can_read(src.len() + 2) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!(),
    };
}

#[test]
fn test_bytearrayreader_reader_read() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    for i in 0..src.len() {
        match ba.read() {
            Ok(v) => assert_eq!(v, src[i]),
            _ => panic!("Unexpected read error!"),
        }
    }
    assert!(ba.read().is_err());
}

#[test]
fn test_bytearrayreader_reader_readall() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let mut ba = ByteArrayReader::new(&src);
    let mut offs: usize = 0;
    let mut buff: [u8; 6] = [0; 6];
    for l in 0..7 {
        let count = if l <= ba.available() {
            l
        } else {
            ba.available()
        };
        let slice = &mut buff[0..count];
        assert!(ba.read_all(slice).is_ok());
        assert_eq!(&src[offs..(offs + count)], slice);
        offs += count;
    }
}

#[test]
fn test_bytearrayreader_skip() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let mut ba = ByteArrayReader::new(&src);
    let mut offs: usize = 0;
    for size in 0..6 {
        assert!(ba.skip(size).is_ok());
        offs += size;
        assert_eq!(ba.offset(), offs);
        assert_eq!(ba.available(), 20 - offs);
    }
    match ba.skip(6) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!(),
    }
    match ba.skip(5) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(ba.offset(), 20);
    match ba.skip(0) {
        Ok(()) => (),
        _ => panic!(),
    }
    match ba.skip(1) {
        Err(ErrorKind::EndOfData) => (),
        _ => panic!(),
    }
}

//=============================================================================
// VecReader
//-----------------------------------------------------------------------------
#[test]
fn test_vecreader_new() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let r = VecReader::new(&src);
    assert_eq!(r.as_slice(), &src);
    assert_eq!(r.offset(), 0);
}

#[test]
fn test_vecreader_get_set_offset() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let mut ba = VecReader::new(&src);

    for i in 0..src.len() + 1 {
        ba.set_offset(i);
        assert_eq!(ba.offset(), i);
    }

    ba.set_offset(ba.as_slice().len() + 1);
    assert_eq!(ba.offset(), ba.as_slice().len());

    ba.set_offset(usize::MAX);
    assert_eq!(ba.offset(), ba.as_slice().len());
}

#[test]
fn test_vecreader_available() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = VecReader::new(&src);

    for i in 0..src.len() {
        assert_eq!(ba.available(), src.len() - i);
        assert!(ba.read().is_ok());
        assert_eq!(ba.available(), src.len() - i - 1);
    }
}

#[test]
fn test_vecreader_can_read() {
    let src = [0u8; 8];
    let ba = VecReader::new(&src);

    for size in 0..src.len() + 1 {
        assert!(ba.can_read(size).is_ok());
    }
    match ba.can_read(src.len() + 2) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!(),
    };
}

#[test]
fn test_vecreader_get_vec() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let ba = VecReader::new(&src);

    assert_eq!(ba.as_slice(), &src);
}

#[test]
fn test_vecreader_reader_read() {
    let mut src = [0u8; 8];
    fill_sample(&mut src);
    let mut ba = VecReader::new(&src);
    for i in 0..src.len() {
        match ba.read() {
            Ok(v) => assert_eq!(v, src[i]),
            _ => panic!("Unexpected read error!"),
        }
    }
    assert!(ba.read().is_err());
}

#[test]
fn test_vecreader_reader_readall() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let mut ba = VecReader::new(&src);
    let mut offs: usize = 0;
    let mut buff: [u8; 6] = [0; 6];
    for l in 0..7 {
        let count = if l <= ba.available() {
            l
        } else {
            ba.available()
        };
        let slice = &mut buff[0..count];
        assert!(ba.read_all(slice).is_ok());
        assert_eq!(&src[offs..(offs + count)], slice);
        offs += count;
    }
}

#[test]
fn test_vecreader_skip() {
    let mut src: [u8; 20] = [0; 20];
    fill_sample(&mut src);

    let mut ba = VecReader::new(&src);
    let mut offs: usize = 0;
    for size in 0..6 {
        assert!(ba.skip(size).is_ok());
        offs += size;
        assert_eq!(ba.offset(), offs);
        assert_eq!(ba.available(), 20 - offs);
    }
    match ba.skip(6) {
        Err(ErrorKind::UnableToReadData) => (),
        _ => panic!(),
    }
    match ba.skip(5) {
        Ok(()) => (),
        _ => panic!(),
    }
    assert_eq!(ba.offset(), 20);
    match ba.skip(0) {
        Ok(()) => (),
        _ => panic!(),
    }
    match ba.skip(1) {
        Err(ErrorKind::EndOfData) => (),
        _ => panic!(),
    }
}

//=============================================================================
// VecWriter
//-----------------------------------------------------------------------------
#[test]
fn test_vecwriter_new() {
    let w = VecWriter::new();

    assert!(!w.is_read_only());
    assert_eq!(w.get_offset(), 0);
    assert_eq!(w.vec().len(), 0);
}

#[test]
fn test_vecwriter_with_capacity() {
    let w = VecWriter::with_capacity(123);

    assert!(!w.is_read_only());
    assert_eq!(w.get_offset(), 0);
    assert_eq!(w.vec().len(), 0);
    assert_eq!(w.vec().capacity(), 123);
}

#[test]
fn test_vecwriter_get_set_read_only() {
    let mut w = VecWriter::new();

    assert!(!w.is_read_only());
    w.set_read_only(true);
    assert!(w.is_read_only());
    w.set_read_only(false);
    assert!(!w.is_read_only());
}

#[test]
fn test_vecwriter_can_write() {
    let mut w = VecWriter::new();

    match w.can_write() {
        Ok(()) => (),
        _ => panic!(),
    }
    w.set_read_only(true);
    match w.can_write() {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }
    w.set_read_only(false);
    match w.can_write() {
        Ok(()) => (),
        _ => panic!(),
    }
}

#[test]
fn test_vecwriter_as_slice() {
    let mut src: [u8; 10] = [0; 10];
    fill_sample(&mut src);

    let mut w = VecWriter::new();

    assert_eq!(w.as_slice().len(), 0);
    for s in &src {
        match w.write(*s) {
            Ok(()) => (),
            _ => panic!(),
        }
    }

    assert_eq!(w.as_slice().len(), src.len());
    assert_eq!(w.as_slice(), &src);
}

#[test]
fn test_vecwriter_get_vec() {
    let mut src: [u8; 10] = [0; 10];
    fill_sample(&mut src);

    let mut w = VecWriter::new();

    assert_eq!(w.vec().len(), 0);
    for s in &src {
        match w.write(*s) {
            Ok(()) => (),
            _ => panic!(),
        }
    }
    assert_eq!(w.vec().len(), src.len());
    assert_eq!(w.vec().as_slice(), &src);
}

#[test]
fn test_vecwriter_writer_write() {
    let mut baw = VecWriter::new();

    let mut src2 = [0u8; 8];
    fill_sample(&mut src2);

    for sample in src2.iter() {
        match baw.write(*sample) {
            Ok(_) => (),
            _ => panic!("Unexpected read_all error!"),
        }
    }
    assert_eq!(&src2, baw.as_slice());
}

#[test]
fn test_vecwriter_writer_write_all() {
    let mut baw = VecWriter::new();

    let mut src2 = [0u8; 8];
    fill_sample(&mut src2);

    match baw.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }
    match baw.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }

    let mut exp = [0u8; 16];
    exp[0..8].copy_from_slice(&src2);
    exp[8..16].copy_from_slice(&src2);
    assert_eq!(exp, baw.as_slice());
}

//=============================================================================
// BorrowedVecWriter
//-----------------------------------------------------------------------------
#[test]
fn test_borrowedvecwriter_new() {
    let mut vec: Vec<u8> = Vec::new();

    let w = BorrowedVecWriter::new(&mut vec);
    assert!(!w.is_read_only());
    assert_eq!(w.get_offset(), 0);
    assert_eq!(w.bytes_written(), 0);

    vec.push(0);
    vec.push(1);
    let w = BorrowedVecWriter::with_offset(&mut vec, 1);
    assert!(!w.is_read_only());
    assert_eq!(w.get_offset(), 1);
    assert_eq!(w.bytes_written(), 0);

    let w = BorrowedVecWriter::with_offset(&mut vec, 1000);
    assert!(!w.is_read_only());
    assert_eq!(w.get_offset(), 2);
    assert_eq!(w.bytes_written(), 0);
}

#[test]
fn test_borrowedvecwriter_get_set_read_only() {
    let mut vec: Vec<u8> = Vec::new();
    let mut w = BorrowedVecWriter::new(&mut vec);

    assert!(!w.is_read_only());
    w.set_read_only(true);
    assert!(w.is_read_only());
    w.set_read_only(false);
    assert!(!w.is_read_only());
}

#[test]
fn test_borrowedvecwriter_can_write() {
    let mut vec: Vec<u8> = Vec::new();
    let mut w = BorrowedVecWriter::new(&mut vec);

    match w.can_write() {
        Ok(()) => (),
        _ => panic!(),
    }
    w.set_read_only(true);
    match w.can_write() {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }
    w.set_read_only(false);
    match w.can_write() {
        Ok(()) => (),
        _ => panic!(),
    }
}

#[test]
fn test_borrowedvecwriter_get_vec() {
    let mut src: [u8; 10] = [0; 10];
    fill_sample(&mut src);

    let mut vec: Vec<u8> = Vec::new();
    let mut w = BorrowedVecWriter::new(&mut vec);

    assert_eq!(w.vec().len(), 0);
    for s in &src {
        match w.write(*s) {
            Ok(()) => (),
            _ => panic!(),
        }
    }
    assert_eq!(w.vec().len(), src.len());
    assert_eq!(w.vec().as_slice(), &src);
}

#[test]
fn test_borrowedvecwriter_writer_write() {
    // Default
    let mut vec: Vec<u8> = Vec::new();
    let mut w = BorrowedVecWriter::new(&mut vec);

    let mut src2 = [0u8; 8];
    fill_sample(&mut src2);

    let mut count: usize = 0;
    for sample in src2.iter() {
        match w.write(*sample) {
            Ok(_) => (),
            _ => panic!("Unexpected read_all error!"),
        }
        count += 1;
        assert_eq!(w.bytes_written(), count);
    }
    assert_eq!(&src2, w.as_slice());

    // Append
    let mut vec: Vec<u8> = Vec::new();
    vec.push(0xFF);
    let mut w = BorrowedVecWriter::with_offset(&mut vec, 1);

    let mut src2 = [0u8; 8];
    fill_sample(&mut src2);

    let mut count: usize = 0;
    for sample in src2.iter() {
        match w.write(*sample) {
            Ok(_) => (),
            _ => panic!("Unexpected read_all error!"),
        }
        count += 1;
        assert_eq!(w.bytes_written(), count);
    }
    assert_eq!(&src2, &w.as_slice()[1..]);
    assert_eq!(0xFF, vec[0]);
}

#[test]
fn test_borrowedvecwriter_writer_write_all() {
    let mut vec: Vec<u8> = Vec::new();
    let mut w = BorrowedVecWriter::new(&mut vec);

    let mut src2 = [0u8; 8];
    fill_sample(&mut src2);

    let mut count = 0 as usize;
    match w.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }
    count += src2.len();
    assert_eq!(w.bytes_written(), count);

    match w.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }
    count += src2.len();
    assert_eq!(w.bytes_written(), count);

    let mut exp = [0u8; 16];
    exp[0..8].copy_from_slice(&src2);
    exp[8..16].copy_from_slice(&src2);
    assert_eq!(exp, w.as_slice());

    // With a vector with a given size
    let mut vec: Vec<u8> = Vec::new();
    vec.resize(10, 0);
    let mut w = BorrowedVecWriter::new(&mut vec);

    let mut count = 0 as usize;
    match w.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }
    count += src2.len();
    assert_eq!(w.bytes_written(), count);

    match w.write_all(&src2) {
        Ok(_) => (),
        _ => panic!("Unexpected read_all error!"),
    }
    count += src2.len();
    assert_eq!(w.bytes_written(), count);

    let mut exp = [0u8; 16];
    exp[0..8].copy_from_slice(&src2);
    exp[8..16].copy_from_slice(&src2);
    assert_eq!(exp, w.as_slice());
}

//=============================================================================
// ByteArrayWriter
//-----------------------------------------------------------------------------

#[test]
fn test_bytearraywriter_impl_new() {
    let mut backend: [u8; 16] = [0; 16];

    let w = ByteArrayWriter::new(&mut backend);
    assert_eq!(w.get_offset(), 0);
    assert_eq!(w.available(), 16);
    drop(w);
}

#[test]
fn test_bytearraywriter_impl_offset_available() {
    let mut backend: [u8; 16] = [0; 16];

    let mut w = ByteArrayWriter::new(&mut backend);
    assert_eq!(w.get_offset(), 0);
    assert_eq!(w.available(), 16);
    w.set_offset(1);
    assert_eq!(w.get_offset(), 1);
    assert_eq!(w.available(), 15);

    w.set_offset(16);
    assert_eq!(w.get_offset(), 16);
    assert_eq!(w.available(), 0);

    w.set_offset(17);
    assert_eq!(w.get_offset(), 16);
    assert_eq!(w.available(), 0);
}

#[test]
fn test_bytearraywriter_impl_can_write() {
    let mut backend: [u8; 16] = [0; 16];

    let mut w = ByteArrayWriter::new(&mut backend);
    w.can_write(0).unwrap();
    w.can_write(1).unwrap();
    w.can_write(16).unwrap();
    match w.can_write(17) {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }

    w.set_offset(10);
    w.can_write(0).unwrap();
    w.can_write(1).unwrap();
    w.can_write(6).unwrap();
    match w.can_write(7) {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }

    w.set_offset(16);
    w.can_write(0).unwrap();
    match w.can_write(7) {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }
}

#[test]
fn test_bytearraywriter_writer_write() {
    let mut backend: [u8; 16] = [0; 16];
    let mut exp: [u8; 16] = [0; 16];

    let mut w = ByteArrayWriter::new(&mut backend);
    for i in 0..16 as u8 {
        assert_eq!(w.get_offset(), i as usize);
        w.write(i).unwrap();
        exp[i as usize] = i;
        assert_eq!(w.get_offset(), (i + 1) as usize)
    }
    match w.write(17) {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }
    drop(w);
    assert_eq!(&backend, &exp);
}

#[test]
fn test_bytearraywriter_writer_all() {
    let mut backend: [u8; 16] = [0; 16];
    let exp: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    let mut w = ByteArrayWriter::new(&mut backend);
    assert_eq!(w.get_offset(), 0);
    w.write_all(&exp[0..1]).unwrap();
    assert_eq!(w.get_offset(), 1);

    w.write_all(&exp[1..5]).unwrap();
    assert_eq!(w.get_offset(), 5);

    w.write_all(&exp[5..16]).unwrap();
    assert_eq!(w.get_offset(), 16);
    match w.write_all(&exp[0..1]) {
        Err(ErrorKind::UnableToWriteData) => (),
        _ => panic!(),
    }
    drop(w);
    assert_eq!(&backend, &exp);
}
