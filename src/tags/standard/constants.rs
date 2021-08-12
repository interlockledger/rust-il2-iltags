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
//! Definition of all standard tag constants.

/// Standard null tag ID.
pub const IL_NULL_TAG_ID: u64 = 0;

/// Standard bool tag ID.
pub const IL_BOOL_TAG_ID: u64 = 1;

/// Standard signed 8-bit integer tag ID.
pub const IL_INT8_TAG_ID: u64 = 2;

/// Standard unsigned 8-bit integer tag ID.
pub const IL_UINT8_TAG_ID: u64 = 3;

/// Standard signed 16-bit integer tag ID.
pub const IL_INT16_TAG_ID: u64 = 4;

/// Standard unsigned 16-bit integer tag ID.
pub const IL_UINT16_TAG_ID: u64 = 5;

/// Standard signed 32-bit integer tag ID.
pub const IL_INT32_TAG_ID: u64 = 6;

/// Standard unsigned 32-bit integer tag ID.
pub const IL_UINT32_TAG_ID: u64 = 7;

/// Standard signed 64-bit integer tag ID.
pub const IL_INT64_TAG_ID: u64 = 8;

/// Standard unsigned 64-bit integer tag ID.
pub const IL_UINT64_TAG_ID: u64 = 9;

/// Standard ILInt tag ID.
pub const IL_ILINT_TAG_ID: u64 = 10;

/// Standard 32-bit floating point tag ID.
pub const IL_BIN32_TAG_ID: u64 = 11;

/// Standard 64-bit floating point tag ID.
pub const IL_BIN64_TAG_ID: u64 = 12;

/// Standard 128-bit floating point tag ID.
pub const IL_BIN128_TAG_ID: u64 = 13;

/// Standard Signed ILInt tag ID.
///
/// New since 1.3.0.
pub const IL_SIGNED_ILINT_TAG_ID: u64 = 14;

/// Standard byte array tag ID.
pub const IL_BYTES_TAG_ID: u64 = 16;

/// Standard string tag ID.
pub const IL_STRING_TAG_ID: u64 = 17;

/// Standard big integer tag ID.
pub const IL_BINT_TAG_ID: u64 = 18;

/// Standard big decimal tag ID.
pub const IL_BDEC_TAG_ID: u64 = 19;

/// Standard ILInt array tag ID.
pub const IL_ILINTARRAY_TAG_ID: u64 = 20;

/// Standard ILTag array tag ID.
pub const IL_ILTAGARRAY_TAG_ID: u64 = 21;

/// Standard ILTag sequence tag ID.
pub const IL_ILTAGSEQ_TAG_ID: u64 = 22;

/// Standard range tag ID.
pub const IL_RANGE_TAG_ID: u64 = 23;

/// Standard version tag ID.
pub const IL_VERSION_TAG_ID: u64 = 24;

/// Standard OID tag ID.
pub const IL_OID_TAG_ID: u64 = 25;

/// Standard dictionary tag ID.
pub const IL_DICTIONARY_TAG_ID: u64 = 30;

/// Standard string-only dictionary tag ID.
pub const IL_STRING_DICTIONARY_TAG_ID: u64 = 31;
