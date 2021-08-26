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

fn serialize_tag_seq(tags: &Vec<Box<dyn ILTag>>) -> Vec<u8> {
    let mut writer = VecWriter::new();
    for t in tags {
        t.as_ref().serialize(&mut writer).unwrap();
    }
    writer.into()
}

fn serialize_tag(tag: &dyn ILTag) -> (Vec<u8>, Vec<u8>) {
    let mut writer = VecWriter::new();
    let mut value = VecWriter::new();

    tag.serialize(&mut writer).unwrap();
    tag.serialize_value(&mut value).unwrap();
    (writer.into(), value.into())
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
fn test_rawragscanner() {
    let tags = create_sample_tag_seq();
    let serialized = serialize_tag_seq(&tags);
    let mut reader = ByteArrayReader::new(serialized.as_slice());

    let mut scanner = RawTagScanner::new(&mut reader);
    let mut offs = 0 as u64;
    for exp in tags {
        // Collect the values
        let next_offs = offs + exp.size();

        // Check the scanner
        let t = scanner.find_next().unwrap().unwrap();
        assert_eq!(t.id(), exp.id());
        assert_eq!(t.offset(), offs);
        assert_eq!(t.next_tag_offset(), next_offs);
        assert_eq!(t.value_size(), t.value_size());

        offs = next_offs;
    }
    assert_eq!(serialized.len() as u64, offs);
}
