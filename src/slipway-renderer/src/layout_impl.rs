use image::{ImageBuffer, Rgba};

use crate::{layoutable::Layoutable, AdaptiveCard, Element, Rect, Size};

// impl Layoutable for AdaptiveCard {
//     fn measure(&self, available_size: Size) -> Size {
//         let mut desired_size = Size::new(0.0, 0.0);

//         for element in &self.body {
//             let element_size = element.measure(available_size);
//             desired_size.width = desired_size.width.max(element_size.width);
//             desired_size.height += element_size.height;
//         }

//         desired_size
//     }

//     fn arrange(&self, final_rect: Rect) -> Rect {
//         let mut y = final_rect.y;

//         for element in &self.body {
//             let element_size = element.layout_data().borrow().actual_rect.unwrap();
//             let element_rect = Rect::new(final_rect.x, y, final_rect.width, element_size.height);
//             element.arrange(element_rect);
//             y += element_size.height;
//         }

//         Rect::new(final_rect.x, final_rect.y, final_rect.width, y - final_rect.y)
//     }

//     fn draw(&self, rect: Rect, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
//         for element in &self.body {
//             let element_rect = element.layout_data().borrow().actual_rect.unwrap();
//             element.draw(element_rect, image);
//         }
//     }

//     fn layout_data(&self) -> RefCell<LayoutData> {
//         self.layout_data.clone()
//     }
// }

// impl Element {
//     fn as_layoutable(&self) -> &dyn Layoutable {
//         match self {
//             Element::TextBlock(e) => e,
//             Element::Image(e) => e,
//             Element::Container(e) => e,
//             Element::ColumnSet(e) => e,
//             Element::Column(e) => e,
//             Element::FactSet(e) => e,
//             Element::ImageSet(e) => e,
//             Element::ActionSet(e) => e,
//             Element::RichTextBlock(e) => e,
//             Element::Custom(e) => e,
//         }
//     }
// }
