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
use crate::io::array::*;
use crate::tags::standard::*;
use crate::tags::*;

//=============================================================================
// Helper functions
//-----------------------------------------------------------------------------
fn serialize_tag_seq(tags: &Vec<Box<dyn ILTag>>) -> Vec<u8> {
    let mut writer = VecWriter::new();
    for t in tags {
        t.as_ref().serialize(&mut writer).unwrap();
    }
    writer.into()
}

//=============================================================================
// RawTagOffset
//-----------------------------------------------------------------------------
#[test]
fn test_rawragoffset_impl() {
    let t = RawTagOffset::new(3, 5, 7, 11);

    assert_eq!(t.id(), 3);
    assert_eq!(t.offset(), 5);
    assert_eq!(t.header_size, 7);
    assert_eq!(t.value_size(), 11);
    assert_eq!(t.size(), 7 + 11);
    assert_eq!(t.value_size(), 11);
    assert_eq!(t.value_offset(), 5 + 7);
    assert_eq!(t.next_tag_offset(), 5 + 7 + 11);
    assert_eq!(t.tag_start(), t.offset() as usize);
    assert_eq!(t.tag_end(), t.next_tag_offset() as usize);
    assert_eq!(t.value_start(), t.value_offset() as usize);
    assert_eq!(t.value_end(), t.next_tag_offset() as usize);
}

#[test]
fn test_rawragoffset_impl_to_parent_space() {
    // Best fit - child is the sole payload of parent
    let parent = RawTagOffset::new(3, 5, 7, 10);
    let child = RawTagOffset::new(4, 0, 2, 8);
    let m = child.map_to_parent_space(&parent).unwrap();
    assert_eq!(m.id, child.id);
    assert_eq!(m.header_size, child.header_size);
    assert_eq!(m.value_size, child.value_size);
    assert_eq!(m.offset, child.offset + parent.value_offset());
    assert_eq!(m.offset(), parent.value_offset());
    assert_eq!(m.next_tag_offset(), parent.next_tag_offset());

    // A little bit larger...
    let parent = RawTagOffset::new(3, 5, 7, 11);
    let child = RawTagOffset::new(4, 0, 2, 8);
    let m = child.map_to_parent_space(&parent).unwrap();
    assert_eq!(m.id, child.id);
    assert_eq!(m.header_size, child.header_size);
    assert_eq!(m.value_size, child.value_size);
    assert_eq!(m.offset, child.offset + parent.value_offset());
    assert_eq!(m.offset(), parent.value_offset());
    assert_eq!(m.next_tag_offset() + 1, parent.next_tag_offset());

    // Not at the begining
    let parent = RawTagOffset::new(3, 5, 7, 11);
    let child = RawTagOffset::new(4, 1, 2, 8);
    let m = child.map_to_parent_space(&parent).unwrap();
    assert_eq!(m.id, child.id);
    assert_eq!(m.header_size, child.header_size);
    assert_eq!(m.value_size, child.value_size);
    assert_eq!(m.offset, child.offset + parent.value_offset());
    assert_eq!(m.offset() - 1, parent.value_offset());
    assert_eq!(m.next_tag_offset(), parent.next_tag_offset());

    // Too large for the payload
    let parent = RawTagOffset::new(3, 5, 7, 10);
    let child = RawTagOffset::new(4, 0, 2, 9);
    assert!(matches!(
        child.map_to_parent_space(&parent),
        Err(ErrorKind::CorruptedData)
    ));

    // Same size but lands outside of the parent
    let parent = RawTagOffset::new(3, 5, 7, 10);
    let child = RawTagOffset::new(4, 1, 2, 8);
    assert!(matches!(
        child.map_to_parent_space(&parent),
        Err(ErrorKind::CorruptedData)
    ));
}

#[test]
fn test_rawragoffset_impl_value_slice() {
    let raw: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let p = RawTagOffset::new(3, 1, 2, 3);
    let s = p.tag_slice(raw.as_slice());
    assert_eq!(s, &raw[1..6]);
    let s = p.value_slice(raw.as_slice());
    assert_eq!(s, &raw[3..6]);
}

//=============================================================================
// Samples
//-----------------------------------------------------------------------------
macro_rules! vec_push_new_tag {
    ($v: expr, $tag_type: ty) => {
        $v.push(Box::new(<$tag_type>::new()));
    };
    ($v: expr, $tag_type: ty, $($next_tags: ty), +) => {
        vec_push_new_tag!($v, $tag_type);
        vec_push_new_tag!($v, $($next_tags), +);
    };
}

fn create_sample_tag_seq() -> Vec<Box<dyn ILTag>> {
    let mut resp = Vec::<Box<dyn ILTag>>::new();

    vec_push_new_tag!(
        resp,
        ILBin128Tag,
        ILBin32Tag,
        ILBin64Tag,
        ILBoolTag,
        ILILInt64Tag,
        ILInt16Tag,
        ILInt32Tag,
        ILInt64Tag,
        ILInt8Tag,
        ILNullTag,
        ILSignedILInt64Tag,
        ILUInt16Tag,
        ILUInt32Tag,
        ILUInt64Tag,
        ILUInt8Tag
    );
    resp.push(Box::new(ILStringTag::with_value("tag1")));
    resp.push(Box::new(ILStringTag::with_id_value(12345, "tag2")));
    resp
}

#[test]
fn test_create_sample_tag_seq() {
    let tags = create_sample_tag_seq();
    assert_eq!(tags.len(), 17);
}

#[test]
fn test_serialize_tag_seq() {
    let tags = create_sample_tag_seq();

    let serialized = serialize_tag_seq(&tags);

    let mut size = 0 as u64;
    for t in tags {
        size += t.size();
    }
    assert_eq!(serialized.len() as u64, size);
}

//=============================================================================
// RawTagScanner
//-----------------------------------------------------------------------------
#[test]
fn test_rawragscanner_next_tag() {
    let tags = create_sample_tag_seq();
    let serialized = serialize_tag_seq(&tags);
    let mut reader = ByteArrayReader::new(serialized.as_slice());

    let mut scanner = RawTagScanner::new(&mut reader);
    let mut offs = 0 as u64;
    for exp in tags {
        // Collect the values
        let next_offs = offs + exp.size();

        // Check the scanner
        let t = scanner.next_tag().unwrap().unwrap();
        assert_eq!(t.id(), exp.id());
        assert_eq!(t.offset(), offs);
        assert_eq!(t.next_tag_offset(), next_offs);
        assert_eq!(t.value_size(), t.value_size());

        offs = next_offs;
    }
    assert_eq!(serialized.len() as u64, offs);
}

#[test]
fn test_rawragscanner_next_tag_if_id() {
    let tags = create_sample_tag_seq();
    let serialized = serialize_tag_seq(&tags);
    let mut reader = ByteArrayReader::new(serialized.as_slice());

    let mut scanner = RawTagScanner::new(&mut reader);
    let mut offs = 0 as u64;
    for exp in tags {
        // Collect the values
        let next_offs = offs + exp.size();

        // Check the scanner
        let t = scanner.next_tag_if_id(exp.id()).unwrap().unwrap();
        assert_eq!(t.id(), exp.id());
        assert_eq!(t.offset(), offs);
        assert_eq!(t.next_tag_offset(), next_offs);
        assert_eq!(t.value_size(), t.value_size());

        offs = next_offs;
    }
    assert_eq!(serialized.len() as u64, offs);

    // Force an error
    let mut reader = ByteArrayReader::new(serialized.as_slice());
    let mut scanner = RawTagScanner::new(&mut reader);
    assert!(matches!(
        scanner.next_tag_if_id(1313123),
        Err(ErrorKind::UnexpectedTagType)
    ));
}
