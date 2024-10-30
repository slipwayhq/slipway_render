use image::Rgba;

/// A struct that can be used to enable various debug features in the renderer
/// which can help when debugging rendering issues.
#[derive(Default, Copy, Clone, Debug)]
pub struct DebugMode {
    /// When this is enabled masks do not fully block pixels from being drawn, but
    /// instead the pixels are drawn faintly.
    pub transparent_masks: bool,

    /// When this is enabled outlines are drawn around elements to help visualize the layout.
    pub outlines: bool,
}

impl DebugMode {
    pub fn none() -> Self {
        DebugMode {
            transparent_masks: false,
            outlines: false,
        }
    }

    pub fn with_transparent_masks() -> Self {
        DebugMode {
            transparent_masks: true,
            outlines: false,
        }
    }

    pub fn with_outlines() -> Self {
        DebugMode {
            transparent_masks: false,
            outlines: true,
        }
    }

    pub fn full() -> Self {
        DebugMode {
            transparent_masks: true,
            outlines: true,
        }
    }
}

/// Return the next color from COLORS using a global counter.
pub fn next_color() -> Rgba<u8> {
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A set of randomly generated colors to use for debug rendering.
    const COLORS: [[u8; 4]; 20] = [
        [204, 143, 148, 255],
        [204, 178, 148, 255],
        [204, 204, 148, 255],
        [148, 204, 160, 255],
        [148, 180, 204, 255],
        [175, 148, 204, 255],
        [204, 161, 169, 255],
        [160, 204, 204, 255],
        [204, 162, 148, 255],
        [163, 204, 163, 255],
        [204, 163, 183, 255],
        [163, 183, 204, 255],
        [148, 204, 204, 255],
        [204, 178, 178, 255],
        [183, 204, 163, 255],
        [204, 163, 163, 255],
        [204, 204, 163, 255],
        [163, 183, 204, 255],
        [204, 184, 163, 255],
        [163, 204, 184, 255],
    ];

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let index = COUNTER.fetch_add(1, Ordering::Relaxed) % COLORS.len();
    Rgba(COLORS[index])
}
