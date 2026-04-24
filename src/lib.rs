use wasm_bindgen::prelude::*;

// mod gamutmap;
mod matrix;
mod spaces;

use matrix::Vector3;
use spaces::{oklab, oklch, srgb};

#[wasm_bindgen]
pub fn srgb_to_oklch(r: f32, g: f32, b: f32) -> Vec<f32> {
    let rgb: Vector3 = [f64::from(r), f64::from(g), f64::from(b)];

    let xyz = srgb::srgb_to_xyz(&rgb);
    let oklab = oklab::xyz_to_oklab(&xyz);
    let [l, c, h] = oklch::oklab_to_oklch(&oklab);

    vec![l as f32, c as f32, h as f32]
}

#[wasm_bindgen]
pub fn oklch_to_srgb(l: f32, c: f32, h: f32) -> Vec<f32> {
    let lch: Vector3 = [f64::from(l), f64::from(c), f64::from(h)];

    let oklab = oklch::oklch_to_oklab(&lch);
    let xyz = oklab::oklab_to_xyz(&oklab);
    let [r, g, b] = srgb::xyz_to_srgb(&xyz);

    vec![r as f32, g as f32, b as f32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        // f32 precision is around 7 decimal places
        const EPSILON: f32 = 1e-5;

        let rgb = [1., 1., 0.];

        let lch = srgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        let back = oklch_to_srgb(lch[0], lch[1], lch[2]);

        for (a, b) in rgb.iter().zip(back.iter()) {
            println!("a - b = {}", (a - b).abs());
            assert!((a - b).abs() < EPSILON);
        }
    }
}
