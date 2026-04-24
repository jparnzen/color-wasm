use crate::matrix::{Matrix3, Vector3, matrix3_mult_vector3};

/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn xyz_to_oklab(xyz: &Vector3) -> Vector3 {
    const XYZD65_TO_LMS: Matrix3 = [
        [0.8190224379967030, 0.3619062600528904, -0.1288737815209879],
        [0.0329836539323885, 0.9292868615863434, 0.0361446663506424],
        [0.0481771893596242, 0.2642395317527308, 0.6335478284694309],
    ];
    const LMS_TO_OKLAB: Matrix3 = [
        [0.2104542683093140, 0.7936177747023054, -0.0040720430116193],
        [1.9779985324311684, -2.4285922420485799, 0.4505937096174110],
        [0.0259040424655478, 0.7827717124575296, -0.8086757549230774],
    ];

    let lms = matrix3_mult_vector3(&XYZD65_TO_LMS, xyz);
    matrix3_mult_vector3(&LMS_TO_OKLAB, &lms.map(|c| c.cbrt()))
}

/// Translated from CSS Color 4 sample javascript code.
/// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
pub fn oklab_to_xyz(lab: &Vector3) -> Vector3 {
    const OKLAB_TO_LMS: Matrix3 = [
        [1.0000000000000000, 0.3963377773761749, 0.2158037573099136],
        [1.0000000000000000, -0.1055613458156586, -0.0638541728258133],
        [1.0000000000000000, -0.0894841775298119, -1.2914855480194092],
    ];
    const LMS_TO_XYZD65: Matrix3 = [
        [1.2268798758459243, -0.5578149944602171, 0.2813910456659647],
        [-0.0405757452148008, 1.1122868032803170, -0.0717110580655164],
        [-0.0763729366746601, -0.4214933324022432, 1.5869240198367816],
    ];

    let lms = matrix3_mult_vector3(&OKLAB_TO_LMS, lab);
    matrix3_mult_vector3(&LMS_TO_XYZD65, &lms.map(|c| c.powi(3)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oklab_xyz_roundtrip() {
        const EPSILON: f64 = 1e-10;

        let original = [0.5, 0.2, -0.05];

        let forward = oklab_to_xyz(&original);
        let back = xyz_to_oklab(&forward);

        for (a, b) in original.iter().zip(back.iter()) {
            assert!((a - b).abs() < EPSILON);
        }
    }
}
