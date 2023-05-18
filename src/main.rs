#![feature(portable_simd)]
use std::simd::u64x4;

fn simd_experiment() {
    // generate a vector of size 100_000_000, with ints ascending 0, 1, 2, ...
    const SIZE: usize = 100_000_000;
    let v1: Vec<u64> = (0u64..SIZE as u64).collect();

    // generate another vector of size 1_000_000, with ints descending
    let v2: Vec<u64> = v1.iter().copied().rev().collect();

    println!("{}", "start");

    // run without simd
    {
        let v3 = mult_vectors_without_simd(&v1, &v2);
        println!("{}", v3[500]);
    }

    // run with simd
    {
        let v4 = mult_vectors_with_simd(&v1, &v2);
        println!("{}", v4[500]);
    }

    // TODO: benchmark
}

fn mult_vectors_without_simd(vec1: &Vec<u64>, vec2: &Vec<u64>) -> Vec<u64> {
    let mut vec3 = vec![];
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        vec3.push(v1 * v2);
    }
    vec3
}

fn mult_vectors_with_simd(vec1: &Vec<u64>, vec2: &Vec<u64>) -> Vec<u64> {
    let mut vec3 = vec![];
    for (v1, v2) in vec1.chunks(4).zip(vec2.chunks(4)) {
        let simd_v1 = u64x4::from_slice(v1);
        let simd_v2 = u64x4::from_slice(v2);
        let simd_v3 = simd_v1 * simd_v2;
        vec3.extend(simd_v3.to_array());
    }
    vec3
}

fn main() {
    simd_experiment();
}
