use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use qoi::{Decoder, Encoder};

use clap::Parser;
use image::RgbaImage;

#[derive(Parser, Debug)]
#[clap(name = "qoiconv", version)]
struct Args {
    /// Input png or qoi to convert
    #[clap(parse(from_os_str))]
    infile: PathBuf,

    /// Output file of the opposite extension
    #[clap(parse(from_os_str))]
    outfile: PathBuf,
}

fn main() {
    let args = Args::parse();

    let img = match args.infile.extension().and_then(OsStr::to_str) {
        Some("png") => image::open(args.infile).unwrap().to_rgba8(),
        Some("qoi") => {
            let file = File::open(args.infile).unwrap();
            let mut decoder = Decoder::new(BufReader::new(file)).unwrap();

            let (width, height) = decoder.info().size();
            let mut buf = vec![0; decoder.output_buffer_size()];
            decoder.read_image(&mut buf).unwrap();

            RgbaImage::from_raw(width, height, buf).unwrap()
        }
        _ => panic!("Input file must be .png or .qoi"),
    };

    match args.outfile.extension().and_then(OsStr::to_str) {
        Some("png") => {
            img.save(args.outfile).unwrap();
        }
        Some("qoi") => {
            let file = File::create(args.outfile).unwrap();
            let (width, height) = img.dimensions();
            let mut encoder = Encoder::new(BufWriter::new(file), width, height).unwrap();
            encoder.write_image_data(img.as_raw()).unwrap();
        }
        _ => panic!("Output file must be .png or .qoi"),
    };
}
