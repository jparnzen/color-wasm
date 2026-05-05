use crate::matrix::Vector3;
// use std::f64::consts::PI;

/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn oklab_to_oklch(lab: &Vector3) -> Vector3 {
    const EPSILON: f64 = 0.000_004;

    let [l, a, b] = lab;
    let chroma = (a.powi(2) + b.powi(2)).sqrt();
    let hue = if chroma <= EPSILON {
        f64::NAN
    } else {
        b.atan2(*a).to_degrees().rem_euclid(360.)
    };

    [*l, chroma, hue]
}

/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn oklch_to_oklab(lch: &Vector3) -> Vector3 {
    const EPSILON: f64 = 0.000_004;

    let [l, c, h] = lch;
    if *c <= EPSILON || h.is_nan() {
        return [*l, 0., 0.];
    }

    // let h_prime = h * PI / 180.;
    let h_prime = h.to_radians();
    [*l, c * h_prime.cos(), c * h_prime.sin()]
}

pub fn oklch_to_oklrch(lch: &Vector3) -> Vector3 {
    let [l, c, h] = lch;
    [toe(*l), *c, *h]
}

pub fn oklrch_to_oklch(lrch: &Vector3) -> Vector3 {
    let [l, c, h] = lrch;
    [inv_toe(*l), *c, *h]
}

/// Translated from C++ source from <https://bottosson.github.io/posts/colorpicker/#common-code>
fn toe(x: f64) -> f64 {
    const K1: f64 = 0.206;
    const K2: f64 = 0.03;
    const K3: f64 = (1. + K1) / (1. + K2);

    0.5 * (K3 * x - K1 + ((K3 * x - K1).powi(2) + 4. * K2 * K3 * x).sqrt())
}

/// Translated from C++ source from <https://bottosson.github.io/posts/colorpicker/#common-code>
fn inv_toe(x: f64) -> f64 {
    const K1: f64 = 0.206;
    const K2: f64 = 0.03;
    const K3: f64 = (1. + K1) / (1. + K2);

    (x * x + K1 * x) / (K3 * (x + K2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toe_roundtrip() {
        const EPSILON: f64 = 1e-10;

        let original: f64 = 0.5;
        let forward = toe(original);
        let back = inv_toe(forward);

        assert!((original - back).abs() < EPSILON);
    }

    #[test]
    fn test_oklch_oklab_roundtrip() {
        const EPSILON: f64 = 1e-10;

        let original = [0.5, 0.4, 29.2];
        let forward = oklch_to_oklab(&original);
        let back = oklab_to_oklch(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }

    #[test]
    fn test_oklch_oklrch_roundtrip() {
        const EPSILON: f64 = 1e-10;

        let original = [0.05, 0.4, 29.2];
        let forward = oklrch_to_oklch(&original);
        let back = oklch_to_oklrch(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }
}
