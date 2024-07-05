use super::generated::{
    Action, ActionOrFallbackOption, BackgroundImage, BackgroundImageOrString, BlockElementHeight,
    Column, ColumnOrFallbackOption, Element, ElementOrFallbackOption, FallbackOption,
    StringOrBlockElementHeight, StringOrNumber, StringOrObject,
};
use serde::{
    de::{self, IntoDeserializer, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use std::{fmt, marker::PhantomData};
// StringOrEnum
// StringOrNumber
// StringOrValue
// TypeOrEnum

// pub(super) fn deserialize_string_or_object_optional<'de, D>(
//     deserializer: D,
// ) -> Result<Option<StringOrObject>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     string_or_other_optional::<String, Value, StringOrObject, D>(
//         deserializer,
//         "a string or an object or null",
//     )
// }

macro_rules! string_or_not_string {
    ($name:ident, $expecting:expr, $t_string:ident, $t_struct:ident, $enum:ident) => {
        pub(super) fn $name<'de, D>(deserializer: D) -> Result<Option<$enum>, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize_string_or_other_optional::<$t_string, $t_struct, $enum, D>(
                deserializer,
                $expecting,
            )
        }
    };
}

string_or_not_string!(
    deserialize_string_or_object_optional,
    "an object or a fallback or null",
    String,
    Value,
    StringOrObject
);

string_or_not_string!(
    deserialize_element_or_fallback_option_optional,
    "an element or a fallback or null",
    FallbackOption,
    Element,
    ElementOrFallbackOption
);

string_or_not_string!(
    deserialize_column_or_fallback_option_optional,
    "a column or a fallback or null",
    FallbackOption,
    Column,
    ColumnOrFallbackOption
);

string_or_not_string!(
    deserialize_action_or_fallback_option_optional,
    "an action or a fallback or null",
    FallbackOption,
    Action,
    ActionOrFallbackOption
);

string_or_not_string!(
    deserialize_background_image_or_string_optional,
    "a background image or a string or null",
    String,
    BackgroundImage,
    BackgroundImageOrString
);

fn deserialize_string_or_other_optional<'de, 'expecting, TString, TStruct, TResult, D>(
    deserializer: D,
    expecting: &'expecting str,
) -> Result<Option<TResult>, D::Error>
where
    TString: Deserialize<'de> + Into<TResult>,
    TStruct: Deserialize<'de> + Into<TResult>,
    D: Deserializer<'de>,
{
    struct StringOrStructVisitor<'expecting, TString, TStruct, TEnum> {
        expecting: &'expecting str,
        marker: std::marker::PhantomData<(TString, TStruct, TEnum)>,
    }

    impl<'de, 'expecting, TString, TStruct, TEnum> Visitor<'de>
        for StringOrStructVisitor<'expecting, TString, TStruct, TEnum>
    where
        TString: Deserialize<'de> + Into<TEnum>,
        TStruct: Deserialize<'de> + Into<TEnum>,
    {
        type Value = Option<TEnum>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let t_string: TString = Deserialize::deserialize(value.into_deserializer())?;
            Ok(Some(t_string.into()))
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let t_struct: TStruct =
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(Some(t_struct.into()))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value =
                deserializer.deserialize_any(StringOrStructVisitor::<TString, TStruct, TEnum> {
                    expecting: self.expecting,
                    marker: PhantomData::<(TString, TStruct, TEnum)> {},
                })?;
            Ok(value)
        }
    }

    deserializer.deserialize_any(StringOrStructVisitor::<TString, TStruct, TResult> {
        expecting,
        marker: std::marker::PhantomData,
    })
}

// pub(super) fn deserialize_string_or_number_optional<'de, D>(
//     deserializer: D,
// ) -> Result<Option<StringOrNumber>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     Ok(Some(match Value::deserialize(deserializer)? {
//         Value::String(s) => StringOrNumber::String(s),
//         Value::Number(n) if n.is_f64() => StringOrNumber::Number(n.as_f64().unwrap()),
//         Value::Null => return Ok(None),
//         _ => return Err(de::Error::custom("Invalid type for labelWidth")),
//     }))
// }
pub(super) fn deserialize_string_or_number_optional<'de, D>(
    deserializer: D,
) -> Result<Option<StringOrNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> Visitor<'de> for StringOrNumberVisitor {
        type Value = Option<StringOrNumber>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, a number, or null")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(StringOrNumber::String(value.to_owned())))
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(StringOrNumber::Number(value)))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(StringOrNumber::Number(value as f64)))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(StringOrNumber::Number(value as f64)))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = deserializer.deserialize_any(StringOrNumberVisitor)?;
            Ok(value)
        }
    }

    deserializer.deserialize_option(StringOrNumberVisitor)
}

pub(super) fn deserialize_string_or_block_element_height_optional<'de, D>(
    deserializer: D,
) -> Result<Option<StringOrBlockElementHeight>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_string_or_enum_optional::<BlockElementHeight, StringOrBlockElementHeight, D>(
        deserializer,
        "a string, a BlockElementHeight, or null",
    )
}

fn deserialize_string_or_enum_optional<'de, 'expecting, TEnum, TResult, D>(
    deserializer: D,
    expecting: &'expecting str,
) -> Result<Option<TResult>, D::Error>
where
    TEnum: Deserialize<'de> + Into<TResult>,
    TResult: From<String>,
    D: Deserializer<'de>,
{
    struct StringOrEnumVisitor<'expecting, TEnum, TResult> {
        expecting: &'expecting str,
        marker: std::marker::PhantomData<(TEnum, TResult)>,
    }

    impl<'de, 'expecting, TEnum, TResult> Visitor<'de>
        for StringOrEnumVisitor<'expecting, TEnum, TResult>
    where
        TEnum: Deserialize<'de> + Into<TResult>,
        TResult: From<String>,
    {
        type Value = Option<TResult>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let maybe_enum_value: Result<TEnum, E> =
                Deserialize::deserialize(value.into_deserializer());

            match maybe_enum_value {
                Ok(enum_value) => Ok(Some(enum_value.into())),
                Err(_) => Ok(Some(value.to_owned().into())),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value = deserializer.deserialize_any(StringOrEnumVisitor::<TEnum, TResult> {
                expecting: self.expecting,
                marker: PhantomData::<(TEnum, TResult)> {},
            })?;
            Ok(value)
        }
    }

    deserializer.deserialize_option(StringOrEnumVisitor::<TEnum, TResult> {
        expecting,
        marker: std::marker::PhantomData,
    })
}
