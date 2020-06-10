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
use super::super::tests::fill_sample;

//Tests for ByteArrayReader
#[test]
fn test_bytearrayreader_new() {
    let mut src= [0u8; 8];
    
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    
    assert_eq!(ba.get_offset(), 0);
    assert_eq!(ba.get_array(), src);

    ba = ByteArrayReader::new(&src[0..1]);
    assert_eq!(ba.get_offset(), 0);
    assert_eq!(ba.get_array(), &src[0..1]);
}

#[test]
fn test_bytearrayreader_get_set_offset() {
    let mut src= [0u8; 8];
    
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    
    for i in 0..src.len() {
        ba.set_offset(i);
        assert_eq!(ba.get_offset(), i);
    }
 }

#[test]
fn test_bytearrayreader_reader_read() {
    let mut src= [0u8; 8];
    
    fill_sample(&mut src);
    let mut ba = ByteArrayReader::new(&src);
    
    for i in 0..src.len() {
        match ba.read() {
            Ok(v) => assert_eq!(v, src[i]),
            _ => panic!("Unexpected read error!")
        }
    }
    assert!(ba.read().is_err());
 }

#[test]
fn test_bytearrayreader_reader_readall() {
    let  src:[u8;8] = [0; 8];

    let mut ba = ByteArrayReader::new( &src);

    let mut buffer:[u8;8] = [0; 8];

    match ba.read_all(&mut buffer) {
        Ok(v) => assert_eq!(v, ()),
        _ => panic!("Unexpected read_all error!")
    }
}

//Tests for ByteArrayWriter
#[test]
fn test_bytearraywriter_new() {
    let mut src1:[u8;8] = [0; 8];
    let mut src2= src1;
    let mut ba1 = ByteArrayWriter::new( &mut src1);
    assert_eq!(ba1.get_offset(), 0);
    assert_eq!(ba1.get_array(), &mut src2);

    let mut src3:[u8;8] = [0; 8];
    for i in 0..src3.len() {
        src3[i] = i as u8;
    }
    let mut src4= src3;
    let mut ba2 = ByteArrayWriter::new(&mut src3[0..1]);
    assert_eq!(ba2.get_offset(), 0);
    assert_eq!(ba2.get_array(), &mut src4[0..1]);
}

#[test]
fn test_bytearraywriter_get_set_offset() {
    let mut src= [0u8; 8];
    let src2 = src;
    fill_sample(&mut src);

    let mut ba = ByteArrayWriter::new(&mut src);

    for i in 0..src2.len() {
        ba.set_offset(i);
        assert_eq!(ba.get_offset(), i);
    }
}

#[test]
fn test_bytearraywriter_writer_write() {
    let mut src= [0u8; 8];
    let mut baw = ByteArrayWriter::new(&mut src);

    let mut src2= [0u8; 8];
    fill_sample(&mut src2);

    for i in 0..src2.len() {
        match baw.write(src2[i]) {
            Ok(_0) => assert_eq!(src2[i], baw.array[i]),
            _ => panic!("Unexpected read error!")
        }
    }
}

#[test]
fn test_bytearraywriter_writer_writeall() {
    let mut src= [0u8; 8];
    let mut baw = ByteArrayWriter::new(&mut src);

    let mut src2= [0u8; 8];
    fill_sample(&mut src2);

    match baw.write_all(&mut src2) {
        Ok(_0) => assert_eq!(src2, baw.get_array()),
        _ => panic!("Unexpected read_all error!")
    }
}