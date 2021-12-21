mod decoder;
mod encoder;
mod info;

pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;
pub use crate::info::{Channels, Colorspace, Info};

use std::io::Error;

use rgb::RGBA;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid number of channels: {0}")]
    InvalidChannels(u8),
    #[error("Invalid colorspace")]
    InvalidColorspace,
    #[error("Invalid QOI signature")]
    InvalidSignature,
    #[error("Decode buffer too small, expected {expected} got {actual}")]
    BufferSize { expected: usize, actual: usize },
    #[error("Image contains out-of-bounds pixels")]
    OutOfBounds,
    #[error(transparent)]
    IoError(#[from] Error),
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("Image has zero width")]
    ZeroWidth,
    #[error("Image has zero height")]
    ZeroHeight,
    #[error(transparent)]
    IoError(#[from] Error),
}
