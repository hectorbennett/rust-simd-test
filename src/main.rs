use rust_simd_test::{mult_vectors_with_simd, mult_vectors_without_simd};

pub mod alpha_blend_pixel_buffers;

fn main() {
    println!("{}", "Hello world!");
    // generate a vector of ints ascending 0, 1, 2, ...
    const SIZE: usize = 10_000_000;
    let v1: Vec<u64> = (0u64..SIZE as u64).collect();

    // generate another vector of size 1_000_000, with ints descending
    let v2: Vec<u64> = v1.iter().copied().rev().collect();

    let v3 = mult_vectors_with_simd(&v1, &v2);

    println!("{}", v3.len())
}
