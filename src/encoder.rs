use std::convert::TryInto;
use std::io::Write;

use crate::info::Info;
use crate::qoi_hash;

use anyhow::{anyhow, Result};
use byteorder::WriteBytesExt;
use rgb::{ComponentSlice, RGBA};

#[derive(Debug)]
pub struct Encoder<W: Write> {
    w: W,
    info: Info,
    index: [RGBA<u8>; 64],
    prev: RGBA<u8>,
    run: u8,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W, width: u32, height: u32) -> Result<Encoder<W>> {
        if width == 0 {
            return Err(anyhow!("zero width"));
        }

        if height == 0 {
            return Err(anyhow!("zero height"));
        }

        Encoder {
            w,
            info: Info::with_size(width, height),
            index: [RGBA::default(); 64],
            prev: RGBA::default(),
            run: 0,
        }
        .write_header()
    }

    pub fn write_header(mut self) -> Result<Self> {
        self.w.write_all(&crate::QOI_MAGIC)?;
        self.info.encode(&mut self.w)?;

        Ok(self)
    }

    pub fn write_image_data(&mut self, data: &[u8]) -> Result<()> {
        let mut pixels = data.chunks_exact(4).peekable();

        while let Some(pixel) = pixels.next() {
            let pixel = TryInto::<[u8; 4]>::try_into(pixel)?.into();

            if pixel == self.prev {
                self.run += 1;
                if self.run == 62 || pixels.peek().is_none() {
                    self.w.write_u8(crate::QOI_OP_RUN | (self.run - 1))?;
                    self.run = 0;
                }
            } else {
                if self.run > 0 {
                    self.w.write_u8(crate::QOI_OP_RUN | (self.run - 1))?;
                    self.run = 0;
                }

                let hash = qoi_hash(pixel);

                if self.index[hash] == pixel {
                    self.w.write_u8(crate::QOI_OP_INDEX | hash as u8)?;
                } else {
                    self.index[hash] = pixel;

                    if pixel.a == self.prev.a {
                        let dr = pixel.r.wrapping_sub(self.prev.r).wrapping_add(2);
                        let dg = pixel.g.wrapping_sub(self.prev.g).wrapping_add(2);
                        let db = pixel.b.wrapping_sub(self.prev.b).wrapping_add(2);

                        if dr < 4 && dg < 4 && db < 4 {
                            let offsets = dr << 4 | dg << 2 | db;
                            self.w.write_u8(crate::QOI_OP_DIFF | offsets)?;
                        } else {
                            let dg_r = dr.wrapping_sub(dg).wrapping_add(8);
                            let dg_b = db.wrapping_sub(dg).wrapping_add(8);
                            let dg = dg.wrapping_add(30);

                            if dg_r < 16 && dg < 64 && dg_b < 16 {
                                self.w.write_u8(crate::QOI_OP_LUMA | dg)?;
                                self.w.write_u8(dg_r << 4 | dg_b)?;
                            } else {
                                self.w.write_u8(crate::QOI_OP_RGB)?;
                                self.w.write_all(pixel.rgb().as_slice())?;
                            }
                        }
                    } else {
                        self.w.write_u8(crate::QOI_OP_RGBA)?;
                        self.w.write_all(pixel.as_slice())?;
                    }
                }

                self.prev = pixel;
            }
        }

        self.w.write_all(&crate::QOI_PADDING)?;

        Ok(())
    }
}
