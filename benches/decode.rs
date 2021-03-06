use std::fs::read;

use qoi::Decoder;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

const IMAGES: &[&str] = &[
    "macaws.qoi",
    "house.qoi",
    "wikipedia.qoi",
    "duckduckgo.qoi",
    "terrazzo_diffuse.qoi",
    "terrazzo_displacement.qoi",
    "terrazzo_normal.qoi",
    "terrazzo_roughness.qoi",
    "icon_image.qoi",
];

fn bench_decode(c: &mut Criterion) {
    for file in IMAGES {
        let mut group = c.benchmark_group("decode");

        let data = read(format!("tests/images/qoi/{}", file)).unwrap();
        let decoder = Decoder::new(data.as_slice()).unwrap();
        let mut buf = vec![0; decoder.output_buffer_size()];
        group.throughput(Throughput::Bytes(decoder.output_buffer_size() as u64));

        group.bench_with_input(*file, &data, |b, data| {
            b.iter(|| {
                let mut decoder = Decoder::new(data.as_slice()).unwrap();
                decoder.read_image(&mut buf).unwrap();
            })
        });
    }
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
