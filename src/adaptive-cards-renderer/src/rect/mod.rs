use imageproc::rect::Rect;

mod contains;

// A copy of the imageproc Region trait so we can add contains for u32.
pub(super) trait SlipwayRegion<T> {
    /// Whether this region contains the given point.
    fn contains(&self, x: T, y: T) -> bool;
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinalRect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl From<Rect> for FinalRect {
    fn from(rect: Rect) -> Self {
        FinalRect {
            x: rect.left(),
            y: rect.top(),
            width: rect.width(),
            height: rect.height(),
        }
    }
}
