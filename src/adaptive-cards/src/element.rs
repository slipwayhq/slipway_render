use crate::{Spacing, StringOrBlockElementHeight};

/// A trait for getting the properties common to all layoutable Adaptive Cards elements.
/// These are the properties which appear in the `Element` and `ToggleableItem` (which
/// `Element` extends) schema items from the Adaptive Cards typed schema.
/// This trait is implemented for all Adaptive Cards elements automatically by the
/// Adaptive Cards types generator.
pub trait LayoutableElement {
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
