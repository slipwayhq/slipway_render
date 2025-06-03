use image::{Rgba, RgbaImage};

/// Blend a premultiplied-alpha source into a premultiplied-alpha destination
pub(super) fn blend_premultiplied(dst: &mut Rgba<u8>, src: Rgba<u8>) {
    let sa = src[3] as f32 / 255.0;
    let da = dst[3] as f32 / 255.0;

    let inv_sa = 1.0 - sa;

    for i in 0..3 {
        let s = src[i] as f32;
        let d = dst[i] as f32;
        dst[i] = ((s + d * inv_sa).clamp(0.0, 255.0)) as u8;
    }

    dst[3] = ((sa + da * inv_sa) * 255.0).clamp(0.0, 255.0) as u8;
}

pub fn image_to_premultiplied_alpha(mut img: RgbaImage) -> RgbaImage {
    for px in img.pixels_mut() {
        px.0 = pixel_to_premultiplied_alpha(*px).0;
    }
    img
}

pub fn pixel_to_premultiplied_alpha(pixel: Rgba<u8>) -> Rgba<u8> {
    let [r, g, b, a] = pixel.0;
    let af = a as f32 / 255.0;
    Rgba([
        (r as f32 * af).round() as u8,
        (g as f32 * af).round() as u8,
        (b as f32 * af).round() as u8,
        a,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba<u8> {
        Rgba([r, g, b, a])
    }

    #[test]
    fn blends_fully_opaque_over_fully_transparent() {
        let mut dst = rgba(0, 0, 0, 0);
        let src = rgba(255, 0, 0, 255); // fully opaque red
        blend_premultiplied(&mut dst, src);
        assert_eq!(dst, rgba(255, 0, 0, 255));
    }

    #[test]
    fn blends_fully_transparent_over_opaque() {
        let mut dst = rgba(0, 255, 0, 255); // fully opaque green
        let src = rgba(0, 0, 0, 0); // fully transparent (ignored)
        blend_premultiplied(&mut dst, src);
        assert_eq!(dst, rgba(0, 255, 0, 255));
    }

    #[test]
    fn blends_half_alpha_red_over_half_alpha_blue() {
        let mut dst = rgba(0, 0, 128, 128); // half blue, premultiplied
        let src = rgba(128, 0, 0, 128); // half red, premultiplied
        blend_premultiplied(&mut dst, src);
        // Expected: red + blue * (1 - 0.5) = (128, 0, 0) + (0, 0, 128) * 0.5 = (128, 0, 64)
        // Actual outputs are slightly different because 128 isn't half of 255.
        assert_eq!(dst, rgba(128, 0, 63, 191)); // alpha: 0.5 + 0.5*(1-0.5) = 0.75
    }

    #[test]
    fn blends_partial_alpha_preserves_math() {
        let mut dst = rgba(100, 100, 100, 128); // premultiplied
        let src = rgba(128, 0, 0, 128); // premultiplied red
        blend_premultiplied(&mut dst, src);
        // Output: r = 128 + 100*(1-0.5) = 128 + 50 = 178
        //         g = 0 + 100*0.5 = 50
        //         b = 0 + 100*0.5 = 50
        //         a = 128 + 128*0.5 = 192
        // Actual outputs are slightly different because 128 isn't half of 255.
        assert_eq!(dst, rgba(177, 49, 49, 191));
    }

    #[test]
    fn pixel_to_premultiplied_alpha_fully_opaque() {
        let pixel = rgba(255, 128, 64, 255);
        let result = pixel_to_premultiplied_alpha(pixel);
        assert_eq!(result, rgba(255, 128, 64, 255));
    }

    #[test]
    fn pixel_to_premultiplied_alpha_fully_transparent() {
        let pixel = rgba(255, 128, 64, 0);
        let result = pixel_to_premultiplied_alpha(pixel);
        assert_eq!(result, rgba(0, 0, 0, 0));
    }

    #[test]
    fn pixel_to_premultiplied_alpha_half_alpha() {
        let pixel = rgba(255, 128, 64, 128);
        let result = pixel_to_premultiplied_alpha(pixel);
        // 128/255 ≈ 0.502, so we expect:
        // r: 255 * 0.502 ≈ 128 (rounded)
        // g: 128 * 0.502 ≈ 64 (rounded)
        // b: 64 * 0.502 ≈ 32 (rounded)
        assert_eq!(result, rgba(128, 64, 32, 128));
    }

    #[test]
    fn pixel_to_premultiplied_alpha_preserves_alpha() {
        let pixel = rgba(100, 50, 200, 200);
        let result = pixel_to_premultiplied_alpha(pixel);
        assert_eq!(result.0[3], 200); // alpha should be unchanged
    }

    #[test]
    fn image_to_premultiplied_alpha_transforms_all_pixels() {
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, rgba(255, 0, 0, 128));
        img.put_pixel(1, 0, rgba(0, 255, 0, 64));
        img.put_pixel(0, 1, rgba(0, 0, 255, 192));
        img.put_pixel(1, 1, rgba(255, 255, 255, 255));

        let result = image_to_premultiplied_alpha(img);

        // Check that each pixel has been premultiplied
        assert_eq!(result.get_pixel(0, 0), &rgba(128, 0, 0, 128));
        assert_eq!(result.get_pixel(1, 0), &rgba(0, 64, 0, 64));
        assert_eq!(result.get_pixel(0, 1), &rgba(0, 0, 192, 192));
        assert_eq!(result.get_pixel(1, 1), &rgba(255, 255, 255, 255));
    }

    #[test]
    fn image_to_premultiplied_alpha_empty_image() {
        let img = RgbaImage::new(0, 0);
        let result = image_to_premultiplied_alpha(img);
        assert_eq!(result.width(), 0);
        assert_eq!(result.height(), 0);
    }
}
