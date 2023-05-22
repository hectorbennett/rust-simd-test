use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_simd_test::alpha_blend_pixel_buffers::blend_pixels;
use rust_simd_test::fibonacci::fibonacci;
use rust_simd_test::mult_vectors::{mult_vectors_with_simd, mult_vectors_without_simd};

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

    // generate 2 vectors to multiply. one of ascending ints, the other descending.
    const SIZE: usize = 1_000_000;
    let v1: Vec<u64> = (0u64..SIZE as u64).collect();
    let v2: Vec<u64> = v1.iter().copied().rev().collect();

    c.bench_function("mult_vectors_without_simd", |b| {
        b.iter(|| mult_vectors_without_simd(black_box(&v1), black_box(&v2)));
    });

    c.bench_function("mult_vectors_with_simd", |b| {
        b.iter(|| mult_vectors_with_simd(black_box(&v1), black_box(&v2)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
