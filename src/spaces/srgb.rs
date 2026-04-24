use crate::matrix::{Matrix3, Vector3, matrix3_mult_vector3};

/// Remove sRGB gamma-encoding from a color component (r, g, or b channel of sRGB).
/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
fn linearize(c: f64) -> f64 {
    let abs_c = c.abs();

    if abs_c <= 0.04045 {
        c / 12.92
    } else {
        ((abs_c + 0.055) / 1.055).powf(2.4).copysign(c)
    }
}

/// Gamma-encode a linear sRGB color component (r, g, or b channel of sRGB).
/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
fn gamma_encode(c: f64) -> f64 {
    let abs_c = c.abs();

    if abs_c <= 0.0031308 {
        c * 12.92
    } else {
        (1.055 * abs_c.powf(1.0 / 2.4) - 0.055).copysign(c)
    }
}

/// Transform a gamma-encoded sRGB color into linear sRGB.
pub fn srgb_to_linear_srgb(rgb: &Vector3) -> Vector3 {
    rgb.map(linearize)
}

/// Transform a linear sRGB color into a gamma-encoded one.
pub fn linear_srgb_to_rgb(lrgb: &Vector3) -> Vector3 {
    lrgb.map(gamma_encode)
}

/// Transform a linear sRGB color to the XYZ color space.
/// This is usually done as part of a larger transformation to
/// another color space, such as OKLAB.
/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn linear_srgb_to_xyz(lrgb: &Vector3) -> Vector3 {
    const LINEAR_SRGB_TO_XYZD65: Matrix3 = [
        [506752. / 1228815., 87881. / 245763., 12673. / 70218.],
        [87098. / 409605., 175762. / 245763., 12673. / 175545.],
        [7918. / 409605., 87881. / 737289., 1001167. / 1053270.],
    ];

    matrix3_mult_vector3(&LINEAR_SRGB_TO_XYZD65, lrgb)
}

/// Transform a color from the XYZ color space to the linear sRGB
/// color space, usually on the way to the gamma-encoded sRGB space.
/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn xyz_to_linear_srgb(xyz: &Vector3) -> Vector3 {
    const XYZD65_TO_LINEAR_SRGB: Matrix3 = [
        [12831. / 3959., -329. / 214., -1974. / 3959.],
        [-851781. / 878810., 1648619. / 878810., 36519. / 878810.],
        [705. / 12673., -2585. / 12673., 705. / 667.],
    ];

    matrix3_mult_vector3(&XYZD65_TO_LINEAR_SRGB, xyz)
}

/// Transform a gamma-encoded sRGB color to the XYZ space, first
/// by transforming to linear sRGB and then to XYZ. This is usually
/// the first part of a larger transformation to another color space
/// such as OKLAB.
pub fn srgb_to_xyz(rgb: &Vector3) -> Vector3 {
    let lrgb = srgb_to_linear_srgb(rgb);
    linear_srgb_to_xyz(&lrgb)
}

/// Transform a color from the XYZ color space to the gamma-encoded
/// sRGB color space, via the linear sRGB color space. This is usually
/// done as part of a transformation from another space such as OKLAB
/// to sRGB.
pub fn xyz_to_srgb(xyz: &Vector3) -> Vector3 {
    let lrgb = xyz_to_linear_srgb(xyz);
    linear_srgb_to_rgb(&lrgb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_linear_srgb_roundtrip() {
        const EPSILON: f64 = 1e-10;

        // #0088c6 — verified against colorjs.io
        let original = [0.0, 0.533, 0.776];
        let forward = srgb_to_linear_srgb(&original);
        let back = linear_srgb_to_rgb(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }

    #[test]
    fn test_srgb_xyz_roundtrip() {
        const EPSILON: f64 = 1e-10;

        // #0088c6 — verified against colorjs.io
        let original = [0.0, 0.533, 0.776];
        let forward = srgb_to_xyz(&original);
        let back = xyz_to_srgb(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }

    #[test]
    fn test_linear_srgb_xyz_roundtrip() {
        const EPSILON: f64 = 1e-10;

        // #0088c6 — verified against colorjs.io
        let original = srgb_to_linear_srgb(&[0.0, 0.533, 0.776]);
        let forward = linear_srgb_to_xyz(&original);
        let back = xyz_to_linear_srgb(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }
}
