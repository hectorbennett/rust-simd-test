use std::simd::u64x4;

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
