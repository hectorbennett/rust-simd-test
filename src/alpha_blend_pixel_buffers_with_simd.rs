use std::simd::f32x4;

/* take in an */
pub fn alpha_blend_pixel_buffers_with_simd(buffers: &Vec<&Vec<f32>>) -> Vec<f32> {
    let mut output: Vec<f32> = buffers[0].clone();
    for buffer in buffers.iter().skip(1) {
        for (i, pixel) in buffer.chunks(4).enumerate() {
            let p: [f32; 4] = blend_pixels_with_simd(&output[i..i + 4], pixel);
            output[i] = p[0];
            output[i + 1] = p[1];
            output[i + 2] = p[2];
            output[i + 3] = p[3];
        }
    }
    output
}

pub fn blend_pixels_with_simd(pixel_bg: &[f32], pixel_fg: &[f32]) -> [f32; 4] {
    /* rgba blend 2 pixels */
    if pixel_fg[3] == 1.0 || pixel_bg[3] == 0.0 {
        return pixel_fg.try_into().unwrap();
    }
    let alpha_bg: f32 = pixel_bg[3];
    let alpha_fg: f32 = pixel_fg[3];

    let alpha_final = alpha_bg + alpha_fg - alpha_bg * alpha_fg;

    if alpha_final == 0.0 {
        return [0.0, 0.0, 0.0, 0.0];
    }

    let [r, g, b] = blend_colour_channels(pixel_bg, pixel_fg, alpha_final);

    [r, g, b, alpha_final]
}

// #[inline(always)]
// fn blend_alpha(alpha_bg: f32, alpha_fg: f32) -> f32 {
//     alpha_bg + alpha_fg - alpha_bg * alpha_fg
// }

#[inline(always)]
fn blend_colour_channels(pixel_bg: &[f32], pixel_fg: &[f32], alpha_final: f32) -> [f32; 3] {
    let alpha_bg = pixel_bg[3];
    let alpha_fg = pixel_fg[3];
    let simd_pixel_bg = f32x4::from_slice(pixel_bg);
    let simd_pixel_fg = f32x4::from_slice(pixel_fg);
    let thing = (1.0 - alpha_fg) / alpha_final;
    let new = (simd_pixel_fg * f32x4::splat(alpha_fg))
        + (simd_pixel_bg * f32x4::splat(alpha_bg)) * f32x4::splat(thing);
    [new[0], new[1], new[2]]
}

#[test]
fn test_select_rect_out_of_bounds() {
    let pixel_buffer_1: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let pixel_buffer_2: Vec<f32> = vec![0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.1];
    let pixel_buffer_3: Vec<f32> = vec![0.5, 0.6, 0.7, 0.8, 0.9, 0.1, 0.2, 0.3];

    let pixel_buffers: Vec<&Vec<f32>> = vec![&pixel_buffer_1, &pixel_buffer_2, &pixel_buffer_3];

    assert_eq!(
        alpha_blend_pixel_buffers_with_simd(&pixel_buffers),
        vec![0.48333335, 0.67645156, 0.50419354, 0.60193545, 0.93000007, 0.6, 0.7, 0.8]
    );
}
