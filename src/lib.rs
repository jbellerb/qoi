pub mod info;

mod decoder;
mod encoder;

pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;

use rgb::RGBA;

const QOI_MAGIC: [u8; 4] = [113, 111, 105, 102];
const QOI_PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

const QOI_OP_INDEX: u8 = 0x00; // 00xxxxxx
const QOI_OP_DIFF: u8 = 0x40; // 01xxxxxx
const QOI_OP_LUMA: u8 = 0x80; // 10xxxxxx
const QOI_OP_RUN: u8 = 0xc0; // 11xxxxxx
const QOI_OP_RGB: u8 = 0xfe; // 11111110
const QOI_OP_RGBA: u8 = 0xff; // 11111111

#[inline(always)]
fn qoi_hash(pixel: RGBA<u8>) -> usize {
    let r = pixel.r as usize;
    let g = pixel.g as usize;
    let b = pixel.b as usize;
    let a = pixel.a as usize;

    (r * 3 + g * 5 + b * 7 + a * 11) % 64
}
