use imageproc::rect::Rect;

mod contains;
mod move_from_origin_into;

// A copy of the imageproc Region trait so we can add contains for u32.
pub(super) trait SlipwayRegion<T> {
    /// Whether this region contains the given point.
    fn contains(&self, x: T, y: T) -> bool;
}

pub(super) trait MoveableFromOrigin {
    fn move_from_origin_into(&self, outer_rect: Rect) -> Rect;
}
