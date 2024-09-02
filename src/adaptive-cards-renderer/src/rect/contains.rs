use imageproc::rect::Rect;

use super::SlipwayRegion;

impl SlipwayRegion<u32> for Rect {
    fn contains(&self, x: u32, y: u32) -> bool {
        let x = i32::try_from(x).expect("x was too large");
        let y = i32::try_from(y).expect("y was too large");
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
