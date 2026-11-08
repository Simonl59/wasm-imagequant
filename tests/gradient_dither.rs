use wasm_imagequant::quantize_and_compress;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 64;
const STOPS: [[f32; 3]; 3] = [[20.0, 40.0, 120.0], [140.0, 60.0, 170.0], [250.0, 150.0, 60.0]];

// Two-axis, three-stop gradient: thousands of colours with no noise to hide banding.
fn gradient() -> Vec<u8> {
    let mut rgba = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let t = x as f32 / (WIDTH - 1) as f32 * (STOPS.len() - 1) as f32;
            let stop = (t.floor() as usize).min(STOPS.len() - 2);
            let (from, to) = (STOPS[stop], STOPS[stop + 1]);
            for c in 0..3 {
                let value = from[c] + (to[c] - from[c]) * (t - stop as f32) + y as f32 / HEIGHT as f32 * 20.0;
                rgba.push(value.round().clamp(0.0, 255.0) as u8);
            }
            rgba.push(255);
        }
    }
    rgba
}

#[test]
fn dithers_a_smooth_gradient_instead_of_banding_it() {
    let png = quantize_and_compress(&gradient(), WIDTH, HEIGHT, 256, 1.0).unwrap();
    let pixels = image::load_from_memory(&png).unwrap().to_rgba8();

    // Share of side-by-side pixels with different colours: dithering mixes neighbours, banding leaves flat runs.
    let mut changes = 0;
    for y in 0..HEIGHT {
        for x in 1..WIDTH {
            if pixels.get_pixel(x, y) != pixels.get_pixel(x - 1, y) {
                changes += 1;
            }
        }
    }
    let share = changes as f32 / ((WIDTH - 1) * HEIGHT) as f32;

    // About 0.65 with full error diffusion; upstream's adaptive damping bands it at about 0.16.
    assert!(share >= 0.4, "gradient is banded: only {share:.2} of neighbours differ");
}
