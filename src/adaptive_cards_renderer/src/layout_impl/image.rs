use std::{cell::RefCell, rc::Rc};

use adaptive_cards::{BlockElementHeight, ImageSize, ImageStyle, StringOrBlockElementHeight};
use image::{imageops::FilterType, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, Canvas},
    point::Point,
    rect::Rect,
};
use taffy::{Dimension, Style, TaffyTree};

use crate::{
    element_layout_data::ElementTaffyData,
    errors::{RenderError, TaffyErrorToRenderError},
    host_config_utils::StringToColor,
    layout_context::LayoutContext,
    layout_scratch::LayoutScratch,
    masked_image::MaskedImage,
    utils::{ClampToI32, ClampToU32, TaffyLayoutUtils},
    ElementLayoutData,
};

use super::{
    image_node_context::ImageNodeContext,
    measure::NodeContext,
    utils::{apply_horizontal_alignment, parse_dimension},
};

enum ImageSource {
    Callout,
    Url,
}

impl ImageSource {
    fn from_image(image: &adaptive_cards::Image<ElementLayoutData>) -> Option<Self> {
        if image.callout.is_some() {
            Some(Self::Callout)
        } else if image.url.is_some() {
            Some(Self::Url)
        } else {
            None
        }
    }
}

impl crate::layoutable::Layoutable for adaptive_cards::Image<ElementLayoutData> {
    fn layout_override(
        &self,
        context: &LayoutContext,
        baseline_style: Style,
        tree: &mut TaffyTree<NodeContext>,
        _scratch: &mut LayoutScratch,
    ) -> Result<ElementTaffyData, RenderError> {
        // Handled by parent
        // self.is_visible
        // self.separator
        // self.spacing
        // self.height

        let mut style = Style { ..baseline_style };
        apply_horizontal_alignment(self.horizontal_alignment, &mut style, context);

        let size = self.size.unwrap_or(ImageSize::Auto);

        if let Some(width) = self.width.as_ref() {
            style.size.width = parse_dimension(width, context)?;
        } else {
            match size {
                adaptive_cards::ImageSize::Auto => {
                    // Image will scale down to fit if needed, but will not scale up to fill the area.
                    style.max_size.width = Dimension::Percent(100.);
                    style.max_size.height = Dimension::Percent(100.);
                }
                adaptive_cards::ImageSize::Stretch => {
                    // Image with both scale down and up to fit as needed.
                    style.max_size.width = Dimension::Percent(100.);
                    style.max_size.height = Dimension::Percent(100.);
                }
                adaptive_cards::ImageSize::Small => {
                    style.size.width =
                        Dimension::Length(context.host_config.image_sizes.small as f32);
                }
                adaptive_cards::ImageSize::Medium => {
                    style.size.width =
                        Dimension::Length(context.host_config.image_sizes.medium as f32);
                }
                adaptive_cards::ImageSize::Large => {
                    style.size.width =
                        Dimension::Length(context.host_config.image_sizes.large as f32);
                }
            }
        }

        let image_source = ImageSource::from_image(self);
        match image_source {
            None => tree
                .new_leaf(style)
                .err_context(context)
                .map(ElementTaffyData::from),
            Some(ImageSource::Callout) => {
                if let StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto) =
                    self.height
                {
                    Err(RenderError::Other {
                        path: context.path.clone(),
                        message: [
                            "Image height is set to Auto, but a callout is present. ",
                            "This will result in a zero height image.",
                        ]
                        .concat(),
                    })
                } else {
                    tree.new_leaf(style)
                        .err_context(context)
                        .map(ElementTaffyData::from)
                }
            }
            Some(ImageSource::Url) => {
                let maybe_source_image = load_source_image(self, context, None)?;
                let Some(source_image) = maybe_source_image else {
                    panic!("URL should always return source image");
                };

                let source_width = source_image.width() as f32;
                let source_height = source_image.height() as f32;

                let max_width = if self.width.as_ref().is_some() {
                    None
                } else {
                    match size {
                        adaptive_cards::ImageSize::Auto => Some(source_width),
                        adaptive_cards::ImageSize::Stretch => None,
                        adaptive_cards::ImageSize::Small => {
                            Some(context.host_config.image_sizes.small as f32)
                        }
                        adaptive_cards::ImageSize::Medium => {
                            Some(context.host_config.image_sizes.medium as f32)
                        }
                        adaptive_cards::ImageSize::Large => {
                            Some(context.host_config.image_sizes.large as f32)
                        }
                    }
                };

                let image_context = ImageNodeContext {
                    source_width,
                    source_height,
                    max_width,
                };

                let node_context = NodeContext::Image(image_context);
                tree.new_leaf_with_context(style, node_context)
                    .err_context(context)
                    .map(ElementTaffyData::from)
            }
        }
    }

    fn draw_override(
        &self,
        context: &LayoutContext,
        tree: &TaffyTree<NodeContext>,
        taffy_data: &ElementTaffyData,
        image: Rc<RefCell<MaskedImage>>,
        _scratch: &mut LayoutScratch,
    ) -> Result<(), RenderError> {
        let node_layout = tree.layout(taffy_data.node_id).err_context(context)?;
        let absolute_rect = node_layout.absolute_rect(context);

        let size = self.size.unwrap_or(ImageSize::Auto);

        let maybe_source_image = load_source_image(self, context, Some(absolute_rect))?;

        let Some(mut source_image) = maybe_source_image else {
            return Ok(());
        };

        let (source_image_width, source_image_height) = source_image.dimensions();
        match size {
            ImageSize::Auto => {
                // Image will scale down to fit if needed, but will not scale up to fill the area.
                if source_image_width > absolute_rect.width()
                    || source_image_height > absolute_rect.height()
                {
                    source_image = scale_to_fit_rect(
                        &source_image,
                        absolute_rect.width(),
                        absolute_rect.height(),
                    );
                }
            }
            ImageSize::Stretch => {
                // Image with both scale down and up to fit as needed.
                if source_image_width != absolute_rect.width()
                    && source_image_height != absolute_rect.height()
                {
                    source_image = scale_to_fit_rect(
                        &source_image,
                        absolute_rect.width(),
                        absolute_rect.height(),
                    );
                }
            }
            ImageSize::Small => {
                let target_width = context.host_config.image_sizes.small.clamp_to_u32();
                scale_to_fit_width(
                    source_image_width,
                    target_width,
                    source_image_height,
                    &mut source_image,
                );
            }
            ImageSize::Medium => {
                let target_width = context.host_config.image_sizes.medium.clamp_to_u32();
                scale_to_fit_width(
                    source_image_width,
                    target_width,
                    source_image_height,
                    &mut source_image,
                );
            }
            ImageSize::Large => {
                let target_width = context.host_config.image_sizes.large.clamp_to_u32();
                scale_to_fit_width(
                    source_image_width,
                    target_width,
                    source_image_height,
                    &mut source_image,
                );
            }
        }

        let horizontal_alignment = self
            .horizontal_alignment
            .unwrap_or(context.inherited.horizontal_alignment);

        let final_position =
            get_new_absolute_position(absolute_rect, &source_image, horizontal_alignment);
        let final_rect = Rect::at(final_position.x, final_position.y)
            .of_size(source_image.width(), source_image.height());

        let style = self.style.unwrap_or(ImageStyle::Default);
        match style {
            ImageStyle::Default => {
                let mut image_mut = image.borrow_mut();
                if let Some(background_color) = self.background_color.as_ref() {
                    let background_color = background_color.to_color()?;
                    draw_filled_rect_mut(&mut *image_mut, final_rect, background_color);
                }
                copy_image(&source_image, &mut image_mut, final_position);
            }
            ImageStyle::Person => {
                // If `style` is set to `person`, apply an ellipse clipping to the image.
                // Assuming the image is square, it'll result in a circle.
                // But if the source image isn't square, it'll result in an ellipse, which is fine.
                let mut image_mut = image.borrow_mut();
                copy_image_within_ellipse(&source_image, &mut image_mut, final_position);
            }
        }

        Ok(())
    }
}

fn load_source_image(
    image: &adaptive_cards::Image<ElementLayoutData>,
    context: &LayoutContext<'_, '_, '_>,
    absolute_rect: Option<Rect>,
) -> Result<Option<RgbaImage>, RenderError> {
    let source_image: Option<RgbaImage> = if let Some(callout) = image.callout.as_ref() {
        let Some(absolute_rect) = absolute_rect else {
            panic!("absolute_rect is required when callout is present");
        };

        let mut input = callout.input.clone();
        input["width"] = serde_json::Value::Number(serde_json::Number::from(absolute_rect.width()));
        input["height"] =
            serde_json::Value::Number(serde_json::Number::from(absolute_rect.height()));

        Some(
            context
                .host_context
                .run_callout(&callout.handle, &input)
                .map_err(|e| RenderError::Other {
                    path: context.path.clone(),
                    message: e.message,
                })?,
        )
    } else if let Some(url) = image.url.as_ref() {
        Some(
            context
                .host_context
                .load_image_from_url(url)
                .map_err(|e| RenderError::Other {
                    path: context.path.clone(),
                    message: e.message,
                })?,
        )
    } else {
        None
    };
    Ok(source_image)
}

fn copy_image(source: &RgbaImage, target: &mut MaskedImage, position: Point<i32>) {
    let x = position.x;
    let y = position.y;
    for sy in 0..source.height() {
        for sx in 0..source.width() {
            let px = source.get_pixel(sx, sy);
            let tx = x + sx.clamp_to_i32();
            let ty = y + sy.clamp_to_i32();
            let Ok(tx) = u32::try_from(tx) else {
                continue;
            };
            let Ok(ty) = u32::try_from(ty) else {
                continue;
            };
            target.draw_pixel(tx, ty, *px);
        }
    }
}

fn copy_image_within_ellipse(source: &RgbaImage, target: &mut MaskedImage, position: Point<i32>) {
    let x = position.x;
    let y = position.y;
    let sw = source.width() as f32;
    let sh = source.height() as f32;
    let cx = sw / 2.0;
    let cy = sh / 2.0;
    let rx = cx;
    let ry = cy;

    for sy in 0..source.height() {
        for sx in 0..source.width() {
            let dx = (sx as f32 - cx) / rx;
            let dy = (sy as f32 - cy) / ry;
            let dist = dx * dx + dy * dy;

            // Dist == 1.0 is on the ellipse boundary, < 1.0 is inside.
            if dist <= 1.0 {
                let px = source.get_pixel(sx, sy);
                let alpha = if dist > 0.9 {
                    // Blend near the edge for a smooth fade.
                    let blend_factor = 1.0 - ((dist - 0.9) / 0.1);
                    (px[3] as f32 * blend_factor) as u8
                } else {
                    px[3]
                };

                let tx = x + sx.clamp_to_i32();
                let ty = y + sy.clamp_to_i32();

                let Ok(tx) = u32::try_from(tx) else {
                    continue;
                };
                let Ok(ty) = u32::try_from(ty) else {
                    continue;
                };

                if tx < target.width() && ty < target.height() {
                    let mut tgt_px = target.get_pixel(tx, ty);
                    let inv_alpha = 255 - alpha;

                    // Simple alpha blending
                    for i in 0..3 {
                        let src_val = px[i] as u16;
                        let tgt_val = tgt_px[i] as u16;
                        let blended = (src_val * alpha as u16 + tgt_val * inv_alpha as u16) / 255;
                        tgt_px[i] = blended as u8;
                    }
                    tgt_px[3] = 255; // Fully opaque result
                    target.draw_pixel(tx, ty, tgt_px);
                }
            }
        }
    }
}

fn get_new_absolute_position(
    absolute_rect: Rect,
    source_image: &RgbaImage,
    horizontal_alignment: adaptive_cards::HorizontalAlignment,
) -> Point<i32> {
    match horizontal_alignment {
        adaptive_cards::HorizontalAlignment::Center => {
            let left = absolute_rect.left()
                + (absolute_rect.width().clamp_to_i32() - source_image.width().clamp_to_i32()) / 2;
            let top = absolute_rect.top()
                + (absolute_rect.height().clamp_to_i32() - source_image.height().clamp_to_i32())
                    / 2;
            Point::new(left, top)
        }
        adaptive_cards::HorizontalAlignment::Left => {
            Point::new(absolute_rect.left(), absolute_rect.top())
        }
        adaptive_cards::HorizontalAlignment::Right => {
            let left = absolute_rect.right() - source_image.width().clamp_to_i32();
            let top = absolute_rect.top();
            Point::new(left, top)
        }
    }
}

fn scale_to_fit_width(
    source_image_width: u32,
    target_width: u32,
    source_image_height: u32,
    source_image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
) {
    if source_image_width != target_width {
        let target_height =
            (source_image_height as f32 * (target_width as f32 / source_image_width as f32)) as u32;
        *source_image = scale_to_fit_rect(&*source_image, target_width, target_height);
    }
}

fn scale_to_fit_rect(img: &RgbaImage, max_width: u32, max_height: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let scale = (max_width as f32 / width as f32).min(max_height as f32 / height as f32);
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    image::imageops::resize(img, new_width, new_height, FilterType::CatmullRom)
}
