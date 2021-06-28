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
use super::explicit::*;
use super::implicit::*;
use crate::io::{LimitedReader, Reader};
use crate::tags::{
    deserialize_ilint, is_implicit_tag, tag_size_to_usize, ErrorKind, ILDefaultTagCreator, ILTag,
    ILTagCreator, ILTagCreatorEngine, ILTagFactory, Result,
};
use ::std::any::Any;

fn create_std_engine(strict: bool) -> ILTagCreatorEngine {
    let mut engine = ILTagCreatorEngine::new(strict);

    engine.register(
        IL_NULL_TAG_ID,
        Box::new(ILDefaultTagCreator::<ILNullTag>::default()),
    );
    engine.register(
        IL_BOOL_TAG_ID,
        Box::new(ILDefaultTagCreator::<ILBoolTag>::default()),
    );

    engine
}

pub struct ILStandardTagFactory {
    engine: ILTagCreatorEngine,
}

impl ILStandardTagFactory {
    pub fn new(strict: bool) -> Self {
        Self {
            engine: create_std_engine(strict),
        }
    }
}

impl ILTagFactory for ILStandardTagFactory {
    fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>> {
        self.engine.create_tag(tag_id)
    }

    fn deserialize(&self, reader: &mut dyn Reader) -> Result<Box<dyn ILTag>> {
        let tag_id = deserialize_ilint(reader)?;
        let tag_size = if is_implicit_tag(tag_id) {
            // TODO
            1
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
