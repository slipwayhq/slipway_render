use std::{cell::RefCell, rc::Rc};

use image::RgbaImage;
use imageproc::rect::Rect;

use crate::{
    errors::RenderError,
    host_config::generated::HostConfig,
    layoutable::{LayoutContext, LayoutPath, Layoutable},
    masked_image::{Ejectable, MaskedImage},
    size::Size,
    AdaptiveCard,
};

pub fn render(
    host_config: &HostConfig,
    target: &str,
    width: u32,
    height: u32,
) -> Result<RgbaImage, RenderError> {
    let size = Size::new(width, height);
    let target = serde_json::from_str::<AdaptiveCard>(target).unwrap();
    let context = LayoutContext {
        host_config,
        path: Rc::new(LayoutPath {
            current: "root".to_string(),
            previous: None,
        }),
    };
    let image = Rc::new(RefCell::new(RgbaImage::new(size.width(), size.height())));
    target.measure(&context, size)?;
    target.arrange(
        &context,
        Rect::at(0, 0).of_size(size.width(), size.height()),
    )?;

    let masked_image = MaskedImage::from_image(image);
    target.draw(&context, masked_image.clone())?;

    let image = masked_image.eject()?;

    Ok(image)
}
