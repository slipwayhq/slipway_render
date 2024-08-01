use crate::{element::LayoutableElement, host_config::SpacingsConfig, Spacing};

mod adaptive_card;

trait ValidSpacing {
    fn from(&self, element: &dyn LayoutableElement) -> u32;
    fn from_spacing(&self, spacing: Spacing) -> u32;
}

impl ValidSpacing for Option<SpacingsConfig> {
    fn from(&self, element: &dyn LayoutableElement) -> u32 {
        self.from_spacing(element.get_spacing())
    }
    fn from_spacing(&self, spacing: Spacing) -> u32 {
        match spacing {
            Spacing::None => 0,
            Spacing::Small => valid_spacing(self.as_ref().map_or(0, |s| s.small)),
            Spacing::Medium => valid_spacing(self.as_ref().map_or(0, |s| s.medium)),
            Spacing::Large => valid_spacing(self.as_ref().map_or(0, |s| s.large)),
            Spacing::ExtraLarge => valid_spacing(self.as_ref().map_or(0, |s| s.extra_large)),
            Spacing::Padding => valid_spacing(self.as_ref().map_or(0, |s| s.padding)),
            Spacing::Default => valid_spacing(self.as_ref().map_or(0, |s| s.default)),
        }
    }
}

fn valid_spacing(spacing: i64) -> u32 {
    if spacing < 0 {
        0
    } else {
        spacing as u32
    }
}
