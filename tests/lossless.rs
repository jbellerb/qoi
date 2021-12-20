use qoi::{Channels, Colorspace, Decoder, Encoder, Info};

use image::{open, ImageBuffer, Pixel};

fn test_image<P>(img: ImageBuffer<P, Vec<u8>>, channels: Channels)
where
    P: Pixel<Subpixel = u8> + 'static,
{
    let (width, height) = img.dimensions();

    let mut enc = Vec::new();
    let info = Info::new(width, height, channels, Colorspace::Srgb);
    let mut encoder = Encoder::new(&mut enc, info).unwrap();
    encoder.write_image_data(img.as_raw()).unwrap();

    let mut decoder = Decoder::new(enc.as_slice()).unwrap();
    let mut dec = vec![0; decoder.output_buffer_size()];
    decoder.read_image(&mut dec).unwrap();

    assert_eq!(img.as_raw(), &dec);
}

#[test]
fn image_macaws() {
    let img = open("tests/images/png/macaws.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_house() {
    let img = open("tests/images/png/house.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_wikipedia() {
    let img = open("tests/images/png/wikipedia.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_duckduckgo() {
    let img = open("tests/images/png/duckduckgo.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_terrazzo_diffuse() {
    let img = open("tests/images/png/terrazzo_diffuse.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_terrazzo_displacement() {
    let img = open("tests/images/png/terrazzo_displacement.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_terrazzo_normal() {
    let img = open("tests/images/png/terrazzo_normal.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_terrazzo_roughness() {
    let img = open("tests/images/png/terrazzo_roughness.png").unwrap();
    test_image(img.to_rgb8(), Channels::Rgb);
}

#[test]
fn image_icon_image() {
    let img = open("tests/images/png/icon_image.png").unwrap();
    test_image(img.to_rgba8(), Channels::Rgba);
}
