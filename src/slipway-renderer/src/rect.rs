#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub(super) struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect {{ x: {}, y: {}, width: {}, height: {} }}",
            self.x, self.y, self.width, self.height
        )
    }
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn move_inside(&self, outer_rect: Rect) -> Rect {
        let moved_x = self.x + outer_rect.x;
        let x = if moved_x < outer_rect.x {
            outer_rect.x
        } else if moved_x > outer_rect.x + outer_rect.width {
            outer_rect.x + outer_rect.width
        } else {
            moved_x
        };

        let moved_y = self.y + outer_rect.y;
        let y = if moved_y < outer_rect.y {
            outer_rect.y
        } else if moved_y > outer_rect.y + outer_rect.height {
            outer_rect.y + outer_rect.height
        } else {
            moved_y
        };

        let width = self.width.min(outer_rect.x + outer_rect.width - x);
        let height = self.height.min(outer_rect.y + outer_rect.height - y);

        Rect::new(x, y, width, height)
    }

    pub fn overlap(&self, b: Rect) -> Option<Rect> {
        // Calculate the maximum starting x and y coordinates
        let start_x = self.x.max(b.x);
        let start_y = self.y.max(b.y);

        // Calculate the minimum ending x and y coordinates
        let end_x = (self.x + self.width).min(b.x + b.width);
        let end_y = (self.y + self.height).min(b.y + b.height);

        // Calculate the overlapping width and height
        let overlap_width = end_x.checked_sub(start_x)?;
        let overlap_height = end_y.checked_sub(start_y)?;

        // Check if there is any overlap
        if overlap_width > 0 && overlap_height > 0 {
            Some(Rect {
                x: start_x,
                y: start_y,
                width: overlap_width,
                height: overlap_height,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_full_overlap() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 2,
            y: 2,
            width: 5,
            height: 5,
        };
        let expected = Some(Rect {
            x: 2,
            y: 2,
            width: 5,
            height: 5,
        });
        assert_eq!(a.overlap(b), expected);
    }

    #[test]
    fn test_overlap_partial_overlap() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 5,
            y: 5,
            width: 10,
            height: 10,
        };
        let expected = Some(Rect {
            x: 5,
            y: 5,
            width: 5,
            height: 5,
        });
        assert_eq!(a.overlap(b), expected);
    }

    #[test]
    fn test_overlap_no_overlap() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 15,
            y: 15,
            width: 5,
            height: 5,
        };
        assert_eq!(a.overlap(b), None);
    }

    #[test]
    fn test_overlap_touching_edges() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 10,
            y: 10,
            width: 5,
            height: 5,
        };
        assert_eq!(a.overlap(b), None);
    }

    #[test]
    fn test_overlap_identical_rects() {
        let a = Rect {
            x: 3,
            y: 3,
            width: 4,
            height: 4,
        };
        let b = Rect {
            x: 3,
            y: 3,
            width: 4,
            height: 4,
        };
        let expected = Some(Rect {
            x: 3,
            y: 3,
            width: 4,
            height: 4,
        });
        assert_eq!(a.overlap(b), expected);
    }

    #[test]
    fn test_overlap_edge_overlap() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 10,
            y: 5,
            width: 5,
            height: 5,
        };
        assert_eq!(a.overlap(b), None);
    }

    #[test]
    fn test_overlap_corner_touch() {
        let a = Rect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        };
        let b = Rect {
            x: 10,
            y: 10,
            width: 5,
            height: 5,
        };
        assert_eq!(a.overlap(b), None);
    }

    #[test]
    fn test_overlap_large_in_small() {
        let a = Rect {
            x: 5,
            y: 5,
            width: 3,
            height: 3,
        };
        let b = Rect {
            x: 0,
            y: 0,
            width: 15,
            height: 15,
        };
        let expected = Some(Rect {
            x: 5,
            y: 5,
            width: 3,
            height: 3,
        });
        assert_eq!(a.overlap(b), expected);
    }
}
