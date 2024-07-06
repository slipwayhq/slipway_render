use super::generated::{
    Action, ActionOrFallbackOption, BackgroundImage, BackgroundImageOrString, BlockElementHeight,
    Column, ColumnOrFallbackOption, Element, ElementOrFallbackOption, FallbackOption, Inline,
    InlineOrString, StringOrBlockElementHeight, StringOrNumber, StringOrObject, TargetElement,
    TargetElementOrString,
};
use serde::{
    de::{self, IntoDeserializer, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;
use std::fmt;
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

impl<'de> Deserialize<'de> for StringOrBlockElementHeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_string_or_enum::<BlockElementHeight, StringOrBlockElementHeight, D>(
            deserializer,
            "a string or a BlockElementHeight",
        )
    }
}

// impl<'de> Deserialize<'de> for ElementOrFallbackOption {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserialize_string_or_other::<FallbackOption, Element, ElementOrFallbackOption, D>(
//             deserializer,
//             "an element or a fallback option",
//         )
//     }
// }
macro_rules! deserialize_string_or_other {
    ($expecting:expr, $t_string:ident, $t_struct:ident, $t_enum:ident) => {
        impl<'de> Deserialize<'de> for $t_enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_string_or_other::<$t_string, $t_struct, $t_enum, D>(
                    deserializer,
                    $expecting,
                )
            }
        }
    };
}

deserialize_string_or_other!("an object or a fallback", String, Value, StringOrObject);

deserialize_string_or_other!(
    "an element or a fallback",
    FallbackOption,
    Element,
    ElementOrFallbackOption
);

deserialize_string_or_other!(
    "a column or a fallback",
    FallbackOption,
    Column,
    ColumnOrFallbackOption
);

deserialize_string_or_other!(
    "an action or a fallback",
    FallbackOption,
    Action,
    ActionOrFallbackOption
);

deserialize_string_or_other!(
    "a background image or a string",
    String,
    BackgroundImage,
    BackgroundImageOrString
);

deserialize_string_or_other!(
    "a target element or element id",
    String,
    TargetElement,
    TargetElementOrString
);

deserialize_string_or_other!(
    "an inline (TextRun) or string",
    String,
    Inline,
    InlineOrString
);

fn deserialize_string_or_other<'de, 'expecting, TString, TStruct, TResult, D>(
    deserializer: D,
    expecting: &'expecting str,
) -> Result<TResult, D::Error>
where
    TString: Deserialize<'de> + Into<TResult>,
    TStruct: Deserialize<'de> + Into<TResult>,
    D: Deserializer<'de>,
{
    struct StringOrStructVisitor<'expecting, TString, TStruct, TResult> {
        expecting: &'expecting str,
        marker: std::marker::PhantomData<(TString, TStruct, TResult)>,
    }

    impl<'de, 'expecting, TString, TStruct, TResult> Visitor<'de>
        for StringOrStructVisitor<'expecting, TString, TStruct, TResult>
    where
        TString: Deserialize<'de> + Into<TResult>,
        TStruct: Deserialize<'de> + Into<TResult>,
    {
        type Value = TResult;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let t_string: TString = Deserialize::deserialize(value.into_deserializer())?;
            Ok(t_string.into())
        }

        fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let t_struct: TStruct =
                Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
            Ok(t_struct.into())
        }
    }

    deserializer.deserialize_any(StringOrStructVisitor::<TString, TStruct, TResult> {
        expecting,
        marker: std::marker::PhantomData,
    })
}

// fn deserialize_string_or_other_optional<'de, 'expecting, TString, TStruct, TResult, D>(
//     deserializer: D,
//     expecting: &'expecting str,
// ) -> Result<Option<TResult>, D::Error>
// where
//     TString: Deserialize<'de> + Into<TResult>,
//     TStruct: Deserialize<'de> + Into<TResult>,
//     D: Deserializer<'de>,
// {
//     struct StringOrStructVisitor<'expecting, TString, TStruct, TEnum> {
//         expecting: &'expecting str,
//         marker: std::marker::PhantomData<(TString, TStruct, TEnum)>,
//     }

//     impl<'de, 'expecting, TString, TStruct, TEnum> Visitor<'de>
//         for StringOrStructVisitor<'expecting, TString, TStruct, TEnum>
//     where
//         TString: Deserialize<'de> + Into<TEnum>,
//         TStruct: Deserialize<'de> + Into<TEnum>,
//     {
//         type Value = Option<TEnum>;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str(self.expecting)
//         }

//         fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             let t_string: TString = Deserialize::deserialize(value.into_deserializer())?;
//             Ok(Some(t_string.into()))
//         }

//         fn visit_map<M>(self, map: M) -> Result<Self::Value, M::Error>
//         where
//             M: de::MapAccess<'de>,
//         {
//             let t_struct: TStruct =
//                 Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))?;
//             Ok(Some(t_struct.into()))
//         }

//         fn visit_none<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_unit<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//         where
//             D: Deserializer<'de>,
//         {
//             let value =
//                 deserializer.deserialize_any(StringOrStructVisitor::<TString, TStruct, TEnum> {
//                     expecting: self.expecting,
//                     marker: PhantomData::<(TString, TStruct, TEnum)> {},
//                 })?;
//             Ok(value)
//         }
//     }

//     deserializer.deserialize_any(StringOrStructVisitor::<TString, TStruct, TResult> {
//         expecting,
//         marker: std::marker::PhantomData,
//     })
// }

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

impl<'de> Deserialize<'de> for StringOrNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl<'de> de::Visitor<'de> for StringOrNumberVisitor {
            type Value = StringOrNumber;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or a number")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber::String(value.to_owned()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber::Number(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber::Number(value as f64))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrNumber::Number(value as f64))
            }
        }

        deserializer.deserialize_any(StringOrNumberVisitor)
    }
}

// pub(super) fn deserialize_string_or_number_optional<'de, D>(
//     deserializer: D,
// ) -> Result<Option<StringOrNumber>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     struct StringOrNumberVisitor;

//     impl<'de> Visitor<'de> for StringOrNumberVisitor {
//         type Value = Option<StringOrNumber>;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str("a string, a number, or null")
//         }

//         fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(Some(StringOrNumber::String(value.to_owned())))
//         }

//         fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(Some(StringOrNumber::Number(value)))
//         }

//         fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(Some(StringOrNumber::Number(value as f64)))
//         }

//         fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(Some(StringOrNumber::Number(value as f64)))
//         }

//         fn visit_none<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_unit<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//         where
//             D: Deserializer<'de>,
//         {
//             let value = deserializer.deserialize_any(StringOrNumberVisitor)?;
//             Ok(value)
//         }
//     }

//     deserializer.deserialize_option(StringOrNumberVisitor)
// }

// pub(super) fn deserialize_string_or_block_element_height_optional<'de, D>(
//     deserializer: D,
// ) -> Result<Option<StringOrBlockElementHeight>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     deserialize_string_or_enum_optional::<BlockElementHeight, StringOrBlockElementHeight, D>(
//         deserializer,
//         "a string, a BlockElementHeight, or null",
//     )
// }

fn deserialize_string_or_enum<'de, 'expecting, TEnum, TResult, D>(
    deserializer: D,
    expecting: &'expecting str,
) -> Result<TResult, D::Error>
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
        type Value = TResult;

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
                Ok(enum_value) => Ok(enum_value.into()),
                Err(_) => Ok(value.to_owned().into()),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            self.visit_str(&String::deserialize(deserializer)?)
        }
    }

    deserializer.deserialize_option(StringOrEnumVisitor::<TEnum, TResult> {
        expecting,
        marker: std::marker::PhantomData,
    })
}

// fn deserialize_string_or_enum_optional<'de, 'expecting, TEnum, TResult, D>(
//     deserializer: D,
//     expecting: &'expecting str,
// ) -> Result<Option<TResult>, D::Error>
// where
//     TEnum: Deserialize<'de> + Into<TResult>,
//     TResult: From<String>,
//     D: Deserializer<'de>,
// {
//     struct StringOrEnumVisitor<'expecting, TEnum, TResult> {
//         expecting: &'expecting str,
//         marker: std::marker::PhantomData<(TEnum, TResult)>,
//     }

//     impl<'de, 'expecting, TEnum, TResult> Visitor<'de>
//         for StringOrEnumVisitor<'expecting, TEnum, TResult>
//     where
//         TEnum: Deserialize<'de> + Into<TResult>,
//         TResult: From<String>,
//     {
//         type Value = Option<TResult>;

//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str(self.expecting)
//         }

//         fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             let maybe_enum_value: Result<TEnum, E> =
//                 Deserialize::deserialize(value.into_deserializer());

//             match maybe_enum_value {
//                 Ok(enum_value) => Ok(Some(enum_value.into())),
//                 Err(_) => Ok(Some(value.to_owned().into())),
//             }
//         }

//         fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             self.visit_str(&value)
//         }

//         fn visit_none<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_unit<E>(self) -> Result<Self::Value, E>
//         where
//             E: de::Error,
//         {
//             Ok(None)
//         }

//         fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
//         where
//             D: Deserializer<'de>,
//         {
//             let value = deserializer.deserialize_any(StringOrEnumVisitor::<TEnum, TResult> {
//                 expecting: self.expecting,
//                 marker: PhantomData::<(TEnum, TResult)> {},
//             })?;
//             Ok(value)
//         }
//     }

//     deserializer.deserialize_option(StringOrEnumVisitor::<TEnum, TResult> {
//         expecting,
//         marker: std::marker::PhantomData,
//     })
// }
