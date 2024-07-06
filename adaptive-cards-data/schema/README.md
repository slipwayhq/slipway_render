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
```
            "classType": {
              "enum": [
                "EnumValue"
              ],
              "description": "Must be `EnumValue`"
            },
```
add
```
            "version": {
              "type": "string",
              "description": "The version of Adaptive Cards that this enum value was introduced in"
            }
```



# Adaptive Cards Rust Types

Generated from the above schema using:
```sh
cargo typify typed-schema.schema.json
```


# Typed Schema files

> `typed-schema-1.6/**`

These define the Adaptive Cards schema in Typed Schema format.

These come from here:

https://github.com/microsoft/AdaptiveCards/tree/main/schemas/src

