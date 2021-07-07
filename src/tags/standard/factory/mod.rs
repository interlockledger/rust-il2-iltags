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
#[cfg(test)]
mod tests;

use super::constants::{
    IL_BDEC_TAG_ID, IL_BIN128_TAG_ID, IL_BIN32_TAG_ID, IL_BIN64_TAG_ID, IL_BINT_TAG_ID,
    IL_BOOL_TAG_ID, IL_BYTES_TAG_ID, IL_DICTIONARY_TAG_ID, IL_ILINTARRAY_TAG_ID, IL_ILINT_TAG_ID,
    IL_ILTAGARRAY_TAG_ID, IL_ILTAGSEQ_TAG_ID, IL_INT16_TAG_ID, IL_INT32_TAG_ID, IL_INT64_TAG_ID,
    IL_INT8_TAG_ID, IL_NULL_TAG_ID, IL_OID_TAG_ID, IL_RANGE_TAG_ID, IL_STRING_DICTIONARY_TAG_ID,
    IL_STRING_TAG_ID, IL_UINT16_TAG_ID, IL_UINT32_TAG_ID, IL_UINT64_TAG_ID, IL_UINT8_TAG_ID,
    IL_VERSION_TAG_ID,
};
use super::explicit::{
    ILBigDecTag, ILBigIntTag, ILByteArrayTag, ILDictTag, ILILIntArrayTag, ILOIDTag, ILRangeTag,
    ILStrDictTag, ILStringTag, ILTagArrayTag, ILTagSeqTag, ILVersionTag,
};
use super::implicit::{
    implicit_tag_size, ILBin128Tag, ILBin32Tag, ILBin64Tag, ILBoolTag, ILILInt64Tag, ILInt16Tag,
    ILInt32Tag, ILInt64Tag, ILInt8Tag, ILNullTag, ILUInt16Tag, ILUInt32Tag, ILUInt64Tag,
    ILUInt8Tag,
};
use crate::io::{LimitedReader, Reader};
use crate::tags::{
    deserialize_ilint, is_implicit_tag, tag_size_to_usize, ErrorKind, ILDefaultTagCreator, ILTag,
    ILTagCreatorEngine, ILTagFactory, Result,
};

macro_rules! engine_register_macro {
    ($engine: ident, $tag_id: expr, $tag_type: ty) => {
        $engine.register(
            $tag_id,
            Box::new(ILDefaultTagCreator::<$tag_type>::default()),
        );
    };
}

/// This function crates a new ILTagCreatorEngine with all standard
/// tags already registered.
///
/// Arguments:
/// - `strict`: Defines if the ILTagCreatorEngine will be created in strict mode or not;
///
/// Returns:
/// - The new instance of ILTagCreatorEngine with all standard tags already registered.
pub fn create_std_engine(strict: bool) -> ILTagCreatorEngine {
    let mut engine = ILTagCreatorEngine::new(strict);
    // Implicit
    engine_register_macro!(engine, IL_NULL_TAG_ID, ILNullTag);
    engine_register_macro!(engine, IL_BOOL_TAG_ID, ILBoolTag);
    engine_register_macro!(engine, IL_INT8_TAG_ID, ILInt8Tag);
    engine_register_macro!(engine, IL_UINT8_TAG_ID, ILUInt8Tag);
    engine_register_macro!(engine, IL_INT16_TAG_ID, ILInt16Tag);
    engine_register_macro!(engine, IL_UINT16_TAG_ID, ILUInt16Tag);
    engine_register_macro!(engine, IL_INT32_TAG_ID, ILInt32Tag);
    engine_register_macro!(engine, IL_UINT32_TAG_ID, ILUInt32Tag);
    engine_register_macro!(engine, IL_INT64_TAG_ID, ILInt64Tag);
    engine_register_macro!(engine, IL_UINT64_TAG_ID, ILUInt64Tag);
    engine_register_macro!(engine, IL_ILINT_TAG_ID, ILILInt64Tag);
    engine_register_macro!(engine, IL_BIN32_TAG_ID, ILBin32Tag);
    engine_register_macro!(engine, IL_BIN64_TAG_ID, ILBin64Tag);
    engine_register_macro!(engine, IL_BIN128_TAG_ID, ILBin128Tag);
    // Standard
    engine_register_macro!(engine, IL_BYTES_TAG_ID, ILByteArrayTag);
    engine_register_macro!(engine, IL_STRING_TAG_ID, ILStringTag);
    engine_register_macro!(engine, IL_BINT_TAG_ID, ILBigIntTag);
    engine_register_macro!(engine, IL_BDEC_TAG_ID, ILBigDecTag);
    engine_register_macro!(engine, IL_ILINTARRAY_TAG_ID, ILILIntArrayTag);
    engine_register_macro!(engine, IL_ILTAGARRAY_TAG_ID, ILTagArrayTag);
    engine_register_macro!(engine, IL_ILTAGSEQ_TAG_ID, ILTagSeqTag);
    engine_register_macro!(engine, IL_RANGE_TAG_ID, ILRangeTag);
    engine_register_macro!(engine, IL_VERSION_TAG_ID, ILVersionTag);
    engine_register_macro!(engine, IL_OID_TAG_ID, ILOIDTag);
    engine_register_macro!(engine, IL_DICTIONARY_TAG_ID, ILDictTag);
    engine_register_macro!(engine, IL_STRING_DICTIONARY_TAG_ID, ILStrDictTag);
    engine
}

//=============================================================================
// ILStandardTagFactory
//-----------------------------------------------------------------------------
/// This struct implements the standard ILTagFactory factory. It
/// can be extended to include custom tags if necessary.
pub struct ILStandardTagFactory {
    engine: ILTagCreatorEngine,
}

impl ILStandardTagFactory {
    /// Creates a new instance of the ILStandardTagFactory.
    ///
    /// Arguments:
    /// - `strict`: If true, this factory will work in strict mode.
    pub fn new(strict: bool) -> Self {
        Self {
            engine: create_std_engine(strict),
        }
    }

    /// Grants a mutable access to the inner engine. It allows the
    /// registration of new tag creators if
    ///
    /// Returns:
    /// - A mutable reference to the inner engine;
    pub fn engine(&mut self) -> &mut ILTagCreatorEngine {
        &mut self.engine
    }
}

impl ILTagFactory for ILStandardTagFactory {
    fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>> {
        self.engine.create_tag(tag_id)
    }

    fn deserialize(&self, reader: &mut dyn Reader) -> Result<Box<dyn ILTag>> {
        let tag_id = deserialize_ilint(reader)?;
        let tag_size = if is_implicit_tag(tag_id) {
            implicit_tag_size(tag_id)
        } else {
            deserialize_ilint(reader)?
        };
        let utag_size = tag_size_to_usize(tag_size)?;
        let mut tag = match self.create_tag(tag_id) {
            Some(t) => t,
            None => return Err(ErrorKind::UnknownTag),
        };
        if tag_id == IL_ILINT_TAG_ID {
            tag.deserialize_value(self, utag_size, reader)?;
        } else {
            let mut lreader = LimitedReader::new(reader, utag_size);
            tag.deserialize_value(self, utag_size, &mut lreader)?;
            if !lreader.empty() {
                return Err(ErrorKind::CorruptedData);
            }
        }
        Ok(tag)
    }
}
