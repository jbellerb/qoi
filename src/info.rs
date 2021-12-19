use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone, Debug)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

impl Default for Info {
    fn default() -> Info {
        Info {
            width: 0,
            height: 0,
            channels: 4,
            colorspace: 0,
        }
    }
}

impl Info {
    pub fn with_size(width: u32, height: u32) -> Self {
        Info {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn encode<W: Write>(&self, mut w: W) -> Result<()> {
        w.write_u32::<BigEndian>(self.width)?;
        w.write_u32::<BigEndian>(self.height)?;
        w.write_u8(self.channels)?;
        w.write_u8(self.colorspace)?;

        Ok(())
    }

    pub fn decode<R: Read>(mut r: R) -> Result<Info> {
        Ok(Info {
            width: r.read_u32::<BigEndian>()?,
            height: r.read_u32::<BigEndian>()?,
            channels: r.read_u8()?,
            colorspace: r.read_u8()?,
        })
    }
}
