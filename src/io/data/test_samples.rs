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

//=============================================================================
// Test values
//-----------------------------------------------------------------------------
pub const SAMPLE_VALUES_00_U8: u8 = 0x00;
pub const SAMPLE_VALUES_01_U16: u16 = 0x0102;
pub const SAMPLE_VALUES_02_U32: u32 = 0x0304_0506;
pub const SAMPLE_VALUES_03_U64: u64 = 0x0708_090A_0B0C_0D0E;
pub const SAMPLE_VALUES_04_I8: i8 = 0x0f;
pub const SAMPLE_VALUES_05_I16: i16 = 0x1011;
pub const SAMPLE_VALUES_06_I32: i32 = 0x1213_1415;
pub const SAMPLE_VALUES_07_I64: i64 = 0x1617_1819_1A1B_1C1D;
pub const SAMPLE_VALUES_08_F32: f32 = 0.000000000000000000008424034;
pub const SAMPLE_VALUES_09_F64: f64 = 0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030657805298494026;
pub const SAMPLE_VALUES_10_STR: &str = "Lágrimas não são argumentos";
pub const SAMPLE_VALUES_10_STR_LEN: usize = SAMPLE_VALUES_10_STR.len();
pub const SAMPLE_VALUES_11_ILINT: u64 = 0x494c_496e_7456_6264;

pub const SAMPLE_VALUES_BIN_SIZE: usize = 81;

// This constant contains the serialization of the values described in
// SAMPLE_VALUES_xx_type
//
// This frase is attibuted to Machado de Assis. It was
// choosen because it contains Latin characters that
// result in a multi-byte characters in UTF-8.
pub const SAMPLE_VALUES_BIN: [u8; SAMPLE_VALUES_BIN_SIZE] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x4c, 0xc3, 0xa1, 0x67, 0x72, 0x69,
    0x6d, 0x61, 0x73, 0x20, 0x6e, 0xc3, 0xa3, 0x6f, 0x20, 0x73, 0xc3, 0xa3, 0x6f, 0x20, 0x61, 0x72,
    0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x6f, 0x73, 0xFF, 0x49, 0x4C, 0x49, 0x6E, 0x74, 0x56, 0x61,
    0x6C,
];
