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

macro_rules! deserialize_string_or_other_generic {
    ($expecting:expr, $t_string:ident, $t_struct:ident, $t_enum:ident) => {
        impl<'de, TLayoutData> Deserialize<'de> for $t_enum<TLayoutData>
        where
            TLayoutData: Default + Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserialize_string_or_other::<
                    $t_string,
                    $t_struct<TLayoutData>,
                    $t_enum<TLayoutData>,
                    D,
                >(deserializer, $expecting)
            }
        }
    };
}

deserialize_string_or_other!("an object or a fallback", String, Value, StringOrObject);

deserialize_string_or_other_generic!(
    "an element or a fallback",
    FallbackOption,
    Element,
    ElementOrFallbackOption
);

deserialize_string_or_other_generic!(
    "a column or a fallback",
    FallbackOption,
    Column,
    ColumnOrFallbackOption
);

deserialize_string_or_other_generic!(
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

deserialize_string_or_other_generic!(
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

    impl<'de, TString, TStruct, TResult> Visitor<'de>
        for StringOrStructVisitor<'_, TString, TStruct, TResult>
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

impl<'de> Deserialize<'de> for StringOrNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrNumberVisitor;

        impl de::Visitor<'_> for StringOrNumberVisitor {
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

    impl<'de, TEnum, TResult> Visitor<'de> for StringOrEnumVisitor<'_, TEnum, TResult>
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
    }

    deserializer.deserialize_any(StringOrEnumVisitor::<TEnum, TResult> {
        expecting,
        marker: std::marker::PhantomData,
    })
}
