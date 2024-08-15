use std::{cell::RefCell, rc::Rc};

use image::RgbaImage;
use taffy::{AvailableSpace, Point, TaffyTree};

use crate::{
    errors::{RenderError, TaffyErrorToRenderError},
    host_config::generated::HostConfig,
    layout_impl::{measure_function, NodeContext},
    layoutable::{DebugMode, LayoutContext, LayoutPath, Layoutable},
    masked_image::{Ejectable, MaskedImage},
    AdaptiveCard,
};

pub(super) fn render(
    host_config: &HostConfig,
    target: &str,
    width: u32,
    height: u32,
    debug_mode: DebugMode,
) -> Result<RgbaImage, RenderError> {
    let target = serde_json::from_str::<AdaptiveCard>(target).unwrap();
    let context = LayoutContext {
        host_config,
        debug_mode: DebugMode::TransparentMasks,
        path: Rc::new(LayoutPath {
            current: "root".to_string(),
            previous: None,
        }),
        current_origin: Point { x: 0., y: 0. },
    };

    let mut tree: TaffyTree<NodeContext> = TaffyTree::new();
    let root = target.layout(&context, Default::default(), &mut tree)?;

    tree.compute_layout_with_measure(
        root,
        taffy::Size {
            width: AvailableSpace::Definite(width as f32),
            height: AvailableSpace::Definite(height as f32),
        },
        // Note: this closure is a FnMut closure and can be used to borrow external context for the duration of layout
        // For example, you may wish to borrow a global font registry and pass it into your text measuring function
        |known_dimensions, available_space, _node_id, node_context, _style| {
            measure_function(known_dimensions, available_space, node_context)
        },
    )
    .err_context(&context)?;

    tree.print_tree(root);

    let image = Rc::new(RefCell::new(RgbaImage::new(width, height)));
    let masked_image = MaskedImage::from_image(image, debug_mode);
    target.draw(&context, &tree, masked_image.clone())?;

    let image = masked_image.eject()?;

    Ok(image)
}
