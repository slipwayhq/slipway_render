#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Expresses a class"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expresses a class\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"description\": \"JSON schema for the JSON file\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of the class\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extends\": {"]
#[doc = "      \"description\": \"Class that this class extends from\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"isAbstract\": {"]
#[doc = "      \"description\": \"Specifies whether this class is abstract\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"properties\": {"]
#[doc = "      \"description\": \"Properties of the class\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/Property\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"shorthand\": {"]
#[doc = "      \"description\": \"Name of one of the properties that represents a shorthand version of this class\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The name of the class\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Class {
    #[doc = "A description of the class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Class that this class extends from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
    #[doc = "Specifies whether this class is abstract"]
    #[serde(
        rename = "isAbstract",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_abstract: Option<bool>,
    #[doc = "Properties of the class"]
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub properties: std::collections::HashMap<String, Property>,
    #[doc = "JSON schema for the JSON file"]
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "Name of one of the properties that represents a shorthand version of this class"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shorthand: Option<String>,
    #[doc = "The name of the class"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&Class> for Class {
    fn from(value: &Class) -> Self {
        value.clone()
    }
}
impl Class {
    pub fn builder() -> builder::Class {
        Default::default()
    }
}
#[doc = "Expresses an enum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expresses an enum\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\","]
#[doc = "    \"values\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"description\": \"JSON schema for the JSON file\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The name of the enum\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"description\": \"The values in the enum\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/EnumValue\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Enum {
    #[doc = "JSON schema for the JSON file"]
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The name of the enum"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The values in the enum"]
    pub values: Vec<EnumValue>,
}
impl From<&Enum> for Enum {
    fn from(value: &Enum) -> Self {
        value.clone()
    }
}
impl Enum {
    pub fn builder() -> builder::Enum {
        Default::default()
    }
}
#[doc = "Expresses an enum value"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Expresses an enum value\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"The name of the enum value\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"description\": {"]
#[doc = "          \"description\": \"A description of the enum value\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"description\": \"Must be `EnumValue`\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"EnumValue\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"description\": \"The name of the enum value\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum EnumValue {
    Variant0(String),
    Variant1 {
        #[doc = "A description of the enum value"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "Must be `EnumValue`"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<EnumValueVariant1Type>,
        #[doc = "The name of the enum value"]
        value: String,
    },
}
impl From<&EnumValue> for EnumValue {
    fn from(value: &EnumValue) -> Self {
        value.clone()
    }
}
#[doc = "Must be `EnumValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Must be `EnumValue`\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"EnumValue\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EnumValueVariant1Type {
    EnumValue,
}
impl From<&EnumValueVariant1Type> for EnumValueVariant1Type {
    fn from(value: &EnumValueVariant1Type) -> Self {
        value.clone()
    }
}
impl ToString for EnumValueVariant1Type {
    fn to_string(&self) -> String {
        match *self {
            Self::EnumValue => "EnumValue".to_string(),
        }
    }
}
impl std::str::FromStr for EnumValueVariant1Type {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "EnumValue" => Ok(Self::EnumValue),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for EnumValueVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EnumValueVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EnumValueVariant1Type {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Property"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"The default value of this property\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A description of the property\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"examples\": {"]
#[doc = "      \"description\": \"Examples of this value\","]
#[doc = "      \"type\": \"array\""]
#[doc = "    },"]
#[doc = "    \"override\": {"]
#[doc = "      \"description\": \"Set this to true if you're overriding a property from an extended class. This will prevent a warning from being emitted.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"Specifies whether the property is required\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"shorthands\": {"]
#[doc = "      \"description\": \"Shorthand alternatives for this property\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Property\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the property\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Property {
    #[doc = "The default value of this property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[doc = "A description of the property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Examples of this value"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<serde_json::Value>,
    #[doc = "Set this to true if you're overriding a property from an extended class. This will prevent a warning from being emitted."]
    #[serde(rename = "override", default, skip_serializing_if = "Option::is_none")]
    pub override_: Option<bool>,
    #[doc = "Specifies whether the property is required"]
    #[serde(default)]
    pub required: bool,
    #[doc = "Shorthand alternatives for this property"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shorthands: Vec<Property>,
    #[doc = "The type of the property"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&Property> for Property {
    fn from(value: &Property) -> Self {
        value.clone()
    }
}
impl Property {
    pub fn builder() -> builder::Property {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Class {
        description: Result<Option<String>, String>,
        extends: Result<Option<String>, String>,
        is_abstract: Result<Option<bool>, String>,
        properties: Result<std::collections::HashMap<String, super::Property>, String>,
        schema: Result<Option<String>, String>,
        shorthand: Result<Option<String>, String>,
        type_: Result<String, String>,
    }
    impl Default for Class {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                extends: Ok(Default::default()),
                is_abstract: Ok(Default::default()),
                properties: Ok(Default::default()),
                schema: Ok(Default::default()),
                shorthand: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Class {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn extends<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.extends = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extends: {}", e));
            self
        }
        pub fn is_abstract<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.is_abstract = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_abstract: {}", e));
            self
        }
        pub fn properties<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<std::collections::HashMap<String, super::Property>>,
            T::Error: std::fmt::Display,
        {
            self.properties = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for properties: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn shorthand<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.shorthand = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shorthand: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Class> for super::Class {
        type Error = super::error::ConversionError;
        fn try_from(value: Class) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                extends: value.extends?,
                is_abstract: value.is_abstract?,
                properties: value.properties?,
                schema: value.schema?,
                shorthand: value.shorthand?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Class> for Class {
        fn from(value: super::Class) -> Self {
            Self {
                description: Ok(value.description),
                extends: Ok(value.extends),
                is_abstract: Ok(value.is_abstract),
                properties: Ok(value.properties),
                schema: Ok(value.schema),
                shorthand: Ok(value.shorthand),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Enum {
        schema: Result<Option<String>, String>,
        type_: Result<String, String>,
        values: Result<Vec<super::EnumValue>, String>,
    }
    impl Default for Enum {
        fn default() -> Self {
            Self {
                schema: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl Enum {
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::EnumValue>>,
            T::Error: std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Enum> for super::Enum {
        type Error = super::error::ConversionError;
        fn try_from(value: Enum) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                schema: value.schema?,
                type_: value.type_?,
                values: value.values?,
            })
        }
    }
    impl From<super::Enum> for Enum {
        fn from(value: super::Enum) -> Self {
            Self {
                schema: Ok(value.schema),
                type_: Ok(value.type_),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Property {
        default: Result<Option<serde_json::Value>, String>,
        description: Result<Option<String>, String>,
        examples: Result<Vec<serde_json::Value>, String>,
        override_: Result<Option<bool>, String>,
        required: Result<bool, String>,
        shorthands: Result<Vec<super::Property>, String>,
        type_: Result<String, String>,
    }
    impl Default for Property {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
                description: Ok(Default::default()),
                examples: Ok(Default::default()),
                override_: Ok(Default::default()),
                required: Ok(Default::default()),
                shorthands: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Property {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn examples<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.examples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for examples: {}", e));
            self
        }
        pub fn override_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.override_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for override_: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn shorthands<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Property>>,
            T::Error: std::fmt::Display,
        {
            self.shorthands = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shorthands: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Property> for super::Property {
        type Error = super::error::ConversionError;
        fn try_from(value: Property) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                description: value.description?,
                examples: value.examples?,
                override_: value.override_?,
                required: value.required?,
                shorthands: value.shorthands?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Property> for Property {
        fn from(value: super::Property) -> Self {
            Self {
                default: Ok(value.default),
                description: Ok(value.description),
                examples: Ok(value.examples),
                override_: Ok(value.override_),
                required: Ok(value.required),
                shorthands: Ok(value.shorthands),
                type_: Ok(value.type_),
            }
        }
    }
}
