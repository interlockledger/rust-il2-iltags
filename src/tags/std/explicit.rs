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
//! This module implements all standard explicit tags defined by
//! [ILTags Specification](https://github.com/interlockledger/specification/tree/master/ILTags).
use super::constants::*;
use super::{DefaultWithId, ErrorKind, ILTag, ILTagFactory, Result};
use crate::base_iltag_impl;
use crate::iltag_default_impl;
use crate::io::data::*;
use crate::io::{Reader, Writer};
use ::std::any::Any;

macro_rules! std_byte_array_tag_impl {
    ($tag_type:ty, $tag_id: expr) => {
        impl $tag_type {
            /// Creates a new instance of this struct.
            pub fn new() -> Self {
                Self { value: Vec::new() }
            }

            /// Creates a new instance of this struct with a given capacity.
            ///
            /// Arguments:
            /// * `capacity`: The expected initial capacity;
            pub fn with_capacity(capacity: usize) -> Self {
                Self {
                    value: Vec::with_capacity(capacity),
                }
            }

            /// Creates a new instance of this struct with the
            /// specified initial value.
            ///
            /// Arguments:
            /// * `value`: A byte slice with the initial value;
            pub fn with_value(value: &[u8]) -> Self {
                let mut v: Vec<u8> = Vec::with_capacity(value.len());
                v.extend_from_slice(value);
                Self { value: v }
            }

            /// Returns an immutable reference to the value.
            pub fn value(&self) -> &Vec<u8> {
                &self.value
            }

            /// Returns a mutable reference to the value.
            pub fn mut_value(&mut self) -> &mut Vec<u8> {
                &mut self.value
            }
        }
    };
}

macro_rules! std_byte_array_tag_iltag_impl {
    ($tag_type:ty, $tag_id: expr) => {
        impl ILTag for $tag_type {
            fn id(&self) -> u64 {
                $tag_id
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_mut_any(&mut self) -> &mut dyn Any {
                self
            }

            fn value_size(&self) -> u64 {
                self.value.len() as u64
            }

            fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
                match writer.write_all(self.value.as_slice()) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(ErrorKind::IOError(e)),
                }
            }

            fn deserialize_value(
                &mut self,
                _factory: &dyn ILTagFactory,
                value_size: usize,
                reader: &mut dyn Reader,
            ) -> Result<()> {
                self.value.resize(value_size, 0);
                match reader.read_all(self.value.as_mut_slice()) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(ErrorKind::IOError(e)),
                }
            }
        }
    };
}

//=============================================================================
// ILByteArrayTag
//-----------------------------------------------------------------------------
/// This struct the standard byte array tag.
pub struct ILByteArrayTag {
    value: Vec<u8>,
}

std_byte_array_tag_impl!(ILByteArrayTag, IL_BYTES_TAG_ID);

std_byte_array_tag_iltag_impl!(ILByteArrayTag, IL_BYTES_TAG_ID);

impl Default for ILByteArrayTag {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// ILBigIntTag
//-----------------------------------------------------------------------------
/// This struct the standard big integer tag.
pub struct ILBigIntTag {
    value: Vec<u8>,
}

std_byte_array_tag_impl!(ILBigIntTag, IL_BINT_TAG_ID);

std_byte_array_tag_iltag_impl!(ILBigIntTag, IL_BINT_TAG_ID);

impl Default for ILBigIntTag {
    fn default() -> Self {
        Self::new()
    }
}
