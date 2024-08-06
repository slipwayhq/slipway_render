use std::{cell::RefCell, rc::Rc};

use crate::rect::SlipwayRegion;
use image::{Rgba, RgbaImage};
use imageproc::{drawing::Canvas, rect::Rect};

enum MaskOrImage {
    Mask(Rc<RefCell<MaskedImage>>),
    Image(Rc<RefCell<RgbaImage>>),
}

pub(super) struct MaskedImage {
    image: MaskOrImage,
    mask: Rect,
}

impl MaskedImage {
    pub fn from_image(image: Rc<RefCell<RgbaImage>>) -> Rc<RefCell<Self>> {
        let (width, height) = image.borrow().dimensions();
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Image(image.clone()),
            mask: Rect::at(0, 0).of_size(width, height),
        }))
    }
    pub fn from_mask(image: Rc<RefCell<MaskedImage>>, mask: Rect) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Mask(image.clone()),
            mask,
        }))
    }
}

trait Maskable {
    fn mask(self, mask: Rect) -> Rc<RefCell<MaskedImage>>;
}

impl Maskable for Rc<RefCell<MaskedImage>> {
    fn mask(self, mask: Rect) -> Rc<RefCell<MaskedImage>> {
        Rc::new(RefCell::new(MaskedImage {
            image: MaskOrImage::Mask(self.clone()),
            mask,
        }))
    }
}

// Implement the Canvas trait for MaskedImage
impl Canvas for MaskedImage {
    type Pixel = Rgba<u8>;

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        if self.mask.contains(x, y) {
            match &mut self.image {
                MaskOrImage::Mask(mask) => mask.borrow_mut().draw_pixel(x, y, pixel),
                MaskOrImage::Image(image) => image.borrow_mut().put_pixel(x, y, pixel),
            }
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        // If the pixel is outside the mask, return a transparent pixel
        if !self.mask.contains(x, y) {
            return Rgba([0, 0, 0, 0]);
        }

        match &self.image {
            MaskOrImage::Mask(mask) => mask.borrow().get_pixel(x, y),
            MaskOrImage::Image(image) => *image.borrow().get_pixel(x, y),
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        match &self.image {
            MaskOrImage::Mask(mask) => mask.borrow().dimensions(),
            MaskOrImage::Image(image) => image.borrow().dimensions(),
        }
    }
}

pub(super) trait SlipwayCanvas {
    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>);
    fn get_pixel(&self, x: u32, y: u32) -> Rgba<u8>;
    fn dimensions(&self) -> (u32, u32);
}

impl SlipwayCanvas for Rc<RefCell<MaskedImage>> {
    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        self.borrow_mut().draw_pixel(x, y, pixel);
    }

    fn get_pixel(&self, x: u32, y: u32) -> Rgba<u8> {
        self.borrow().get_pixel(x, y)
    }

    fn dimensions(&self) -> (u32, u32) {
        self.borrow().dimensions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};
    use std::cell::RefCell;
    use std::rc::Rc;

    // Helper function to create a new RgbaImage
    fn create_image(width: u32, height: u32, color: Rgba<u8>) -> Rc<RefCell<RgbaImage>> {
        let image = ImageBuffer::from_fn(width, height, |_, _| color);
        Rc::new(RefCell::new(image))
    }

    // Test creating MaskedImage from an RgbaImage
    #[test]
    fn test_create_from_image() {
        let width = 10;
        let height = 10;
        let image = create_image(width, height, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);

        let masked_image = MaskedImage::from_image(image.clone()).mask(mask);

        // Check initial dimensions
        assert_eq!(masked_image.borrow().dimensions(), (width, height));

        // Ensure the mask is set correctly
        let m = masked_image.borrow();
        assert_eq!(m.mask.left(), 2);
        assert_eq!(m.mask.top(), 2);
        assert_eq!(m.mask.width(), 5);
        assert_eq!(m.mask.height(), 5);
    }

    // Test creating MaskedImage from another MaskedImage
    #[test]
    fn test_create_from_mask() {
        let base_image = create_image(10, 10, Rgba([255, 0, 0, 255]));
        let base_mask = Rect::at(0, 0).of_size(10, 10);
        let base_masked_image = MaskedImage::from_image(base_image).mask(base_mask);

        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image = MaskedImage::from_mask(base_masked_image.clone(), mask);

        // Check dimensions remain the same
        assert_eq!(masked_image.borrow().dimensions(), (10, 10));

        // Ensure the mask is set correctly
        let m = masked_image.borrow();
        assert_eq!(m.mask.left(), 2);
        assert_eq!(m.mask.top(), 2);
        assert_eq!(m.mask.width(), 5);
        assert_eq!(m.mask.height(), 5);
    }

    // Test drawing pixels inside and outside the mask
    #[test]
    fn test_draw_pixel() {
        let image = create_image(10, 10, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image = MaskedImage::from_image(image.clone()).mask(mask);

        // Draw a pixel inside the mask
        masked_image
            .borrow_mut()
            .draw_pixel(3, 3, Rgba([0, 0, 0, 255]));
        assert_eq!(image.borrow().get_pixel(3, 3), &Rgba([0, 0, 0, 255]));

        // Draw a pixel outside the mask, should not change
        masked_image
            .borrow_mut()
            .draw_pixel(1, 1, Rgba([0, 0, 0, 255]));
        assert_eq!(image.borrow().get_pixel(1, 1), &Rgba([255, 255, 255, 255]));
    }

    // Test getting pixel values
    #[test]
    fn test_get_pixel() {
        let image = create_image(10, 10, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image = MaskedImage::from_image(image.clone()).mask(mask);

        // Get a pixel inside the mask
        assert_eq!(
            masked_image.borrow().get_pixel(3, 3),
            Rgba([255, 255, 255, 255])
        );

        // Get a pixel outside the mask, should return transparent
        assert_eq!(masked_image.borrow().get_pixel(1, 1), Rgba([0, 0, 0, 0]));
    }

    // Test dimensions
    #[test]
    fn test_dimensions() {
        let image = create_image(8, 6, Rgba([0, 0, 255, 255]));
        let mask = Rect::at(1, 1).of_size(3, 3);
        let masked_image = MaskedImage::from_image(image.clone()).mask(mask);

        assert_eq!(masked_image.borrow().dimensions(), (8, 6));
    }
}
