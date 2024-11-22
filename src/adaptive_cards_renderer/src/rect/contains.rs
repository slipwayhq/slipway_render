use imageproc::rect::Rect;

use super::SlipwayRegion;

impl SlipwayRegion<u32> for Rect {
    fn contains(&self, x: u32, y: u32) -> bool {
        // If we're outside the bounds of an i32 then the point is definitely not in the rect.
        let Ok(x) = i32::try_from(x) else {
            println!("rect x overflow: {}", x);
            return false;
        };

        let Ok(y) = i32::try_from(y) else {
            println!("rect y overflow: {}", y);
            return false;
        };

        self.left() <= x && x <= self.right() && self.top() <= y && y <= self.bottom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        assert!(Rect::at(0, 0).of_size(10, 10).contains(5, 5));
        assert!(Rect::at(10, 10).of_size(10, 10).contains(15, 15));
        assert!(!Rect::at(0, 0).of_size(10, 10).contains(15, 5));
        assert!(!Rect::at(0, 0).of_size(10, 10).contains(5, 15));
        assert!(!Rect::at(0, 0).of_size(10, 10).contains(15, 15));
        assert!(!Rect::at(10, 10).of_size(10, 10).contains(5, 5));
    }
}
