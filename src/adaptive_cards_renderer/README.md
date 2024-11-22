# Overview

## Type Generation

We generate custom types from the Adaptive Cards typed schema files located in
`adaptive_cards_data/schema/typed-schema-1.6`.

Every Adaptive Cards type must implement the `Layoutable` trait. The code generator adds the
default trait implementation to any types marked as not explicitly implemented (see the
`UNIMPLEMENTED_LAYOUTABLE_TYPES` array).

As we implement support rendering more Adaptive Cards types we will remove the types from
the unimplemented list and implement the trait explicitly.

The code generator also implements the `HasLayoutData` trait for each type and adds the required 
`RefCell<ElementLayoutData>` data to the struct.

The `ElementLayoutData` contains the transient data used during the layout and draw passes.

The default `Layoutable` trait implementations of `layout` and `draw` are designed to be sufficient
for all types, and instead each type should override the implementation of `layout_override` and
`draw_override` which are called by `layout` and `draw` respectively.

Rendering starts with the `crate::render::render` method. This performs the card's `layout` pass which
populates all the data required by Taffy, then asks Taffy to compute the layout, and finally creates
the output image and asks the card to draw itself to the image.