# Adaptive Cards Samples

This folder comes from here:

https://github.com/microsoft/AdaptiveCards/tree/main/samples


# Adaptive Cards Typed Schema

> `typed-schema.schema.json`

This is the schema for the Adaptive Cards "Typed Schema" format.

This format is how the Adaptive Cards repo defines the Adaptive Cards schemas before converting them to JSON Schema format.

We use this to generate Rust types for reading the Adaptive Cards Typed Schema files.

This file comes from here:

https://github.com/microsoft/AdaptiveCards/blob/main/source/nodejs/ac-typed-schema/schema/lib/Type.json

Currently you need to manually add the following to the Enum definition:

```json
        "description": {
          "type": "string",
          "description": "The description of the enum"
        },
        "version": {
          "type": "string",
          "description": "The version of Adaptive Cards that this enum was introduced in"
        },
        "features": {
          "type": "array",
          "items": {
            "type": "number"
          },
          "description": "Features of the enum"
        },
```

The following should be removed from Enum and the `required` properties:
```json
        "type": {
          "type": "string",
          "description": "The name of the enum"
        },
```

Remove from EnumValue:
```json
            "classType": {
              "enum": [
                "EnumValue"
              ],
              "description": "Must be `EnumValue`"
            },
```
add
```json
            "version": {
              "type": "string",
              "description": "The version of Adaptive Cards that this enum value was introduced in"
            }
```

# Some fixes have been applied locally

https://github.com/microsoft/AdaptiveCards/discussions/8943


# Adaptive Cards Rust Types

Generated from the above schema using:
```sh
cargo typify typed-schema.schema.json
```


# Typed Schema files

> `typed-schema-1.6/**`

These define the Adaptive Cards schema in Typed Schema format.

These come from here:

https://github.com/microsoft/AdaptiveCards/tree/main/schemas


# Deviations from Adaptive Cards standard rendering

While I'm trying to retain compatibility with Adaptive Cards as much as possible,
compatibility isn't actually essential (our use cases are quite different), and I don't want
to adopt their mistakes.

I'll list here any deviations from what Adaptive Cards does here.

## Horizontal Alignment on Containers and root AdaptiveCard.

This is not supported, which seems surprising to me. Other people have also queried this:

https://github.com/microsoft/AdaptiveCards/issues/8535

I've added support in Container, Column and AdaptiveCard.

## BlockElementWidth

Previously element heights were `string|BlockElementHeight` but column widths were `string|number` where number could be `auto` or `stretch` or something else.

Now column widths are `string|BlockElementWidth|number` for consistency with element heights.

## VerticalContentAlignment removed, use VerticalAlignment instead.

These two have identical definitions, and are used interchangeably (see Table, TableRow, TableCell).

It seems like Table, TableRow, TableColumn should technically have used VerticalContentAlignment, but because there
wasn't an equivalent HorizontalContentAlignment they used HorizontalAlignment and VerticalAlignment for consistency
between the properties, even though it created an inconsistency with TableCell.

## hostConfig.containerStyles.[styleName].borderColor

In the official Adaptive Cards implementation the `borderColor` property of a container style is not used for
specifying the border color of a container, but is instead used to color the grid lines of a table when 
the "gridStyle" property is specified.

For Slipway, the `borderColor` **is** used for the border color of a container, adhering the principle of least
surprise.

How I color the grid lines will be decided when I implement tables. Perhaps using a separate `gridColor` property, or perhaps using the `borderColor` property.

## hostConfig.font[Family|Sizes|Weights]

Deprecated. `hostConfig.fontTypes` should be used instead. These fields are ignored by the renderer.
https://github.com/microsoft/AdaptiveCards/issues/1078


## font weights are consistently "lighter", "default" and "bolder", rather than occasionally "lighter", "normal" and "bolder".

# Not Yet Supported

- Actions
- RTL (right-to-left) flags
