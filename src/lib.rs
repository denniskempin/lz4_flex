//! Pure Rust implementation of LZ4 compression.
//!
//! A detailed explanation of the algorithm can be found [here](http://ticki.github.io/blog/how-lz4-works/).

extern crate byteorder;
#[macro_use]
extern crate quick_error;

mod block;
mod decompress_old;
#[cfg(test)]
mod tests;

pub use block::compress::{compress, compress_into};
pub use block::decompress::{decompress, decompress_into};

pub use decompress_old::{decompress as decompress_old, decompress_into as decompress_old_into};

const ONLY_HIGH_BIT_U8: u16 = 0b_1000_0000_0000_0000;
pub const TOKEN_FULL_DUPLICATE_U16: u16 = 0b_0111_1111_1111_1111;

#[inline(always)]
pub fn set_high_bit_u16(input: u16) -> u16 {
    input | ONLY_HIGH_BIT_U8
}

#[inline(always)]
pub fn is_full(input: u16) -> bool {
    input & ONLY_HIGH_BIT_U8 != 0
}

#[inline(always)]
pub fn is_high_bit_set(input: u16) -> bool {
    input & ONLY_HIGH_BIT_U8 != 0
}
