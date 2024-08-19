use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::RenderError,
    layoutable::{DebugMode, LayoutContext},
    rect::SlipwayRegion,
    utils::extract_from_rc_refcell,
};
use image::{Pixel, Rgba, RgbaImage};
use imageproc::{drawing::Canvas, rect::Rect};

enum MaskOrImage {
    Mask(Rc<RefCell<MaskedImage>>),
    Image {
        image: Rc<RefCell<RgbaImage>>,
        debug_mode: DebugMode,
    },
}

pub(super) struct MaskedImage {
    image: MaskOrImage,
    mask: Rect,
}

impl MaskedImage {
    pub fn from_image_context(
        image: Rc<RefCell<RgbaImage>>,
        context: &LayoutContext,
    ) -> Rc<RefCell<Self>> {
        Self::from_image(image, context.debug_mode)
    }

    pub fn from_image(image: Rc<RefCell<RgbaImage>>, debug_mode: DebugMode) -> Rc<RefCell<Self>> {
        let (width, height) = image.borrow().dimensions();
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Image {
                image: image.clone(),
                debug_mode,
            },
            mask: Rect::at(0, 0).of_size(width, height),
        }))
    }

    pub fn from_mask(image: Rc<RefCell<MaskedImage>>, mask: Rect) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            image: MaskOrImage::Mask(image.clone()),
            mask,
        }))
    }

    pub fn eject(self) -> Result<RgbaImage, RenderError> {
        match self.image {
            MaskOrImage::Mask(mask) => extract_from_rc_refcell(mask)
                .ok_or(RenderError::ImageReferenceCountNotOne)?
                .eject(),
            MaskOrImage::Image { image, .. } => {
                extract_from_rc_refcell(image).ok_or(RenderError::ImageReferenceCountNotOne)
            }
        }
    }

    fn debug_mode(&self) -> DebugMode {
        match &self.image {
            MaskOrImage::Image { debug_mode, .. } => *debug_mode,
            MaskOrImage::Mask(mask) => mask.borrow().debug_mode(),
        }
    }
}

trait Maskable {
    fn mask(self, mask: Rect) -> Rc<RefCell<MaskedImage>>;
}

impl Maskable for Rc<RefCell<MaskedImage>> {
    fn mask(self, mask: Rect) -> Rc<RefCell<MaskedImage>> {
        MaskedImage::from_mask(self, mask)
    }
}

pub(super) trait Ejectable<T> {
    fn eject(self) -> Result<T, RenderError>;
}

impl Ejectable<RgbaImage> for Rc<RefCell<MaskedImage>> {
    fn eject(self) -> Result<RgbaImage, RenderError> {
        extract_from_rc_refcell(self)
            .ok_or(RenderError::ImageReferenceCountNotOne)?
            .eject()
    }
}

// Implement the Canvas trait for MaskedImage
impl Canvas for MaskedImage {
    type Pixel = Rgba<u8>;

    fn draw_pixel(&mut self, x: u32, y: u32, pixel: Self::Pixel) {
        let final_pixel = if self.mask.contains(x, y) {
            Some(pixel)
        } else if self.debug_mode().transparent_masks {
            Some(Rgba([200, 200, 200, 255]))
        } else {
            None
        };

        if let Some(final_pixel) = final_pixel {
            match &mut self.image {
                MaskOrImage::Mask(mask) => mask.borrow_mut().draw_pixel(x, y, final_pixel),
                MaskOrImage::Image { image, .. } => {
                    image.borrow_mut().blend_pixel(x, y, final_pixel)
                }
            }
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        match &self.image {
            MaskOrImage::Mask(mask) => mask.borrow().get_pixel(x, y),
            MaskOrImage::Image { image, .. } => *image.borrow().get_pixel(x, y),
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        match &self.image {
            MaskOrImage::Mask(mask) => mask.borrow().dimensions(),
            MaskOrImage::Image { image, .. } => image.borrow().dimensions(),
        }
    }
}

trait BlendingPutPixel {
    fn blend_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>);
}

impl BlendingPutPixel for RgbaImage {
    fn blend_pixel(&mut self, x: u32, y: u32, pixel: Rgba<u8>) {
        let current_pixel = self.get_pixel_mut(x, y);
        current_pixel.blend(&pixel);
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

    fn create_image(width: u32, height: u32, color: Rgba<u8>) -> Rc<RefCell<RgbaImage>> {
        let image = ImageBuffer::from_fn(width, height, |_, _| color);
        Rc::new(RefCell::new(image))
    }

    #[test]
    fn create_from_image() {
        let width = 10;
        let height = 10;
        let image = create_image(width, height, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);

        let masked_image = MaskedImage::from_image(image.clone(), DebugMode::none()).mask(mask);

        // Check initial dimensions
        assert_eq!(masked_image.borrow().dimensions(), (width, height));

        // Ensure the mask is set correctly
        let m = masked_image.borrow();
        assert_eq!(m.mask.left(), 2);
        assert_eq!(m.mask.top(), 2);
        assert_eq!(m.mask.width(), 5);
        assert_eq!(m.mask.height(), 5);
    }

    #[test]
    fn create_from_mask() {
        let base_image = create_image(10, 10, Rgba([255, 0, 0, 255]));
        let base_mask = Rect::at(0, 0).of_size(10, 10);
        let base_masked_image =
            MaskedImage::from_image(base_image, DebugMode::none()).mask(base_mask);

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

    #[test]
    fn draw_pixel() {
        let image = create_image(10, 10, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image = MaskedImage::from_image(image.clone(), DebugMode::none()).mask(mask);

        // Draw a pixel inside the mask
        masked_image
            .borrow_mut()
            .draw_pixel(3, 3, Rgba([255, 0, 0, 255]));
        assert_eq!(image.borrow().get_pixel(3, 3), &Rgba([255, 0, 0, 255]));

        // Draw a pixel outside the mask, should not change
        masked_image
            .borrow_mut()
            .draw_pixel(1, 1, Rgba([0, 255, 0, 255]));
        assert_eq!(image.borrow().get_pixel(1, 1), &Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn draw_pixel_with_transparent_masks() {
        let image = create_image(10, 10, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image =
            MaskedImage::from_image(image.clone(), DebugMode::with_transparent_masks()).mask(mask);

        // Draw a pixel inside the mask
        masked_image
            .borrow_mut()
            .draw_pixel(3, 3, Rgba([255, 0, 0, 255]));
        assert_eq!(image.borrow().get_pixel(3, 3), &Rgba([255, 0, 0, 255]));

        // Draw a pixel outside the mask with DebugMode::TransparentMasks should change.
        masked_image
            .borrow_mut()
            .draw_pixel(1, 1, Rgba([0, 255, 0, 255]));

        // It should equal neither the original color nor the color we set.
        assert_ne!(image.borrow().get_pixel(1, 1), &Rgba([0, 255, 0, 255]));
        assert_ne!(image.borrow().get_pixel(1, 1), &Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn get_pixel() {
        let image = create_image(10, 10, Rgba([255, 255, 255, 255]));
        let mask = Rect::at(2, 2).of_size(5, 5);
        let masked_image = MaskedImage::from_image(image.clone(), DebugMode::none()).mask(mask);

        // Get a pixel inside the mask
        assert_eq!(
            masked_image.borrow().get_pixel(3, 3),
            Rgba([255, 255, 255, 255])
        );

        // Get a pixel outside the mask, should still return pixel
        assert_eq!(
            masked_image.borrow().get_pixel(1, 1),
            Rgba([255, 255, 255, 255])
        );
    }

    #[test]
    fn dimensions() {
        let image = create_image(8, 6, Rgba([0, 0, 255, 255]));
        let mask = Rect::at(1, 1).of_size(3, 3);
        let masked_image = MaskedImage::from_image(image.clone(), DebugMode::none()).mask(mask);

        assert_eq!(masked_image.borrow().dimensions(), (8, 6));
    }
}
