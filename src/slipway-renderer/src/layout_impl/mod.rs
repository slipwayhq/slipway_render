use crate::host_config::SpacingsConfig;

mod adaptive_card;

trait ValidSpacing {
    fn default(&self) -> u32;
}

impl ValidSpacing for Option<SpacingsConfig> {
    fn default(&self) -> u32 {
        let result = self.as_ref().map_or(0, |s| s.default);

        if result < 0 {
            0
        } else {
            result as u32
        }
    }
}
