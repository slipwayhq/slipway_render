use imageproc::rect::Rect;

use super::MoveableFromOrigin;

impl MoveableFromOrigin for Rect {
    fn move_from_origin_into(&self, outer_rect: Rect) -> Rect {
        let moved_x = self.left() + outer_rect.left();
        let x = if moved_x < outer_rect.left() {
            outer_rect.left()
        } else if moved_x > outer_rect.right() {
            outer_rect.right()
        } else {
            moved_x
        };

        let moved_y = self.top() + outer_rect.top();
        let y = if moved_y < outer_rect.top() {
            outer_rect.top()
        } else if moved_y > outer_rect.bottom() {
            outer_rect.bottom()
        } else {
            moved_y
        };

        let width = self
            .width()
            .min(u32::try_from(outer_rect.right() - x + 1).unwrap_or(0));
        let height = self
            .height()
            .min(u32::try_from(outer_rect.bottom() - y + 1).unwrap_or(0));

        Rect::at(x, y).of_size(width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fits_within_target_at_origin() {
        let inner = Rect::at(0, 0).of_size(10, 12);
        let outer = Rect::at(0, 0).of_size(20, 18);
        let expected = Rect::at(0, 0).of_size(10, 12);
        assert_eq!(inner.move_from_origin_into(outer), expected);
    }

    #[test]
    fn fits_within_target_not_at_origin() {
        let inner = Rect::at(0, 0).of_size(10, 12);
        let outer = Rect::at(100, 50).of_size(20, 18);
        let expected = Rect::at(100, 50).of_size(10, 12);
        assert_eq!(inner.move_from_origin_into(outer), expected);
    }

    #[test]
    fn too_large() {
        let inner = Rect::at(0, 0).of_size(30, 30);
        let outer = Rect::at(100, 50).of_size(20, 18);
        let expected = Rect::at(100, 50).of_size(20, 18);
        assert_eq!(inner.move_from_origin_into(outer), expected);
    }

    #[test]
    fn inner_not_at_origin() {
        let inner = Rect::at(5, 5).of_size(20, 22);
        let outer = Rect::at(100, 50).of_size(20, 18);
        let expected = Rect::at(105, 55).of_size(15, 13);
        assert_eq!(inner.move_from_origin_into(outer), expected);
    }
}
