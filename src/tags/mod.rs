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
//! This module contains the implementation of the **IL2 ILTags** standard.
pub mod std;
#[cfg(test)]
mod tests;

use crate::ilint::encoded_size;
use crate::io::data::*;
use crate::io::{Reader, Writer};
use ::std::any::Any;
use ::std::collections::HashMap;

/// Definition of the errors from this package.
pub enum ErrorKind {
    UnknownTag,
    UnsupportedTag,
    CorruptedData,
    IOError(crate::io::ErrorKind),
    Boxed(Box<dyn ::std::error::Error>),
}

/// Alias to errors from this package.
pub type Result<T> = ::std::result::Result<T, ErrorKind>;

/// Maximum tag id value for implicit tags.
pub const IMPLICIT_ID_MAX: u64 = 0x0F;

/// Maximum tag id value for reserved tags.
pub const RESERVED_ID_MAX: u64 = 0x1F;

/// Verifies if a given tag id represents an implicit tag.
///
/// Arguments:
///
/// * `id`: The tag id to be verified;
///
/// Returns:
///
/// * true if the tag id is implicit or false otherwise.
///
pub fn is_implicit_tag(id: u64) -> bool {
    id <= IMPLICIT_ID_MAX
}

/// Verifies if a given tag id represents a reserved tag.
///
/// Arguments:
///
/// * `id`: The tag id to be verified;
///
/// Returns:
///
/// * true if the tag id is reserved or false otherwise.
///
pub fn is_reserved_tag(id: u64) -> bool {
    id <= RESERVED_ID_MAX
}

//=============================================================================
// ILTag
//-----------------------------------------------------------------------------

/// This trait must be implemented by all ILTags on this library.
pub trait ILTag: Any {
    /// Returns the ID of the tag.
    fn id(&self) -> u64;

    /// Verifies if this tag is implicity.
    fn is_implicity(&self) -> bool {
        is_implicit_tag(self.id())
    }

    /// Verifies if this tag is reserved.
    fn is_reserved(&self) -> bool {
        is_reserved_tag(self.id())
    }

    /// Retuns the size of the serialized value in bytes.
    fn value_size(&self) -> u64;

    /// Returns the total size of the tag in bytes.
    fn size(&self) -> u64 {
        let mut size: u64 = encoded_size(self.id()) as u64;
        if !self.is_implicity() {
            size += encoded_size(self.value_size()) as u64;
        }
        size + self.value_size()
    }

    /// Serializes the payload of this tag.
    ///
    /// Arguments:
    ///
    /// * `writer`: The writer that will receive the encoded value;
    ///
    /// Returns:
    ///
    /// * `Ok()`: On success.
    /// * `Err(())`: If the buffer is too small to hold the encoded value.
    ///
    fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()>;

    /// Serializes this tag.
    ///
    /// Arguments:
    ///
    /// * `writer`: The writer that will receive the encoded value;
    ///
    /// Returns:
    ///
    /// * `Ok()`: On success.
    /// * `Err(())`: If the buffer is too small to hold the encoded value.
    ///
    fn serialize(&self, writer: &mut dyn Writer) -> Result<()> {
        match write_ilint(self.id(), writer) {
            Ok(()) => (),
            Err(e) => return Err(ErrorKind::IOError(e)),
        }
        if !self.is_implicity() {
            match write_ilint(self.value_size() as u64, writer) {
                Ok(()) => (),
                Err(e) => return Err(ErrorKind::IOError(e)),
            }
        }
        self.serialize_value(writer)
    }

    /// Deserializes the value.
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
    /// * `Err(())`: If the buffer is too small to hold the encoded value.
    fn deserialize_value(
        &mut self,
        factory: &dyn ILTagFactory,
        value_size: usize,
        reader: &mut dyn Reader,
    ) -> Result<()>;

    /// Returns a reference as Any.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference as Any.
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

/// Downcasts a ILTag trait to its concrete type.
///
/// Arguments:
/// * `tag`: The tag to be downcast;
///
/// Returns:
/// An option with a reference to the concrete type or None if
/// the conversion is not possible.
pub fn tag_downcast_ref<T: ILTag + Any>(tag: &dyn ILTag) -> Option<&T> {
    tag.as_any().downcast_ref::<T>()
}

/// Downcasts a ILTag trait to its concrete type as a mutable reference.
///
/// Arguments:
/// * `tag`: The tag to be downcast;
///
/// Returns:
/// An option with a reference to the concrete type or None if
/// the conversion is not possible.
pub fn tag_downcast_mut<T: ILTag + Any>(tag: &mut dyn ILTag) -> Option<&mut T> {
    tag.as_mut_any().downcast_mut::<T>()
}

/// This macro implements the methods id(), as_any() and as_mut_any()
/// from ILTag trait.
///
/// This macro requires that the `id` value be stored in the field `id`.
#[macro_export]
macro_rules! base_iltag_impl {
    () => {
        fn id(&self) -> u64 {
            self.id
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
        }
    };
}

//=============================================================================
// ILTagFactory
//-----------------------------------------------------------------------------
/// This trait must be implemented by all tag factories.
pub trait ILTagFactory {
    fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>>;

    fn deserialize(&self, reader: &mut dyn Reader) -> Result<Box<dyn ILTag>>;
}

//=============================================================================
// ILTagCreator
//-----------------------------------------------------------------------------

/// This trait must be implemented by all tag creators.
pub trait ILTagCreator {
    /// Creates a new boxed instance of the the class.
    ///
    /// Arguments:
    ///
    /// * `tag_id`: The tag id.
    ///
    /// Returns:
    /// * `Box<dyn ILTag>`: The new empty boxed tag.
    fn create_empty_tag(&self, tag_id: u64) -> Box<dyn ILTag>;
}

//=============================================================================
// ILCreatorEngine
//-----------------------------------------------------------------------------
/// This struct implements an engine that maps the ILTagCreators
/// to the associated tag ID. It can be used as a component to implement
/// ILTagFactory trait.
pub struct ILTagCreatorEngine {
    creators: HashMap<u64, Box<dyn ILTagCreator>>,
    strict: bool,
}

impl ILTagCreatorEngine {
    /// Creates a new instance of the ILTagCreatorEngine.
    ///
    /// Arguments:
    /// * `strict`: Strict mode. If false, unknown tags will be mapped to RawTag
    /// instances.
    pub fn new(strict: bool) -> ILTagCreatorEngine {
        ILTagCreatorEngine {
            creators: HashMap::new(),
            strict,
        }
    }

    /// Rreturns the current strict flag.
    pub fn strict(&self) -> bool {
        self.strict
    }

    /// Registers a new ILTagCreator.
    ///
    /// Arguments:
    /// * `tag_id`: The tag id;
    /// * `creator`: The new creator;
    ///
    /// Returns:
    /// * `Some<Box<dyn ILTagCreator>>`: The previously registered creator for the specified id;
    /// * `None`: If the id is not associated with a new creator;
    pub fn register(
        &mut self,
        tag_id: u64,
        creator: Box<dyn ILTagCreator>,
    ) -> Option<Box<dyn ILTagCreator>> {
        self.creators.insert(tag_id, creator)
    }

    /// Creates a new empty tag for the given id. It uses the registered creators
    /// to perform the operation.
    ///
    /// Arguments:
    /// * `tag_id`: The tag id;
    ///
    /// Returns:
    /// * `Some<Box<dyn ILTag>>`: The new empty tag instance;
    /// * `None`: If the tag id is unknown (only if strict mode is enabled);
    pub fn create_tag(&self, tag_id: u64) -> Option<Box<dyn ILTag>> {
        match self.creators.get(&tag_id) {
            Some(c) => Some(c.create_empty_tag(tag_id)),
            None => {
                if !self.strict && !is_implicit_tag(tag_id) {
                    Some(Box::new(ILRawTag::new(tag_id)))
                } else {
                    None
                }
            }
        }
    }
}

//=============================================================================
// ILRawTag
//-----------------------------------------------------------------------------
/// This struct implements a raw tag. It can be used to store any non
/// explicit tag.
pub struct ILRawTag {
    id: u64,
    value: Vec<u8>,
}

impl ILRawTag {
    /// Creates a new instance of this struct.
    ///
    /// Arguments:
    /// * `id`: The tag id;
    ///
    /// Returns:
    /// * The new instance of RawTag.
    pub fn new(id: u64) -> ILRawTag {
        assert!(!is_implicit_tag(id));
        ILRawTag {
            id,
            value: Vec::new(),
        }
    }

    /// Initializes a new RawTag with a given capacity.
    ///
    /// Arguments:
    /// * `id`: The tag id;
    /// * `capacity`: The expected initial capacity;
    ///
    /// Returns:
    /// * The new instance of RawTag.
    pub fn with_capacity(id: u64, capacity: usize) -> ILRawTag {
        assert!(!is_implicit_tag(id));
        ILRawTag {
            id,
            value: Vec::with_capacity(capacity),
        }
    }

    /// Initializes a new RawTag with an initial value.
    ///
    /// Arguments:
    /// * `id`: The tag id;
    /// * `value`: A byte slice with the initial value;
    ///
    /// Returns:
    /// * The new instance of RawTag.
    pub fn with_value(id: u64, value: &[u8]) -> ILRawTag {
        assert!(!is_implicit_tag(id));
        let mut v: Vec<u8> = Vec::with_capacity(value.len());
        v.extend_from_slice(value);
        ILRawTag { id, value: v }
    }

    /// Returns an immutable reference to the payload.
    pub fn value(&self) -> &Vec<u8> {
        &self.value
    }

    /// Returns a mutable reference to the payload.
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
}

impl ILTag for ILRawTag {
    base_iltag_impl!();

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
