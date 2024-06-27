// use image::{ImageBuffer, Rgba};
// use serde::Deserialize;

// // Data structures for AdaptiveCard and layout
// #[derive(Deserialize)]
// struct AdaptiveCard {
//     #[serde(rename = "type")]
//     card_type: String,
//     version: String,
//     body: Vec<CardElement>,
// }

// #[derive(Deserialize)]
// #[serde(tag = "type")]
// enum CardElement {
//     Container(Container),
//     TextBlock(TextBlock),
//     Image(ImageElement),
//     // Add other elements as needed
// }

// #[derive(Deserialize)]
// struct Container {
//     items: Vec<CardElement>,
//     height: Option<String>, // "Stretch" or specific value
//                             // Add other properties as needed
// }

// #[derive(Deserialize)]
// struct TextBlock {
//     text: String,
//     size: Option<String>,
//     weight: Option<String>,
//     // Add other properties as needed
// }

// #[derive(Deserialize)]
// struct ImageElement {
//     url: String,
//     // Add other properties as needed
// }

// // Structure for storing computed layout
// struct Layout {
//     x: u32,
//     y: u32,
//     width: u32,
//     height: u32,
// }

// // Function to parse the Adaptive Card JSON
// fn parse_adaptive_card(json_data: &str) -> AdaptiveCard {
//     serde_json::from_str(json_data).unwrap()
// }

// // Function to compute the layout of the card
// fn compute_layout(card: &AdaptiveCard, width: u32, height: u32) -> Vec<(CardElement, Layout)> {
//     let mut layouts = Vec::new();
//     let mut y_offset = 0;

//     // First pass: compute fixed heights and count stretchable elements
//     let mut stretchable_count = 0;
//     let mut fixed_height = 0;
//     for element in &card.body {
//         match element {
//             CardElement::Container(container) => {
//                 if container.height.as_deref() == Some("Stretch") {
//                     stretchable_count += 1;
//                 } else {
//                     fixed_height += 100; // Assume a fixed height for simplicity
//                 }
//             }
//             _ => fixed_height += 50, // Assume a fixed height for simplicity
//         }
//     }

//     // Calculate stretchable height
//     let stretchable_height = (height - fixed_height) / stretchable_count.max(1) as u32;

//     // Second pass: assign positions and sizes
//     for element in &card.body {
//         let element_height = match element {
//             CardElement::Container(container) => {
//                 if container.height.as_deref() == Some("Stretch") {
//                     stretchable_height
//                 } else {
//                     100 // Assume a fixed height for simplicity
//                 }
//             }
//             _ => 50, // Assume a fixed height for simplicity
//         };

//         layouts.push((
//             element.clone(),
//             Layout {
//                 x: 0,
//                 y: y_offset,
//                 width,
//                 height: element_height,
//             },
//         ));
//         y_offset += element_height;
//     }

//     layouts
// }

// // Function to render the card using the computed layout
// fn render_card(card: &AdaptiveCard, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//     let mut img = ImageBuffer::new(width, height);
//     let layouts = compute_layout(card, width, height);

//     for (element, layout) in layouts {
//         match element {
//             CardElement::TextBlock(text_block) => render_text_block(&mut img, &text_block, &layout),
//             CardElement::Image(image_element) => render_image(&mut img, &image_element, &layout),
//             CardElement::Container(container) => render_container(&mut img, &container, &layout),
//             // Handle other elements
//         }
//     }

//     img
// }

// fn render_text_block(
//     img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
//     text_block: &TextBlock,
//     layout: &Layout,
// ) {
//     // Implement text rendering logic using layout
// }

// fn render_image(
//     img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
//     image_element: &ImageElement,
//     layout: &Layout,
// ) {
//     // Implement image rendering logic using layout
// }

// fn render_container(
//     img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
//     container: &Container,
//     layout: &Layout,
// ) {
//     // Implement container rendering logic using layout
// }

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

//         let card = parse_adaptive_card(json_data);
//         let bitmap = render_card(&card, 800, 600);
//         bitmap.save("output.png").unwrap();
//     }
// }
