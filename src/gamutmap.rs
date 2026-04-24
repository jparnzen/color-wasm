use crate::matrix::Vector3;

pub fn in_gamut(rgb: &Vector3) -> bool {
    todo!()
}

pub fn clip(rgb: &Vector3) -> Vector3 {
    rgb.map(|c| c.clamp(0., 1.))
}

#[allow(non_snake_case)]
pub fn gamutmap_local_MINDE(rgb: &Vector3) -> Vector3 {
    todo!()
}

pub fn gamutmap_raytrace(rgb: &Vector3) -> Vector3 {
    todo!()
}

fn cast_ray(start_rgb: &Vector3, end_rgb: &Vector3) -> Option<Vector3> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
