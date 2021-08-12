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
//! This is module defines all standard tags and tag factories.
use super::{DefaultWithId, ErrorKind, ILTag, ILTagFactory, Result};

pub mod constants;
pub mod explicit;
pub mod factory;
pub mod implicit;

#[cfg(test)]
mod tests;

pub use constants::{
    IL_BDEC_TAG_ID, IL_BIN128_TAG_ID, IL_BIN32_TAG_ID, IL_BIN64_TAG_ID, IL_BINT_TAG_ID,
    IL_BOOL_TAG_ID, IL_BYTES_TAG_ID, IL_DICTIONARY_TAG_ID, IL_ILINTARRAY_TAG_ID, IL_ILINT_TAG_ID,
    IL_ILTAGARRAY_TAG_ID, IL_ILTAGSEQ_TAG_ID, IL_INT16_TAG_ID, IL_INT32_TAG_ID, IL_INT64_TAG_ID,
    IL_INT8_TAG_ID, IL_NULL_TAG_ID, IL_OID_TAG_ID, IL_RANGE_TAG_ID, IL_SIGNED_ILINT_TAG_ID,
    IL_STRING_DICTIONARY_TAG_ID, IL_STRING_TAG_ID, IL_UINT16_TAG_ID, IL_UINT32_TAG_ID,
    IL_UINT64_TAG_ID, IL_UINT8_TAG_ID, IL_VERSION_TAG_ID,
};
pub use explicit::{
    ILBigDecTag, ILBigIntTag, ILByteArrayTag, ILDictTag, ILILIntArrayTag, ILOIDTag, ILRangeTag,
    ILStrDictTag, ILStringTag, ILTagArrayTag, ILTagSeqTag, ILVersionTag,
};
pub use factory::ILStandardTagFactory;
pub use implicit::{
    ILBin128Tag, ILBin32Tag, ILBin64Tag, ILBoolTag, ILILInt64Tag, ILInt16Tag, ILInt32Tag,
    ILInt64Tag, ILInt8Tag, ILNullTag, ILSignedILInt64Tag, ILUInt16Tag, ILUInt32Tag, ILUInt64Tag,
    ILUInt8Tag,
};
