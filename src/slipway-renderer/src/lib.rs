mod adaptive_cards_types;
mod errors;
mod layout_impl;
mod layoutable;

use adaptive_cards_types::generated::*;

#[derive(PartialEq, Copy, Clone, Default, Debug)]
struct Size {
    width: f32,
    height: f32,
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
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    fn is_invalid(&self) -> bool {
        self.width.is_nan() || self.height.is_nan()
    }
}

#[derive(PartialEq, Copy, Clone, Default, Debug)]
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn is_invalid(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.width.is_nan() || self.height.is_nan()
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
