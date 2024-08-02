#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub(super) struct Size {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Size {{ width: {}, height: {} }}",
            self.width, self.height
        )
    }
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn constrain(&self, max_size: Size) -> Size {
        let width = self.width.min(max_size.width);
        let height = self.height.min(max_size.height);
        Size::new(width, height)
    }
}
