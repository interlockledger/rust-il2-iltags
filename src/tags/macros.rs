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

/// This macro implements the methods `ILTag::as_any()` and
/// `ILTag::as_mut_any()` from `ILTag` trait.
///
/// This macro requires the presence of a field `id` (u64) that will hold the id of the
/// tag.
///
/// Example:
/// ```
/// pub struct SampleTag {
///     id: u64,
///     ...
/// }
///
/// impl SampleTag{
///     base_iltag_impl!();
///     ...
/// }
/// ```
///
/// It defines the following methods:
/// - `fn as_any(&self) -> &dyn Any`;
/// - `fn as_mut_any(&mut self) -> &mut dyn Any`;
///
macro_rules! iltag_as_any_impl {
    () => {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_mut_any(&mut self) -> &mut dyn Any {
            self
        }
    };
}

/// This macro implements the methods `ILTag::id()`, `ILTag::as_any()` and
/// `ILTag::as_mut_any()` from `ILTag` trait.
///
/// This macro requires the presence of a field `id` (u64) that will hold the id of the
/// tag.
///
/// Example:
/// ```
/// pub struct SampleTag {
///     id: u64,
///     ...
/// }
///
/// impl SampleTag{
///     base_iltag_impl!();
///     ...
/// }
/// ```
///
/// It defines the following methods:
/// - `fn id(&self) -> u64`;
/// - `fn as_any(&self) -> &dyn Any`;
/// - `fn as_mut_any(&mut self) -> &mut dyn Any`;
macro_rules! iltag_base_func_impl {
    () => {
        fn id(&self) -> u64 {
            self.id
        }

        iltag_as_any_impl!();
    };
}

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
///     ...
/// }
///
/// iltag_default_impl!(SampleTag);
/// ```
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

/// This macro is equivalent to iltag_base_func_impl but was
/// designed to work with structs that uses another tag
/// implementation internally.
///
/// Example:
///
/// ```
/// pub struct NewTag{
///     inner: ILRawTag,
/// }
///
/// impl NewTag {
///     base_iltag_inner_impl!();
/// }
/// ```
///
/// It defines the following methods:
/// - `fn id(&self) -> u64`;
/// - `fn as_any(&self) -> &dyn Any`;
/// - `fn as_mut_any(&mut self) -> &mut dyn Any`;
macro_rules! inner_iltag_base_func_impl {
    () => {
        fn id(&self) -> u64 {
            self.inner.id()
        }

        iltag_as_any_impl!();
    };
}

/// This macro is implements the ILTag for all structs that uses
/// another tag implementation internally.
///
/// Example:
///
/// ```
/// pub struct NewTag{
///     inner: ILRawTag,
/// }
///
/// inner_iltag_default_impl!(NewTag);
/// ```
///
/// It defines all methods of ILTag delegating the functionalities
/// to the inner tag.
macro_rules! inner_iltag_default_impl {
    ($tag_type:ty) => {
        impl ILTag for $tag_type {
            inner_iltag_base_func_impl!();

            fn value_size(&self) -> u64 {
                self.inner.value_size()
            }

            fn serialize_value(&self, writer: &mut dyn Writer) -> Result<()> {
                self.inner.serialize_value(writer)
            }

            fn deserialize_value(
                &mut self,
                factory: &dyn ILTagFactory,
                value_size: usize,
                reader: &mut dyn Reader,
            ) -> Result<()> {
                self.inner.deserialize_value(factory, value_size, reader)
            }
        }
    };
}
