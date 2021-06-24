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

use super::*;

#[test]
fn test_constants() {
    // Implicit tags
    assert_eq!(IL_NULL_TAG_ID, 0);
    assert_eq!(IL_BOOL_TAG_ID, 1);
    assert_eq!(IL_INT8_TAG_ID, 2);
    assert_eq!(IL_UINT8_TAG_ID, 3);
    assert_eq!(IL_INT16_TAG_ID, 4);
    assert_eq!(IL_UINT16_TAG_ID, 5);
    assert_eq!(IL_INT32_TAG_ID, 6);
    assert_eq!(IL_UINT32_TAG_ID, 7);
    assert_eq!(IL_INT64_TAG_ID, 8);
    assert_eq!(IL_UINT64_TAG_ID, 9);
    assert_eq!(IL_ILINT_TAG_ID, 10);
    assert_eq!(IL_BIN32_TAG_ID, 11);
    assert_eq!(IL_BIN64_TAG_ID, 12);
    assert_eq!(IL_BIN128_TAG_ID, 13);

    // Reserved tags
    assert_eq!(IL_BYTES_TAG_ID, 16);
    assert_eq!(IL_STRING_TAG_ID, 17);
    assert_eq!(IL_BINT_TAG_ID, 18);
    assert_eq!(IL_BDEC_TAG_ID, 19);
    assert_eq!(IL_ILINTARRAY_TAG_ID, 20);
    assert_eq!(IL_ILTAGARRAY_TAG_ID, 21);
    assert_eq!(IL_ILTAGSEQ_TAG_ID, 22);
    assert_eq!(IL_RANGE_TAG_ID, 23);
    assert_eq!(IL_VERSION_TAG_ID, 24);
    assert_eq!(IL_OID_TAG_ID, 25);
    assert_eq!(IL_DICTIONARY_TAG_ID, 30);
    assert_eq!(IL_STRING_DICTIONARY_TAG_ID, 31);
}
