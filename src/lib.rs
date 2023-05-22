#![feature(test)]
#![feature(portable_simd)]

extern crate test;

use std::simd::{f32x4, u64x4};

pub fn mult_vectors_without_simd(vec1: &Vec<u64>, vec2: &Vec<u64>) -> Vec<u64> {
    let mut vec3 = vec![];
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        vec3.push(v1 * v2);
    }
    vec3
}

pub fn mult_vectors_with_simd(vec1: &Vec<u64>, vec2: &Vec<u64>) -> Vec<u64> {
    let mut vec3 = vec![];
    for (v1, v2) in vec1.chunks(4).zip(vec2.chunks(4)) {
        let simd_v1 = u64x4::from_slice(v1);
        let simd_v2 = u64x4::from_slice(v2);
        let simd_v3 = simd_v1 * simd_v2;
        vec3.extend(simd_v3.to_array());
    }
    vec3
}

pub fn div_vectors_without_simd(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut vec3 = vec![];
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        vec3.push(v1 / v2);
    }
    vec3
}

// fn fast_sum(x: &[f32]) -> f32 {
//     assert!(x.len() % 4 == 0);
//     let mut sum = f32x4::splat(0.); // [0., 0., 0., 0.]
//     for i in (0..x.len()).step_by(4) {
//         sum += f32x4::from_slice(&x[i..]);
//     }
//     sum.sum()
// }

// fn slow_sum(x: &[f32]) -> f32 {
//     assert!(x.len() % 4 == 0);
//     let mut sum: f32 = 0.;
//     for i in (0..x.len()).step_by(4) {
//         sum += f32x4::from_slice(&x[i..]).iter().sum();
//     }
//     sum
// }

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| fibonacci(test::black_box(20)));
    }

    #[bench]
    fn bench_mult_vectors_without_simd(b: &mut Bencher) {
        // generate a vector of ints ascending 0, 1, 2, ...
        const SIZE: usize = 1_000_000;
        let v1: Vec<u64> = (0u64..SIZE as u64).collect();

        // generate another vector of size 1_000_000, with ints descending
        let v2: Vec<u64> = v1.iter().copied().rev().collect();

        b.iter(|| mult_vectors_without_simd(test::black_box(&v1), test::black_box(&v2)));
    }

    #[bench]
    fn bench_mult_vectors_with_simd(b: &mut Bencher) {
        // generate a vector of ints ascending 0, 1, 2, ...
        const SIZE: usize = 1_000_000;
        let v1: Vec<u64> = (0u64..SIZE as u64).collect();

        // generate another vector of size 1_000_000, with ints descending
        let v2: Vec<u64> = v1.iter().copied().rev().collect();

        b.iter(|| mult_vectors_with_simd(test::black_box(&v1), test::black_box(&v2)));
    }

    #[bench]
    fn bench_div_vectors_without_simd(b: &mut Bencher) {
        // generate a vector of ints ascending 0, 1, 2, ...
        const SIZE: usize = 1_000_000;
        let v1: Vec<f64> = (0u32..SIZE as u32).map(f64::from).collect();

        // generate another vector of size 1_000_000, with ints descending
        let v2: Vec<f64> = v1.iter().copied().rev().collect();

        b.iter(|| div_vectors_without_simd(test::black_box(&v1), test::black_box(&v2)));
    }
}
