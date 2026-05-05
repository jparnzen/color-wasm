use crate::matrix::Vector3;
use crate::spaces::{oklab, oklch};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ColorSpace {
    SRGB,
    SRGBLinear,
    OKLAB,
    OKLCH,
    OKLrAB,
    OKLrCH,
    XYZD65,
}

impl ColorSpace {
    fn is_rgb(&self) -> bool {
        if matches!(self, Self::SRGB | Self::SRGBLinear) {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    pub space: ColorSpace,
    pub coords: Vector3,
}

impl Color {
    fn new(space: ColorSpace, coords: Vector3) -> Self {
        Color { space, coords }
    }

    fn convert(&self, dest_color_space: ColorSpace) -> Self {
        if self.space == dest_color_space {
            return *self;
        }

        let to_xyz = match self.space {
            ColorSpace::SRGB => srgb::srgb_to_xyz(&self.coords),
            ColorSpace::SRGBLinear => srgb::linear_srgb_to_xyz(&self.coords),
            ColorSpace::OKLAB => oklab::oklab_to_xyz(&self.coords),
            ColorSpace::OKLCH => oklab::oklab_to_xyz(&oklch::oklch_to_oklab(&self.coords)),
            ColorSpace::XYZD65 => self.coords,
        };

        let from_xyz = match dest_color_space {
            ColorSpace::SRGB => srgb::xyz_to_srgb(&to_xyz),
            ColorSpace::SRGBLinear => srgb::xyz_to_linear_srgb(&to_xyz),
            ColorSpace::OKLAB => oklab::xyz_to_oklab(&to_xyz),
            ColorSpace::OKLCH => oklch::oklab_to_oklch(&oklab::xyz_to_oklab(&to_xyz)),
            ColorSpace::XYZD65 => to_xyz,
        };

        Color {
            space: dest_color_space,
            coords: from_xyz,
        }
    }

    fn is_rgb(&self) -> bool {
        if matches!(self.space, ColorSpace::SRGB | ColorSpace::SRGBLinear) {
            true
        } else {
            false
        }
    }
}

/// 5. let inGamut(color) be a function which returns true if, when passed a color, that color is inside the gamut of destination. For HSL and HWB, it returns true if the color is inside the gamut of sRGB.
/// Expects an RGB value, such as all components are in [0., 1.].
/// Non-RGB colors are considered in-gamut by default.
pub fn in_gamut(color: &Color, dest_color_space: ColorSpace) -> bool {
    let temp_color = color.convert(dest_color_space);

    if temp_color.is_rgb() {
        temp_color.coords.iter().all(|c| *c >= 0. && *c <= 1.)
    } else {
        true
    }
}

/// 10. let clip(color) be a function which converts color to destination, clamps each component to the bounds of the reference range for that component and returns the result
/// Expects an RGB value, such as all components are in [0., 1.]
pub fn clip(color: &Color, dest_color_space: ColorSpace) -> Color {
    let new_color = color.convert(dest_color_space);

    if new_color.is_rgb() {
        Color::new(new_color.space, new_color.coords.map(|c| c.clamp(0., 1.)))
    } else {
        new_color
    }
}

/// <https://drafts.csswg.org/css-color-4/#pseudo-binsearch>
#[allow(non_snake_case)]
pub fn gamutmap_local_MINDE(origin: &Color, dest_color_space: ColorSpace) -> Color {
    // 1. if destination has no gamut limits (XYZ-D65, XYZ-D50, Lab, LCH, Oklab, OkLCh) convert origin to destination and return it as the gamut mapped color
    if matches!(
        dest_color_space,
        ColorSpace::XYZD65 | ColorSpace::OKLAB | ColorSpace::OKLCH
    ) {
        return origin.convert(dest_color_space);
    }

    // 2. let origin_OkLCh be origin converted from origin color space to the OkLCh color space
    let origin_oklch = origin.convert(ColorSpace::OKLCH);

    // 3. if the Lightness of origin_OkLCh is greater than or equal to 100%, convert `oklab(1 0 0 / origin.alpha)` to destination and return it as the gamut mapped color
    if origin_oklch.coords[0] >= 1. {
        return Color::new(ColorSpace::OKLAB, [1., 0., 0.]).convert(dest_color_space);
    }

    // 4. if the Lightness of origin_OkLCh is less than than or equal to 0%, convert `oklab(0 0 0 / origin.alpha)` to destination and return it as the gamut mapped color
    if origin_oklch.coords[0] <= 0. {
        return Color::new(ColorSpace::OKLAB, [0., 0., 0.]).convert(dest_color_space);
    }

    // 5. let inGamut(color) be a function which returns true if, when passed a color, that color is inside the gamut of destination. For HSL and HWB, it returns true if the color is inside the gamut of sRGB.
    // 6. if inGamut(origin_OkLCh) is true, convert origin_OkLCh to destination and return it as the gamut mapped color
    if in_gamut(origin, dest_color_space) {
        return origin_oklch.convert(dest_color_space);
    }

    // 7. otherwise, let delta(one, two) be a function which returns the deltaEOK of color one compared to color two
    // 8. let JND be 0.02
    const JND: f64 = 0.02;

    // 9. let epsilon be 0.0001
    const EPSILON: f64 = 0.0001;

    // 10. let clip(color) be a function which converts color to destination, clamps each component to the bounds of the reference range for that component and returns the result
    // 11. set current to origin_OkLCh
    let mut current = origin_oklch;

    // 12. set clipped to clip(current)
    let mut clipped = clip(&current, dest_color_space);

    // 13. set E to delta(clipped, current)
    let mut e = delta_eok(&clipped, &current);

    // 14. if E < JND
    //     14.1. return clipped as the gamut mapped color
    if e < JND {
        return clipped;
    }

    // 15. set min to zero
    let mut min = 0.;

    // 16. set max to the OkLCh chroma of origin_OkLCh
    let mut max = origin_oklch.coords[1];

    // 17. let min_inGamut be a boolean that represents when min is still in gamut, and set it to true
    let mut min_in_gamut = true;

    // 18. while (max - min is greater than epsilon) repeat the following steps
    while (max - min) > EPSILON {
        // 18.1. set chroma to (min + max) /2
        let chroma = (min + max) / 2.;

        // 18.2. set the chroma component of current to chroma
        current.coords[1] = chroma;

        // 18.3. if min_inGamut is true and also if inGamut(current) is true, set min to chroma and continue to repeat these steps
        if min_in_gamut && in_gamut(&current, dest_color_space) {
            min = chroma;
            continue;
        }

        // 18.4. otherwise, carry out these steps:
        // 18.4.1. set clipped to clip(current)
        clipped = clip(&current, dest_color_space);

        // 18.4.2. set E to delta(clipped, current)
        e = delta_eok(&clipped, &current);

        // 18.4.3. if E < JND
        if e < JND {
            // 18.4.3.1. if (JND - E < epsilon) return clipped as the gamut mapped color
            if (JND - e) < EPSILON {
                return clipped;
            }

            // 18.4.3.2. otherwise,
            // 18.4.3.2.1. set min_inGamut to false
            min_in_gamut = false;

            // 18.4.3.2.2. set min to chroma
            min = chroma;
        }
        // 18.4.4. otherwise, set max to chroma and continue to repeat these steps
        else {
            max = chroma;
        }
    }

    // 19. return clipped as the gamut mapped color
    clipped
}

/// <https://drafts.csswg.org/css-color-4/#color-difference-OK>
fn delta_eok(reference: &Color, sample: &Color) -> f64 {
    // make sure both colors are in OKLAB space for distance calculation
    let oklab_reference = reference.convert(ColorSpace::OKLAB);
    let oklab_sample = sample.convert(ColorSpace::OKLAB);

    let [l1, a1, b1] = oklab_reference.coords;
    let [l2, a2, b2] = oklab_sample.coords;

    let (delta_l, delta_a, delta_b) = (l1 - l2, a1 - a2, b1 - b2);

    (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt()
}

/// <https://drafts.csswg.org/css-color-4/#pseudo-raytrace>
pub fn gamutmap_raytrace(color: &Vector3) -> Vector3 {
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
        // let original = [1., 1., 0.];
        let original = Color::new(ColorSpace::SRGB, [1., 1., 0.]);

        assert_eq!(in_gamut(&original, ColorSpace::SRGB), true);
    }

    #[test]
    fn test_not_in_gamut() {
        let original = Color::new(ColorSpace::SRGB, [1.02, 0.5, -0.234]);

        assert_eq!(in_gamut(&original, ColorSpace::SRGB), false);
    }

    #[test]
    fn test_clip() {
        let original = Color::new(ColorSpace::SRGB, [1.02, 0.5, -0.234]);

        assert_eq!(
            clip(&original, ColorSpace::SRGB),
            Color::new(ColorSpace::SRGB, [1., 0.5, 0.])
        );
    }
}
