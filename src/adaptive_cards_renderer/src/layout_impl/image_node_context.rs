use taffy::{AvailableSpace, Size};

#[derive(Debug)]
pub struct ImageNodeContext {
    pub source_width: f32,
    pub source_height: f32,
    pub max_width: Option<f32>,
}

impl ImageNodeContext {
    // https://github.com/DioxusLabs/taffy/blob/b5a5f80013a83a27c3be778ea4cf37db5bf40764/examples/common/image.rs
    pub(crate) fn measure(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32> {
        match self.max_width {
            Some(max_width) => self.measure_auto(max_width, known_dimensions),
            None => self.measure_stretch(known_dimensions, available_space),
        }
    }

    fn measure_auto(&self, max_width: f32, known_dimensions: Size<Option<f32>>) -> Size<f32> {
        match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => {
                let known_ratio = width / height;
                let source_ratio = self.source_width / self.source_height;
                if known_ratio > source_ratio {
                    // The known ratio is wider than the source ratio, so the height is the limiting factor.
                    self.measure_auto(
                        max_width,
                        Size {
                            width: None,
                            height: Some(height),
                        },
                    )
                } else {
                    // The known ratio is taller than the source ratio, so the width is the limiting factor.
                    self.measure_auto(
                        max_width,
                        Size {
                            width: Some(width),
                            height: None,
                        },
                    )
                }
            }
            (Some(width), None) => {
                let width = width.min(max_width);
                Size {
                    width,
                    height: (width / self.source_width) * self.source_height,
                }
            }
            (None, Some(height)) => {
                // Calculate the width from the known height, but constrain it to the max width.
                let width = max_width.min((height / self.source_height) * self.source_width);
                Size {
                    width,
                    height: (width / self.source_width) * self.source_height,
                }
            }
            (None, None) => {
                let width = max_width;
                Size {
                    width,
                    height: (width / self.source_width) * self.source_height,
                }
            }
        }
    }

    fn measure_stretch(
        &self,
        known_dimensions: Size<Option<f32>>,
        available_space: Size<AvailableSpace>,
    ) -> Size<f32> {
        match (known_dimensions.width, known_dimensions.height) {
            (Some(width), Some(height)) => {
                let known_ratio = width / height;
                let source_ratio = self.source_width / self.source_height;
                if known_ratio > source_ratio {
                    // The known ratio is wider than the source ratio, so the height is the limiting factor.
                    self.measure_stretch(
                        Size {
                            width: None,
                            height: Some(height),
                        },
                        available_space,
                    )
                } else {
                    // The known ratio is taller than the source ratio, so the width is the limiting factor.
                    self.measure_stretch(
                        Size {
                            width: Some(width),
                            height: None,
                        },
                        available_space,
                    )
                }
            }
            (Some(width), None) => Size {
                width,
                height: (width / self.source_width) * self.source_height,
            },
            (None, Some(height)) => Size {
                width: (height / self.source_height) * self.source_width,
                height,
            },
            (None, None) => {
                if let AvailableSpace::Definite(width) = available_space.width {
                    if let AvailableSpace::Definite(height) = available_space.height {
                        let known_ratio = width / height;
                        let source_ratio = self.source_width / self.source_height;
                        if known_ratio > source_ratio {
                            // The known ratio is wider than the source ratio, so the height is the limiting factor.
                            self.measure_stretch(
                                Size {
                                    width: None,
                                    height: Some(height),
                                },
                                available_space,
                            )
                        } else {
                            // The known ratio is taller than the source ratio, so the width is the limiting factor.
                            self.measure_stretch(
                                Size {
                                    width: Some(width),
                                    height: None,
                                },
                                available_space,
                            )
                        }
                    } else {
                        Size {
                            width,
                            height: (width / self.source_width) * self.source_height,
                        }
                    }
                } else if let AvailableSpace::Definite(height) = available_space.height {
                    Size {
                        width: (height / self.source_height) * self.source_width,
                        height,
                    }
                } else {
                    Size {
                        width: self.source_width,
                        height: self.source_height,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use taffy::{AvailableSpace, Size};

    // Helper to create a default context with source dimensions
    fn context(source_width: f32, source_height: f32, max_width: Option<f32>) -> ImageNodeContext {
        ImageNodeContext {
            source_width,
            source_height,
            max_width,
        }
    }

    mod auto_tests {
        use super::*;
        #[test]
        fn measure_auto_both_known_dimensions_wider_ratio() {
            let ctx = context(100.0, 50.0, Some(500.0));
            let size = ctx.measure(
                Size {
                    width: Some(300.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            // Height is limiting factor
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_auto_both_known_dimensions_taller_ratio() {
            let ctx = context(100.0, 50.0, Some(500.0));
            let size = ctx.measure(
                Size {
                    width: Some(150.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            // Width is limiting factor
            assert_eq!(size.width, 150.0);
            assert_eq!(size.height, 75.0);
        }

        #[test]
        fn measure_auto_both_known_dimensions_wider_ratio_max_width_constrained() {
            let ctx = context(100.0, 50.0, Some(50.0));
            let size = ctx.measure(
                Size {
                    width: Some(300.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            // Max width is limiting factor
            assert_eq!(size.width, 50.0);
            assert_eq!(size.height, 25.0);
        }

        #[test]
        fn measure_auto_both_known_dimensions_taller_ratio_max_width_constrained() {
            let ctx = context(100.0, 50.0, Some(50.0));
            let size = ctx.measure(
                Size {
                    width: Some(150.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            // Max width is limiting factor
            assert_eq!(size.width, 50.0);
            assert_eq!(size.height, 25.0);
        }

        #[test]
        fn measure_auto_known_width_only() {
            let ctx = context(100.0, 50.0, Some(200.0));
            let size = ctx.measure(
                Size {
                    width: Some(80.0),
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 80.0);
            assert_eq!(size.height, 40.0);
        }

        #[test]
        fn measure_auto_known_width_only_large_width() {
            let ctx = context(100.0, 50.0, Some(200.0));
            let size = ctx.measure(
                Size {
                    width: Some(400.0),
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_auto_known_height_only() {
            let ctx = context(100.0, 50.0, Some(200.0));
            let size = ctx.measure(
                Size {
                    width: None,
                    height: Some(40.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 80.0);
            assert_eq!(size.height, 40.0);
        }

        #[test]
        fn measure_auto_known_height_only_large_height() {
            let ctx = context(100.0, 50.0, Some(200.0));
            let size = ctx.measure(
                Size {
                    width: None,
                    height: Some(300.0),
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_auto_none_known_large_max_width() {
            let ctx = context(100.0, 50.0, Some(200.0));
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_auto_none_known_small_max_width() {
            let ctx = context(100.0, 50.0, Some(80.0));
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MaxContent,
                },
            );
            assert_eq!(size.width, 80.0);
            assert_eq!(size.height, 40.0);
        }
    }

    mod stretch_tests {
        use super::*;

        #[test]
        fn measure_stretch_both_known_dimensions_wider_ratio() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: Some(300.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            // Height is limiting factor
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_stretch_both_known_dimensions_taller_ratio() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: Some(150.0),
                    height: Some(100.0),
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            // Width is limiting factor
            assert_eq!(size.width, 150.0);
            assert_eq!(size.height, 75.0);
        }

        #[test]
        fn measure_stretch_known_width_only() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: Some(80.0),
                    height: None,
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            assert_eq!(size.width, 80.0);
            assert_eq!(size.height, 40.0);
        }

        #[test]
        fn measure_stretch_known_width_only_large_width() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: Some(400.0),
                    height: None,
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            assert_eq!(size.width, 400.0);
            assert_eq!(size.height, 200.0);
        }

        #[test]
        fn measure_stretch_known_height_only() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: Some(40.0),
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            assert_eq!(size.width, 80.0);
            assert_eq!(size.height, 40.0);
        }

        #[test]
        fn measure_stretch_known_height_only_large_height() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: Some(200.0),
                },
                Size {
                    width: AvailableSpace::Definite(500.0),
                    height: AvailableSpace::Definite(500.0),
                },
            );
            assert_eq!(size.width, 400.0);
            assert_eq!(size.height, 200.0);
        }

        #[test]
        fn measure_stretch_neither_known_both_definite_wider_ratio() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::Definite(300.0),
                    height: AvailableSpace::Definite(100.0),
                },
            );
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_stretch_neither_known_both_definite_taller_ratio() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::Definite(150.0),
                    height: AvailableSpace::Definite(100.0),
                },
            );
            assert_eq!(size.width, 150.0);
            assert_eq!(size.height, 75.0);
        }

        #[test]
        fn measure_stretch_neither_known_width_only_definite() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::Definite(150.0),
                    height: AvailableSpace::MaxContent, // indefinite
                },
            );
            assert_eq!(size.width, 150.0);
            assert_eq!(size.height, 75.0);
        }

        #[test]
        fn measure_stretch_neither_known_height_only_definite() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::MinContent, // indefinite
                    height: AvailableSpace::Definite(100.0),
                },
            );
            assert_eq!(size.width, 200.0);
            assert_eq!(size.height, 100.0);
        }

        #[test]
        fn measure_stretch_neither_known_both_indefinite() {
            let ctx = context(100.0, 50.0, None);
            let size = ctx.measure(
                Size {
                    width: None,
                    height: None,
                },
                Size {
                    width: AvailableSpace::MaxContent,
                    height: AvailableSpace::MinContent,
                },
            );
            assert_eq!(size.width, 100.0);
            assert_eq!(size.height, 50.0);
        }
    }
}
