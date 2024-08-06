mod adaptive_cards_types;
mod element;
mod errors;
mod host_config;
mod layout_impl;
mod layoutable;
mod masked_image;
mod rect;
pub mod render;
mod size;
use adaptive_cards_types::generated::*;

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
