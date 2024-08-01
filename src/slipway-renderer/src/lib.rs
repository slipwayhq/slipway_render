mod adaptive_cards_types;
mod errors;
mod host_config;
mod layout_impl;
mod layoutable;

use adaptive_cards_types::generated::*;

type SlipwayImage = image::RgbaImage;

#[derive(PartialEq, Copy, Clone, Default, Debug)]
struct Size {
    width: u32,
    height: u32,
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
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn constrain(&self, max_size: Size) -> Size {
        let width = self.width.min(max_size.width);
        let height = self.height.min(max_size.height);
        Size::new(width, height)
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
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
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn constrain(&self, outer_rect: Rect) -> Rect {
        let x = if self.x < outer_rect.x {
            outer_rect.x
        } else if self.x > outer_rect.x + outer_rect.width {
            outer_rect.x + outer_rect.width
        } else {
            self.x
        };

        let y = if self.y < outer_rect.y {
            outer_rect.y
        } else if self.y > outer_rect.y + outer_rect.height {
            outer_rect.y + outer_rect.height
        } else {
            self.y
        };

        let width = self.width.min(outer_rect.x + outer_rect.width - x);
        let height = self.height.min(outer_rect.y + outer_rect.height - y);

        Rect::new(x, y, width, height)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let json_data = r#"
//         {
//             "type": "AdaptiveCard",
//             "version": "1.0",
//             "body": [
//                 {
//                     "type": "TextBlock",
//                     "text": "Hello, World!"
//                 },
//                 {
//                     "type": "Container",
//                     "items": [
//                         {
//                             "type": "TextBlock",
//                             "text": "Nested Text"
//                         }
//                     ],
//                     "height": "Stretch"
//                 },
//                 {
//                     "type": "Image",
//                     "url": "http://example.com/image.png"
//                 }
//             ]
//         }
//         "#;

//         todo!();
//         // let card = parse_adaptive_card(json_data);
//         // let bitmap = render_card(&card, 800, 600);
//         // bitmap.save("output.png").unwrap();
//     }
// }
