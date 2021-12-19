use qoi::{Decoder, Encoder};

use image::{open, GenericImageView};

fn test_image(path: &str) {
    let img = open(path).unwrap();
    let (width, height) = img.dimensions();
    let img = img.into_rgba8();

    let mut enc = Vec::new();
    let mut encoder = Encoder::new(&mut enc, width, height).unwrap();
    encoder.write_image_data(img.as_raw()).unwrap();

    let mut decoder = Decoder::new(enc.as_slice()).unwrap();
    let mut dec = vec![0; decoder.output_buffer_size()];
    decoder.read_image(&mut dec).unwrap();

    assert_eq!(img.as_raw(), &dec);
}

#[test]
fn image_macaws() {
    test_image("tests/images/png/macaws.png");
}

#[test]
fn image_house() {
    test_image("tests/images/png/house.png");
}

#[test]
fn image_wikipedia() {
    test_image("tests/images/png/wikipedia.png");
}

#[test]
fn image_duckduckgo() {
    test_image("tests/images/png/duckduckgo.png");
}

#[test]
fn image_terrazzo_diffuse() {
    test_image("tests/images/png/terrazzo_diffuse.png");
}

#[test]
fn image_terrazzo_displacement() {
    test_image("tests/images/png/terrazzo_displacement.png");
}

#[test]
fn image_terrazzo_normal() {
    test_image("tests/images/png/terrazzo_normal.png");
}

#[test]
fn image_terrazzo_roughness() {
    test_image("tests/images/png/terrazzo_roughness.png");
}

#[test]
fn image_icon_image() {
    test_image("tests/images/png/icon_image.png");
}
