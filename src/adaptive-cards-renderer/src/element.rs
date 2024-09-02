use crate::{Spacing, StringOrBlockElementHeight};

pub(super) trait LayoutableElement {
    fn get_height(&self) -> StringOrBlockElementHeight;
    fn get_separator(&self) -> bool;
    fn get_spacing(&self) -> Spacing;
    fn get_is_visible(&self) -> bool;
}

impl<T: LayoutableElement> LayoutableElement for Box<T> {
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.as_ref().get_height()
    }

    fn get_separator(&self) -> bool {
        self.as_ref().get_separator()
    }

    fn get_spacing(&self) -> Spacing {
        self.as_ref().get_spacing()
    }

    fn get_is_visible(&self) -> bool {
        self.as_ref().get_is_visible()
    }
}
