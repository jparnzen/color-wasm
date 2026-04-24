use crate::matrix::Vector3;

/// Expects an RGB value, such as all components are in [0., 1.]
pub fn in_gamut(rgb: &Vector3) -> bool {
    rgb.iter().all(|c| *c >= 0. && *c <= 1.)
}

/// Expects an RGB value, such as all components are in [0., 1.]
pub fn clip(rgb: &Vector3) -> Vector3 {
    rgb.map(|c| c.clamp(0., 1.))
}

#[allow(non_snake_case)]
pub fn gamutmap_local_MINDE(rgb: &Vector3) -> Vector3 {
    todo!()
}

fn delta_eok(rgb: &Vector3) -> f64 {
    todo!()
}

pub fn gamutmap_raytrace(rgb: &Vector3) -> Vector3 {
    todo!()
}

fn cast_ray(start: &Vector3, end: &Vector3) -> Option<Vector3> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_gamut() {
        let original = [1., 1., 0.];

        assert_eq!(in_gamut(&original), true);
    }

    #[test]
    fn test_not_in_gamut() {
        let original = [1.02, 0.5, -0.234];

        assert_eq!(in_gamut(&original), false);
    }

    #[test]
    fn test_clip() {
        let original = [1.02, 0.5, -0.234];

        assert_eq!(clip(&original), [1., 0.5, 0.]);
    }
}
