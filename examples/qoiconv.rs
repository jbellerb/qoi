use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use qoi::{Channels, Colorspace, Decoder, Encoder, Info};

use clap::Parser;
use image::{DynamicImage, ImageBuffer, Pixel, RgbImage, RgbaImage};

#[derive(Parser, Debug)]
#[clap(name = "qoiconv", version)]
struct Args {
    /// Input png or qoi to convert
    #[clap(parse(from_os_str))]
    infile: PathBuf,

    /// Output png or qoi to create
    #[clap(parse(from_os_str))]
    outfile: PathBuf,
}

#[derive(Debug)]
enum QoiImage {
    ImageRgb(RgbImage),
    ImageRgba(RgbaImage),
}

fn main() {
    let args = Args::parse();

    let img = match args.infile.extension().and_then(OsStr::to_str) {
        Some("png") => {
            let img = image::open(args.infile).unwrap();

            match img {
                DynamicImage::ImageLuma8(..)
                | DynamicImage::ImageRgb8(..)
                | DynamicImage::ImageBgr8(..)
                | DynamicImage::ImageLuma16(..)
                | DynamicImage::ImageRgb16(..) => QoiImage::ImageRgb(img.to_rgb8()),
                DynamicImage::ImageLumaA8(..)
                | DynamicImage::ImageRgba8(..)
                | DynamicImage::ImageBgra8(..)
                | DynamicImage::ImageLumaA16(..)
                | DynamicImage::ImageRgba16(..) => QoiImage::ImageRgba(img.to_rgba8()),
            }
        }
        Some("qoi") => {
            let file = File::open(args.infile).unwrap();
            let mut decoder = Decoder::new(BufReader::new(file)).unwrap();

            let (width, height) = decoder.info().dimensions();
            let mut buf = vec![0; decoder.output_buffer_size()];
            decoder.read_image(&mut buf).unwrap();

            match decoder.info().channels() {
                Channels::Rgb => {
                    QoiImage::ImageRgb(RgbImage::from_raw(width, height, buf).unwrap())
                }
                Channels::Rgba => {
                    QoiImage::ImageRgba(RgbaImage::from_raw(width, height, buf).unwrap())
                }
            }
        }
        _ => panic!("Input file must be .png or .qoi"),
    };

    match args.outfile.extension().and_then(OsStr::to_str) {
        Some("png") => match img {
            QoiImage::ImageRgb(img) => img.save(args.outfile).unwrap(),
            QoiImage::ImageRgba(img) => img.save(args.outfile).unwrap(),
        },
        Some("qoi") => {
            let file = File::create(args.outfile).unwrap();

            match img {
                QoiImage::ImageRgb(img) => save_qoi(img, file, Channels::Rgb),
                QoiImage::ImageRgba(img) => save_qoi(img, file, Channels::Rgba),
            }
        }
        _ => panic!("Output file must be .png or .qoi"),
    };
}

fn save_qoi<P>(img: ImageBuffer<P, Vec<u8>>, file: File, channels: Channels)
where
    P: Pixel<Subpixel = u8> + 'static,
{
    let (width, height) = img.dimensions();
    let info = Info::new(width, height, channels, Colorspace::Srgb);
    let mut encoder = Encoder::new(BufWriter::new(file), info).unwrap();
    encoder.write_image_data(img.as_raw()).unwrap();
}
