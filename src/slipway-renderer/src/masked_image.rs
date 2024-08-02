use image::Rgba;
use imageproc::drawing::Canvas;

use crate::rect::Rect;

// Define the MaskedImage struct
pub(super) struct MaskedImage<'a> {
    image: &'a mut dyn Canvas<Pixel = Rgba<u8>>,
    mask: Rect,
}

impl<'a> MaskedImage<'a> {
    pub fn new(image: &'a mut dyn Canvas<Pixel = Rgba<u8>>, mask: Rect) -> Self {
        Self { image, mask }
    }
}

// Implement the Canvas trait for MaskedImage
impl<'a> Canvas for MaskedImage<'a> {
    type Pixel = Rgba<u8>;

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        // Only modify pixels within the mask
        if x >= self.mask.x
            && x < self.mask.x + self.mask.width
            && y >= self.mask.y
            && y < self.mask.y + self.mask.height
        {
            self.image.draw_pixel(x, y, pixel);
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        // If the pixel is outside the mask, return a transparent pixel
        if x < self.mask.x
            || x >= self.mask.x + self.mask.width
            || y < self.mask.y
            || y >= self.mask.y + self.mask.height
        {
            return Rgba([0, 0, 0, 0]);
        }

        // Otherwise, return the pixel from the image
        self.image.get_pixel(x, y)
    }

    fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }
}
