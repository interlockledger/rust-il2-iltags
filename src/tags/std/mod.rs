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
//! This is the base module for all standard tags and tag factories.
use super::{DefaultWithId, ErrorKind, ILTag, ILTagFactory, Result};

pub mod constants;
pub mod explicit;
pub mod implicit;

#[cfg(test)]
mod constants_tests;
#[cfg(test)]
mod implicit_tests;
//#[cfg(test)]
//mod tests;

pub use constants::*;
pub use explicit::*;
pub use implicit::*;

/// Implementation of `Default` and `DefaultWithId` traits for all
/// standard tags.
///
/// This macro requires that the struct implementation has a
/// constructor called `new()` that takes no arguments.
///
/// Example:
/// ```
/// pub struct SampleTag{...}
///
/// impl SampleTag {
///     pub fn new() -> Self {...}
///
///     pub fn with_id(id:u64) -> Self {...}
/// }
///
/// iltag_default_impl!(SampleTag);
/// ```
#[macro_export]
macro_rules! iltag_default_impl {
    ($tag_type: ty) => {
        impl Default for $tag_type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl DefaultWithId for $tag_type {
            fn default_with_id(id: u64) -> Self {
                Self::with_id(id)
            }
        }
    };
}
