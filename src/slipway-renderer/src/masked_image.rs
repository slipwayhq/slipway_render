use std::{cell::RefCell, rc::Rc};

use image::{Rgba, RgbaImage};
use imageproc::drawing::Canvas;

use crate::rect::Rect;

enum MaskOrImage {
    Mask(Rc<RefCell<MaskedImage>>),
    Image(Rc<RefCell<RgbaImage>>),
}
// Define the MaskedImage struct
pub(super) struct MaskedImage {
    image: MaskOrImage,
    mask: Rect,
}

impl MaskedImage {
    pub fn from_image(image: Rc<RefCell<RgbaImage>>, mask: Rect) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Image(image.clone()),
            mask,
        }))
    }
    pub fn from_mask(image: Rc<RefCell<MaskedImage>>, mask: Rect) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Mask(image.clone()),
            mask,
        }))
    }
}

pub(super) trait SlipwayCanvas {
    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>);
    fn get_pixel(&self, x: u32, y: u32) -> Rgba<u8>;
    fn dimensions(&self) -> (u32, u32);
}

// Implement the Canvas trait for MaskedImage
impl Canvas for MaskedImage {
    type Pixel = Rgba<u8>;

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        // Only modify pixels within the mask
        if x >= self.mask.x
            && x < self.mask.x + self.mask.width
            && y >= self.mask.y
            && y < self.mask.y + self.mask.height
        {
            match &mut self.image {
                MaskOrImage::Mask(mask) => mask.borrow_mut().draw_pixel(x, y, pixel),
                MaskOrImage::Image(image) => image.borrow_mut().put_pixel(x, y, pixel),
            }
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
