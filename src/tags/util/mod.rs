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
//! This module contains some utility functions that may help the usage of this
//! library.

#[cfg(test)]
mod tests;

use super::standard::factory::ILStandardTagFactory;
use super::{ILTag, ILTagFactory, Result};
use crate::io::array::{ByteArrayReader, VecWriter};

/// This function compares ILTag instances by serializing them and
/// compare if the serialization matches.
///
/// This is, by far, the least efficient way to perform the comparison but it
/// is guaranteed to work for all scenarios, including those when
/// the same tag is implemented by distinct structs.
///
/// Arguments:
/// * `a`: ILTag A;
/// * `b`: ILTag B;
///
/// Returns:
/// - true if the tags are equal or false otherwise or in case the comparison
///   cannot be performed (due to errors in the serialization).
pub fn iltag_are_equal(a: &dyn ILTag, b: &dyn ILTag) -> bool {
    let mut writer_a = VecWriter::default();
    match a.serialize(&mut writer_a) {
        Ok(()) => (),
        _ => return false,
    }
    let mut writer_b = VecWriter::default();
    match b.serialize(&mut writer_b) {
        Ok(()) => (),
        _ => return false,
    }
    writer_a.vec().as_slice() == writer_b.vec().as_slice()
}

/// Clones the given tag using a serialization/deserialization process.
///
/// It is not the most efficient way to perform the cloning but it allows
/// the copy of any tag that can be built by the specified factory.
/// Furthermore it guarantees no relationship with the original tag
/// and its clone.
///
/// Arguments:
/// * `factory`: The tag factory to be used;
/// * `tag`: The tag to be cloned;
///
/// Returns:
/// * Ok(tag): The cloned tag;
/// * Err(_): If the cloning process fails;
pub fn iltag_clone_with_factory(
    factory: &dyn ILTagFactory,
    tag: &dyn ILTag,
) -> Result<Box<dyn ILTag>> {
    let mut writer = VecWriter::default();
    tag.serialize(&mut writer)?;
    let mut reader = ByteArrayReader::new(writer.vec().as_slice());
    factory.deserialize(&mut reader)
}

/// Clones the given tag using a serialization/deserialization process.
/// It is similar to [`iltag_clone_with_factory()`] but uses a
/// [`ILStandardTagFactory`] instance runing on non strict mode.
///
///
///
/// Arguments:
/// * `tag`: The tag to be cloned;
///
/// Returns:
/// * Ok(tag): The cloned tag;
/// * Err(_): If the cloning process fails;
pub fn iltag_clone(tag: &dyn ILTag) -> Result<Box<dyn ILTag>> {
    let factory = ILStandardTagFactory::new(false);
    iltag_clone_with_factory(&factory, tag)
}
