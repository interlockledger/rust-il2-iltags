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
//! This module defines the [`ILGenericPayloadTag`] and the trait required to
//! implement its payload.

#[cfg(test)]
mod tests;

use crate::io::{Reader, Writer};
use crate::tags::{DefaultWithId, ILTag, ILTagFactory, Result};
use std::any::Any;
use std::ops::{Deref, DerefMut};

//=============================================================================
// ILTagPayload
//-----------------------------------------------------------------------------
/// This trait must be implemented by all payloads that will be used in
/// conjuction with [`ILGenericPayloadTag`].
///
/// Since 1.1.1.
pub trait ILTagPayload: 'static {
    /// Returns the serialized size in bytes. It is equivalent
    /// [`crate::tags::ILTag::value_size()`].
    fn serialized_size(&self) -> usize;

    /// Serializes this payload. The total lenght of the data written must match
    /// the size returned by `serialized_size()`. It is equivalent to
    /// [`crate::tags::ILTag::serialize_value()`].
    ///
    /// Arguments:
    ///
    /// - `writer`: The writer;
    ///
    /// Returns:
    ///
    /// - `Ok(())`: In case of success;
    /// - `Err(_)`: In case of error;
    fn serialize(&self, writer: &mut dyn Writer) -> Result<()>;

    /// Deserializes the payload and initializes this instace with it. It is equivalent to
    /// [`crate::tags::ILTag::deserialize_value()`].
    ///
    /// Arguments:
    ///
    /// * `factory`: The current tag factory. It is used to create new inner tags if necessary.
    /// * `value_size`: Size of the value in bytes;
    /// * `reader`: The tag reader to be used;
    ///
    /// Returns:
    ///
    /// * `Ok()`: On success.
    /// * `Err(())`: In case of error.
    fn deserialize(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()>;
}

//=============================================================================
// ILGenericPayloadTag
//-----------------------------------------------------------------------------
/// This generic struct implements a tag that contains a generic payload. The
/// payload must handle all details about the data types and its serialization/
/// deserialization procedures while this struct will handle the [`ILTag`]
/// related functionalities.
///
/// This struct implements the trait [`DefaultWithId`] which allows it to be
/// used in conjuction with the generic [`crate::tags::ILDefaultWithIdTagCreator`].
///
/// A new tag with a custom payload can be defined as follows:
///
/// ```
/// use il2_iltags::io::{Reader, Writer};
/// use il2_iltags::tags::{ErrorKind, ILTagFactory, Result};
/// use il2_iltags::tags::payload::*;
///
/// struct DummyPayload{};
///
/// impl ILTagPayload for DummyPayload {
///     fn serialized_size(&self) -> usize {
///         // Zero as it has no fields
///         0
///     }
///
///     fn serialize(&self, writer: &mut dyn Writer) -> Result<()> {
///         // Nothing to do as it has no fields
///         Ok(())
///     }
///
///     fn deserialize(&mut self, _factory: &dyn ILTagFactory, value_size: usize, _reader: &mut dyn Reader) -> Result<()> {
///          // The size must be zero as it has no fields
///         match value_size {
///             0 => Ok(()),
///             _ => Err(ErrorKind::CorruptedData),
///         }
///     }
/// }
///
/// type DummyPayloadTag = ILGenericPayloadTag<DummyPayload>;
///
/// ```
///
/// In this example, the tag uses `DummyPayload` as its payload, defining a new tag type `DummyPayloadTag`.
///
/// Since 1.1.1.
pub struct ILGenericPayloadTag<T: ILTagPayload + Default> {
    id: u64,
    payload: T,
}

impl<T: ILTagPayload + Send + Default> ILGenericPayloadTag<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            payload: T::default(),
        }
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn mut_payload(&mut self) -> &mut T {
        &mut self.payload
    }
}

impl<T: ILTagPayload + Send + Default> ILTag for ILGenericPayloadTag<T> {
    fn id(&self) -> u64 {
        self.id
    }

    fn value_size(&self) -> u64 {
        self.payload.serialized_size() as u64
    }

    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
        self.payload.serialize(writer)
    }

    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()> {
        self.payload.deserialize(factory, value_size, reader)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T: ILTagPayload + Send + Default> DefaultWithId for ILGenericPayloadTag<T> {
    fn default_with_id(id: u64) -> Self {
        Self::new(id)
    }
}

impl<T: ILTagPayload + Send + Default> Deref for ILGenericPayloadTag<T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.payload()
    }
}

impl<T: ILTagPayload + Send + Default> DerefMut for ILGenericPayloadTag<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mut_payload()
    }
}
