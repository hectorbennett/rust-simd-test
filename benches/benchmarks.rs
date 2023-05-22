use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_simd_test::alpha_blend_pixel_buffers::blend_pixels;
use rust_simd_test::fibonacci::fibonacci;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("blend_pixels", |b| {
        b.iter(|| {
            blend_pixels(
                black_box(&[1.0, 2.0, 3.0, 4.0]),
                black_box(&[5.0, 6.0, 7.0, 8.0]),
            )
        })
    });

    c.bench_function("fibonacci", |b| {
        b.iter(|| fibonacci(black_box(20)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
