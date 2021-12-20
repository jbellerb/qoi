use std::io::{Read, Write};

use anyhow::{anyhow, Error, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Channels {
    Rgb = 3,
    Rgba = 4,
}

impl TryFrom<u8> for Channels {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self> {
        match v {
            3 => Ok(Channels::Rgb),
            4 => Ok(Channels::Rgba),
            _ => Err(anyhow!("invalid number of channels")),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Colorspace {
    Srgb = 0,
    Linear = 1,
}

impl TryFrom<u8> for Colorspace {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self> {
        match v {
            0 => Ok(Colorspace::Srgb),
            1 => Ok(Colorspace::Linear),
            _ => Err(anyhow!("invalid colorspace")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub channels: Channels,
    pub colorspace: Colorspace,
}

impl Info {
    pub fn new(width: u32, height: u32, channels: Channels, colorspace: Colorspace) -> Self {
        Info {
            width,
            height,
            channels,
            colorspace,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn channels(&self) -> Channels {
        self.channels
    }

    pub fn colorspace(&self) -> Colorspace {
        self.colorspace
    }

    pub fn encode<W: Write>(&self, mut w: W) -> Result<()> {
        w.write_u32::<BigEndian>(self.width)?;
        w.write_u32::<BigEndian>(self.height)?;
        w.write_u8(self.channels as u8)?;
        w.write_u8(self.colorspace as u8)?;

        Ok(())
    }

    pub fn decode<R: Read>(mut r: R) -> Result<Self> {
        Ok(Info {
            width: r.read_u32::<BigEndian>()?,
            height: r.read_u32::<BigEndian>()?,
            channels: r.read_u8()?.try_into()?,
            colorspace: r.read_u8()?.try_into()?,
        })
    }
}
