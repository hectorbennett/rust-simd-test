/* take in an */
pub fn alpha_blend_pixel_buffers(buffers: &Vec<&Vec<f32>>) -> Vec<f32> {
    let mut output: Vec<f32> = buffers[0].clone();
    for buffer in buffers.iter().skip(1) {
        for (i, pixel) in buffer.chunks(4).enumerate() {
            let bg: [f32; 4] = buffer[i..i+4].try_into().unwrap();
            let fg: [f32; 4] = pixel.try_into().unwrap();
            let new_pixel = blend_pixels(&bg, &fg);

            output[i] = new_pixel[0];
            output[i + 1] = new_pixel[1];
            output[i + 2] = new_pixel[2];
            output[i + 3] = new_pixel[3];
        }
    }
    output
}

pub fn blend_pixels(pixel_bg: &[f32; 4], pixel_fg: &[f32; 4]) -> [f32; 4] {
    /* rgba blend 2 pixels */
    if pixel_fg[3] == 1.0 || pixel_bg[3] == 0.0 {
        return pixel_fg.clone();
    }
    let alpha_fg: f32 = pixel_fg[3];

    let red_fg: f32 = pixel_fg[0];
    let green_fg: f32 = pixel_fg[1];
    let blue_fg: f32 = pixel_fg[2];

    let red_bg: f32 = pixel_bg[0];
    let green_bg: f32 = pixel_bg[1];
    let blue_bg: f32 = pixel_bg[2];
    let alpha_bg: f32 = pixel_bg[3];

    let alpha_final = blend_alpha(alpha_fg, alpha_bg);

    if alpha_final == 0.0 {
        return [0.0, 0.0, 0.0, 0.0];
    }

    let red_final: f32 = blend_colour_channel(red_fg, red_bg, alpha_fg, alpha_bg, alpha_final);
    let green_final: f32 =
        blend_colour_channel(green_fg, green_bg, alpha_fg, alpha_bg, alpha_final);
    let blue_final: f32 = blend_colour_channel(blue_fg, blue_bg, alpha_fg, alpha_bg, alpha_final);

    [red_final, green_final, blue_final, alpha_final]
}

#[inline(always)]
fn blend_alpha(alpha_fg: f32, alpha_bg: f32) -> f32 {
    alpha_bg + alpha_fg - alpha_bg * alpha_fg
}

#[inline(always)]
fn blend_colour_channel(
    colour_fg: f32,
    colour_bg: f32,
    alpha_fg: f32,
    alpha_bg: f32,
    alpha_final: f32,
) -> f32 {
    (colour_fg * alpha_fg) + (colour_bg * alpha_bg) * (1.0 - alpha_fg) / alpha_final
}

#[test]
fn test_select_rect_out_of_bounds() {
    let pixel_buffer_1: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    let pixel_buffer_2: Vec<f32> = vec![0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.1];
    let pixel_buffer_3: Vec<f32> = vec![0.5, 0.6, 0.7, 0.8, 0.9, 0.1, 0.2, 0.3];

    let pixel_buffers: Vec<&Vec<f32>> = vec![&pixel_buffer_1, &pixel_buffer_2, &pixel_buffer_3];

    assert_eq!(
        alpha_blend_pixel_buffers(&pixel_buffers),
        vec![0.48333335, 0.6764516, 0.5041935, 0.6019354, 0.93000007, 0.6, 0.7, 0.8]
    );
}
