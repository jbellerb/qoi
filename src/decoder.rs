use std::io::Read;

use crate::{qoi_hash, Channels, Info};

use anyhow::{ensure, Result};
use byteorder::ReadBytesExt;
use bytes::BufMut;
use rgb::{ComponentSlice, RGBA};

#[derive(Debug)]
pub struct Decoder<R: Read> {
    r: R,
    info: Info,
    pixel: usize,
    index: [RGBA<u8>; 64],
    prev: RGBA<u8>,
}

impl<R: Read> Decoder<R> {
    pub fn new(mut r: R) -> Result<Self> {
        let mut magic_bytes = [0; 4];
        r.read_exact(&mut magic_bytes)?;
        ensure!(magic_bytes == crate::QOI_MAGIC, "magic number mismatch");

        let info = Info::decode(&mut r)?;

        Ok(Decoder {
            r,
            info,
            pixel: 0,
            index: [RGBA::default(); 64],
            prev: RGBA::new(0, 0, 0, 255),
        })
    }

    pub fn info(&self) -> &Info {
        &self.info
    }

    pub fn output_buffer_size(&self) -> usize {
        let (width, height) = self.info.dimensions();

        (width * height) as usize * self.info.channels as usize
    }

    pub fn read_image(&mut self, mut buf: &mut [u8]) -> Result<()> {
        ensure!(buf.len() >= self.output_buffer_size(), "buf too small");

        let size = self.output_buffer_size() / self.info.channels as usize;

        while self.pixel < size {
            let mut pixel = self.prev;
            let byte = self.r.read_u8()?;

            if byte == crate::QOI_OP_RGBA {
                pixel.r = self.r.read_u8()?;
                pixel.g = self.r.read_u8()?;
                pixel.b = self.r.read_u8()?;
                pixel.a = self.r.read_u8()?;
            } else if byte == crate::QOI_OP_RGB {
                pixel.r = self.r.read_u8()?;
                pixel.g = self.r.read_u8()?;
                pixel.b = self.r.read_u8()?;
            } else {
                match byte & 0xc0 {
                    crate::QOI_OP_INDEX => {
                        pixel = self.index[byte as usize];
                    }
                    crate::QOI_OP_DIFF => {
                        let dr = ((byte >> 4) & 0x03).wrapping_sub(2);
                        let dg = ((byte >> 2) & 0x03).wrapping_sub(2);
                        let db = (byte & 0x03).wrapping_sub(2);

                        pixel.r = pixel.r.wrapping_add(dr);
                        pixel.g = pixel.g.wrapping_add(dg);
                        pixel.b = pixel.b.wrapping_add(db);
                    }
                    crate::QOI_OP_LUMA => {
                        let r_b = self.r.read_u8()?;
                        let dg = (byte & 0x3f).wrapping_sub(32);

                        let dr = dg.wrapping_sub(8).wrapping_add((r_b >> 4) & 0x0f);
                        let db = dg.wrapping_sub(8).wrapping_add(r_b & 0x0f);

                        pixel.r = pixel.r.wrapping_add(dr);
                        pixel.g = pixel.g.wrapping_add(dg);
                        pixel.b = pixel.b.wrapping_add(db);
                    }
                    crate::QOI_OP_RUN => {
                        let run = byte & 0x3f;
                        ensure!(self.pixel + (run as usize) < size, "overrun");

                        for _ in 0..run {
                            match self.info.channels {
                                Channels::Rgb => buf.put(pixel.rgb_mut().as_slice()),
                                Channels::Rgba => buf.put(pixel.as_slice()),
                            };
                        }
                        self.pixel += run as usize;
                    }
                    _ => unreachable!(),
                }
            }

            self.index[qoi_hash(pixel)] = pixel;

            match self.info.channels {
                Channels::Rgb => buf.put(pixel.rgb_mut().as_slice()),
                Channels::Rgba => buf.put(pixel.as_slice()),
            };
            self.prev = pixel;
            self.pixel += 1;
        }

        Ok(())
    }
}
