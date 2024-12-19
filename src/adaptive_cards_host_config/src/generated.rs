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
#[doc = "Options for `Action`s"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Options for `Action`s\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"actionAlignment\": {"]
#[doc = "      \"description\": \"Control layout of buttons\","]
#[doc = "      \"default\": \"stretch\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"left\","]
#[doc = "        \"center\","]
#[doc = "        \"right\","]
#[doc = "        \"stretch\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"actionsOrientation\": {"]
#[doc = "      \"description\": \"Controls how buttons are laid out\","]
#[doc = "      \"default\": \"horizontal\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"horizontal\","]
#[doc = "        \"vertical\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"buttonSpacing\": {"]
#[doc = "      \"description\": \"Controls how much spacing to use between buttons\","]
#[doc = "      \"default\": 10,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"iconPlacement\": {"]
#[doc = "      \"description\": \"Controls where to place the action icon\","]
#[doc = "      \"default\": \"aboveTitle\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"aboveTitle\","]
#[doc = "        \"leftOfTitle\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"iconSize\": {"]
#[doc = "      \"description\": \"Controls size of action icon\","]
#[doc = "      \"default\": 30,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"maxActions\": {"]
#[doc = "      \"description\": \"Controls how many actions are allowed in total\","]
#[doc = "      \"default\": 5,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"preExpandSingleShowCardAction\": {"]
#[doc = "      \"description\": \"Controls whether to pre-expand single show card actions\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"showCard\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"actionMode\": \"inline\","]
#[doc = "        \"inlineTopMargin\": 16,"]
#[doc = "        \"style\": \"emphasis\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ShowCardConfig\""]
#[doc = "    },"]
#[doc = "    \"spacing\": {"]
#[doc = "      \"description\": \"Controls overall spacing of action element\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"none\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\","]
#[doc = "        \"padding\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActionsConfig {
    #[doc = "Control layout of buttons"]
    #[serde(
        rename = "actionAlignment",
        default = "defaults::actions_config_action_alignment"
    )]
    pub action_alignment: ActionsConfigActionAlignment,
    #[doc = "Controls how buttons are laid out"]
    #[serde(
        rename = "actionsOrientation",
        default = "defaults::actions_config_actions_orientation"
    )]
    pub actions_orientation: ActionsConfigActionsOrientation,
    #[doc = "Controls how much spacing to use between buttons"]
    #[serde(rename = "buttonSpacing", default = "defaults::default_u64::<i64, 10>")]
    pub button_spacing: i64,
    #[doc = "Controls where to place the action icon"]
    #[serde(
        rename = "iconPlacement",
        default = "defaults::actions_config_icon_placement"
    )]
    pub icon_placement: ActionsConfigIconPlacement,
    #[doc = "Controls size of action icon"]
    #[serde(rename = "iconSize", default = "defaults::default_u64::<i64, 30>")]
    pub icon_size: i64,
    #[doc = "Controls how many actions are allowed in total"]
    #[serde(rename = "maxActions", default = "defaults::default_u64::<i64, 5>")]
    pub max_actions: i64,
    #[doc = "Controls whether to pre-expand single show card actions"]
    #[serde(rename = "preExpandSingleShowCardAction", default)]
    pub pre_expand_single_show_card_action: bool,
    #[serde(rename = "showCard", default = "defaults::actions_config_show_card")]
    pub show_card: ShowCardConfig,
    #[doc = "Controls overall spacing of action element"]
    #[serde(default = "defaults::actions_config_spacing")]
    pub spacing: ActionsConfigSpacing,
}
impl From<&ActionsConfig> for ActionsConfig {
    fn from(value: &ActionsConfig) -> Self {
        value.clone()
    }
}
impl ActionsConfig {
    pub fn builder() -> builder::ActionsConfig {
        Default::default()
    }
}
#[doc = "Control layout of buttons"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Control layout of buttons\","]
#[doc = "  \"default\": \"stretch\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"left\","]
#[doc = "    \"center\","]
#[doc = "    \"right\","]
#[doc = "    \"stretch\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ActionsConfigActionAlignment {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "stretch")]
    Stretch,
}
impl From<&ActionsConfigActionAlignment> for ActionsConfigActionAlignment {
    fn from(value: &ActionsConfigActionAlignment) -> Self {
        value.clone()
    }
}
impl ToString for ActionsConfigActionAlignment {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "left".to_string(),
            Self::Center => "center".to_string(),
            Self::Right => "right".to_string(),
            Self::Stretch => "stretch".to_string(),
        }
    }
}
impl std::str::FromStr for ActionsConfigActionAlignment {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            "stretch" => Ok(Self::Stretch),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ActionsConfigActionAlignment {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ActionsConfigActionAlignment {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ActionsConfigActionAlignment {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ActionsConfigActionAlignment {
    fn default() -> Self {
        ActionsConfigActionAlignment::Stretch
    }
}
#[doc = "Controls how buttons are laid out"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls how buttons are laid out\","]
#[doc = "  \"default\": \"horizontal\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"horizontal\","]
#[doc = "    \"vertical\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ActionsConfigActionsOrientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}
impl From<&ActionsConfigActionsOrientation> for ActionsConfigActionsOrientation {
    fn from(value: &ActionsConfigActionsOrientation) -> Self {
        value.clone()
    }
}
impl ToString for ActionsConfigActionsOrientation {
    fn to_string(&self) -> String {
        match *self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
        }
    }
}
impl std::str::FromStr for ActionsConfigActionsOrientation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "horizontal" => Ok(Self::Horizontal),
            "vertical" => Ok(Self::Vertical),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ActionsConfigActionsOrientation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ActionsConfigActionsOrientation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ActionsConfigActionsOrientation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ActionsConfigActionsOrientation {
    fn default() -> Self {
        ActionsConfigActionsOrientation::Horizontal
    }
}
#[doc = "Controls where to place the action icon"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls where to place the action icon\","]
#[doc = "  \"default\": \"aboveTitle\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"aboveTitle\","]
#[doc = "    \"leftOfTitle\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ActionsConfigIconPlacement {
    #[serde(rename = "aboveTitle")]
    AboveTitle,
    #[serde(rename = "leftOfTitle")]
    LeftOfTitle,
}
impl From<&ActionsConfigIconPlacement> for ActionsConfigIconPlacement {
    fn from(value: &ActionsConfigIconPlacement) -> Self {
        value.clone()
    }
}
impl ToString for ActionsConfigIconPlacement {
    fn to_string(&self) -> String {
        match *self {
            Self::AboveTitle => "aboveTitle".to_string(),
            Self::LeftOfTitle => "leftOfTitle".to_string(),
        }
    }
}
impl std::str::FromStr for ActionsConfigIconPlacement {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "aboveTitle" => Ok(Self::AboveTitle),
            "leftOfTitle" => Ok(Self::LeftOfTitle),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ActionsConfigIconPlacement {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ActionsConfigIconPlacement {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ActionsConfigIconPlacement {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ActionsConfigIconPlacement {
    fn default() -> Self {
        ActionsConfigIconPlacement::AboveTitle
    }
}
#[doc = "Controls overall spacing of action element"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls overall spacing of action element\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"none\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\","]
#[doc = "    \"padding\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ActionsConfigSpacing {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
    #[serde(rename = "padding")]
    Padding,
}
impl From<&ActionsConfigSpacing> for ActionsConfigSpacing {
    fn from(value: &ActionsConfigSpacing) -> Self {
        value.clone()
    }
}
impl ToString for ActionsConfigSpacing {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::None => "none".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
            Self::Padding => "padding".to_string(),
        }
    }
}
impl std::str::FromStr for ActionsConfigSpacing {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "none" => Ok(Self::None),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            "padding" => Ok(Self::Padding),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ActionsConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ActionsConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ActionsConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ActionsConfigSpacing {
    fn default() -> Self {
        ActionsConfigSpacing::Default
    }
}
#[doc = "Toplevel options for `AdaptiveCards`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Toplevel options for `AdaptiveCards`\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowCustomStyle\": {"]
#[doc = "      \"description\": \"Controls whether custom styling is allowed\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveCardConfig {
    #[doc = "Controls whether custom styling is allowed"]
    #[serde(
        rename = "allowCustomStyle",
        default = "defaults::default_bool::<true>"
    )]
    pub allow_custom_style: bool,
}
impl From<&AdaptiveCardConfig> for AdaptiveCardConfig {
    fn from(value: &AdaptiveCardConfig) -> Self {
        value.clone()
    }
}
impl AdaptiveCardConfig {
    pub fn builder() -> builder::AdaptiveCardConfig {
        Default::default()
    }
}
#[doc = "Controls styling of a container"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls styling of a container\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"backgroundColor\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"borderColor\": {"]
#[doc = "      \"description\": \"Color of borders.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"borderThickness\": {"]
#[doc = "      \"description\": \"Thickness of border line.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"foregroundColors\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"accent\": {"]
#[doc = "          \"default\": \"#6264a7\","]
#[doc = "          \"subtle\": \"#8b8cc7\""]
#[doc = "        },"]
#[doc = "        \"attention\": {"]
#[doc = "          \"default\": \"#c4314b\","]
#[doc = "          \"subtle\": \"#e5c4314b\""]
#[doc = "        },"]
#[doc = "        \"dark\": {"]
#[doc = "          \"default\": \"#252424\","]
#[doc = "          \"subtle\": \"#bf252424\""]
#[doc = "        },"]
#[doc = "        \"default\": {"]
#[doc = "          \"default\": \"#ff252424\","]
#[doc = "          \"subtle\": \"#bf252424\""]
#[doc = "        },"]
#[doc = "        \"good\": {"]
#[doc = "          \"default\": \"#92c353\","]
#[doc = "          \"subtle\": \"#e592c353\""]
#[doc = "        },"]
#[doc = "        \"light\": {"]
#[doc = "          \"default\": \"#ffffff\","]
#[doc = "          \"subtle\": \"#fff3f2f1\""]
#[doc = "        },"]
#[doc = "        \"warning\": {"]
#[doc = "          \"default\": \"#f8d22a\","]
#[doc = "          \"subtle\": \"#e5f8d22a\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ForegroundColorsConfig\""]
#[doc = "    },"]
#[doc = "    \"tableGridLinesColor\": {"]
#[doc = "      \"description\": \"Color of table grid lines. Falls back to the border color, then separator color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tableGridLinesThickness\": {"]
#[doc = "      \"description\": \"Thickness of table grid lines. Falls back to separator thickness.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainerStyleConfig {
    #[serde(
        rename = "backgroundColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub background_color: Option<String>,
    #[doc = "Color of borders."]
    #[serde(
        rename = "borderColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub border_color: Option<String>,
    #[doc = "Thickness of border line."]
    #[serde(
        rename = "borderThickness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub border_thickness: Option<i64>,
    #[serde(
        rename = "foregroundColors",
        default = "defaults::container_style_config_foreground_colors"
    )]
    pub foreground_colors: ForegroundColorsConfig,
    #[doc = "Color of table grid lines. Falls back to the border color, then separator color."]
    #[serde(
        rename = "tableGridLinesColor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub table_grid_lines_color: Option<String>,
    #[doc = "Thickness of table grid lines. Falls back to separator thickness."]
    #[serde(
        rename = "tableGridLinesThickness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub table_grid_lines_thickness: Option<i64>,
}
impl From<&ContainerStyleConfig> for ContainerStyleConfig {
    fn from(value: &ContainerStyleConfig) -> Self {
        value.clone()
    }
}
impl ContainerStyleConfig {
    pub fn builder() -> builder::ContainerStyleConfig {
        Default::default()
    }
}
#[doc = "Controls styling for default and emphasis containers"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls styling for default and emphasis containers\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"accent\": {"]
#[doc = "      \"description\": \"Container style to use for accent\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#C7DEF9\","]
#[doc = "        \"borderColor\": \"#62A8F7\","]
#[doc = "        \"borderThickness\": 1,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"attention\": {"]
#[doc = "      \"description\": \"Container style to use for attention\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#FFC5B2\","]
#[doc = "        \"borderColor\": \"#FF764C\","]
#[doc = "        \"borderThickness\": 1,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Default container style\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#ffffff\","]
#[doc = "        \"borderColor\": \"#CCCCCC\","]
#[doc = "        \"borderThickness\": 0,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"emphasis\": {"]
#[doc = "      \"description\": \"Container style to use for emphasis\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#fff9f8f7\","]
#[doc = "        \"borderColor\": \"#666666\","]
#[doc = "        \"borderThickness\": 1,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"good\": {"]
#[doc = "      \"description\": \"Container style to use for good\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#CCFFCC\","]
#[doc = "        \"borderColor\": \"#69E569\","]
#[doc = "        \"borderThickness\": 1,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"warning\": {"]
#[doc = "      \"description\": \"Container style to use for warning\","]
#[doc = "      \"default\": {"]
#[doc = "        \"backgroundColor\": \"#FFE2B2\","]
#[doc = "        \"borderColor\": \"#FFBC51\","]
#[doc = "        \"borderThickness\": 1,"]
#[doc = "        \"foregroundColors\": {"]
#[doc = "          \"accent\": {"]
#[doc = "            \"default\": \"#6264a7\","]
#[doc = "            \"subtle\": \"#8b8cc7\""]
#[doc = "          },"]
#[doc = "          \"attention\": {"]
#[doc = "            \"default\": \"#c4314b\","]
#[doc = "            \"subtle\": \"#e5c4314b\""]
#[doc = "          },"]
#[doc = "          \"dark\": {"]
#[doc = "            \"default\": \"#252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"default\": {"]
#[doc = "            \"default\": \"#ff252424\","]
#[doc = "            \"subtle\": \"#bf252424\""]
#[doc = "          },"]
#[doc = "          \"good\": {"]
#[doc = "            \"default\": \"#92c353\","]
#[doc = "            \"subtle\": \"#e592c353\""]
#[doc = "          },"]
#[doc = "          \"light\": {"]
#[doc = "            \"default\": \"#ffffff\","]
#[doc = "            \"subtle\": \"#fff3f2f1\""]
#[doc = "          },"]
#[doc = "          \"warning\": {"]
#[doc = "            \"default\": \"#f8d22a\","]
#[doc = "            \"subtle\": \"#e5f8d22a\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"tableGridLinesColor\": null"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStyleConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContainerStylesConfig {
    #[doc = "Container style to use for accent"]
    #[serde(default = "defaults::container_styles_config_accent")]
    pub accent: ContainerStyleConfig,
    #[doc = "Container style to use for attention"]
    #[serde(default = "defaults::container_styles_config_attention")]
    pub attention: ContainerStyleConfig,
    #[doc = "Default container style"]
    #[serde(default = "defaults::container_styles_config_default")]
    pub default: ContainerStyleConfig,
    #[doc = "Container style to use for emphasis"]
    #[serde(default = "defaults::container_styles_config_emphasis")]
    pub emphasis: ContainerStyleConfig,
    #[doc = "Container style to use for good"]
    #[serde(default = "defaults::container_styles_config_good")]
    pub good: ContainerStyleConfig,
    #[doc = "Container style to use for warning"]
    #[serde(default = "defaults::container_styles_config_warning")]
    pub warning: ContainerStyleConfig,
}
impl From<&ContainerStylesConfig> for ContainerStylesConfig {
    fn from(value: &ContainerStylesConfig) -> Self {
        value.clone()
    }
}
impl ContainerStylesConfig {
    pub fn builder() -> builder::ContainerStylesConfig {
        Default::default()
    }
}
#[doc = "Controls styling for input error messages"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls styling for input error messages\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Font size to use for the error message\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"small\","]
#[doc = "        \"default\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"spacing\": {"]
#[doc = "      \"description\": \"Amount of spacing to be used between input and error message\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"none\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\","]
#[doc = "        \"padding\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"weight\": {"]
#[doc = "      \"description\": \"Font weight that should be used for error messages\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"lighter\","]
#[doc = "        \"default\","]
#[doc = "        \"bolder\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"version\": \"1.3\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorMessageConfig {
    #[doc = "Font size to use for the error message"]
    #[serde(default = "defaults::error_message_config_size")]
    pub size: ErrorMessageConfigSize,
    #[doc = "Amount of spacing to be used between input and error message"]
    #[serde(default = "defaults::error_message_config_spacing")]
    pub spacing: ErrorMessageConfigSpacing,
    #[doc = "Font weight that should be used for error messages"]
    #[serde(default = "defaults::error_message_config_weight")]
    pub weight: ErrorMessageConfigWeight,
}
impl From<&ErrorMessageConfig> for ErrorMessageConfig {
    fn from(value: &ErrorMessageConfig) -> Self {
        value.clone()
    }
}
impl ErrorMessageConfig {
    pub fn builder() -> builder::ErrorMessageConfig {
        Default::default()
    }
}
#[doc = "Font size to use for the error message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Font size to use for the error message\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"small\","]
#[doc = "    \"default\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ErrorMessageConfigSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
}
impl From<&ErrorMessageConfigSize> for ErrorMessageConfigSize {
    fn from(value: &ErrorMessageConfigSize) -> Self {
        value.clone()
    }
}
impl ToString for ErrorMessageConfigSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Small => "small".to_string(),
            Self::Default => "default".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
        }
    }
}
impl std::str::FromStr for ErrorMessageConfigSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "small" => Ok(Self::Small),
            "default" => Ok(Self::Default),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ErrorMessageConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ErrorMessageConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ErrorMessageConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ErrorMessageConfigSize {
    fn default() -> Self {
        ErrorMessageConfigSize::Default
    }
}
#[doc = "Amount of spacing to be used between input and error message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Amount of spacing to be used between input and error message\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"none\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\","]
#[doc = "    \"padding\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ErrorMessageConfigSpacing {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
    #[serde(rename = "padding")]
    Padding,
}
impl From<&ErrorMessageConfigSpacing> for ErrorMessageConfigSpacing {
    fn from(value: &ErrorMessageConfigSpacing) -> Self {
        value.clone()
    }
}
impl ToString for ErrorMessageConfigSpacing {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::None => "none".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
            Self::Padding => "padding".to_string(),
        }
    }
}
impl std::str::FromStr for ErrorMessageConfigSpacing {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "none" => Ok(Self::None),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            "padding" => Ok(Self::Padding),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ErrorMessageConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ErrorMessageConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ErrorMessageConfigSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ErrorMessageConfigSpacing {
    fn default() -> Self {
        ErrorMessageConfigSpacing::Default
    }
}
#[doc = "Font weight that should be used for error messages"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Font weight that should be used for error messages\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"lighter\","]
#[doc = "    \"default\","]
#[doc = "    \"bolder\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ErrorMessageConfigWeight {
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "bolder")]
    Bolder,
}
impl From<&ErrorMessageConfigWeight> for ErrorMessageConfigWeight {
    fn from(value: &ErrorMessageConfigWeight) -> Self {
        value.clone()
    }
}
impl ToString for ErrorMessageConfigWeight {
    fn to_string(&self) -> String {
        match *self {
            Self::Lighter => "lighter".to_string(),
            Self::Default => "default".to_string(),
            Self::Bolder => "bolder".to_string(),
        }
    }
}
impl std::str::FromStr for ErrorMessageConfigWeight {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "lighter" => Ok(Self::Lighter),
            "default" => Ok(Self::Default),
            "bolder" => Ok(Self::Bolder),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ErrorMessageConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ErrorMessageConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ErrorMessageConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ErrorMessageConfigWeight {
    fn default() -> Self {
        ErrorMessageConfigWeight::Default
    }
}
#[doc = "Controls the display of `FactSet`s"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls the display of `FactSet`s\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"spacing\": {"]
#[doc = "      \"default\": 16,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"fontType\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"maxWidth\": 150,"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"weight\": \"bolder\","]
#[doc = "        \"wrap\": true"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FactSetTextConfig\""]
#[doc = "    },"]
#[doc = "    \"value\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"fontType\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"maxWidth\": 0,"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"weight\": \"default\","]
#[doc = "        \"wrap\": true"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FactSetTextConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactSetConfig {
    #[serde(default = "defaults::default_u64::<i64, 16>")]
    pub spacing: i64,
    #[serde(default = "defaults::fact_set_config_title")]
    pub title: FactSetTextConfig,
    #[serde(default = "defaults::fact_set_config_value")]
    pub value: FactSetTextConfig,
}
impl From<&FactSetConfig> for FactSetConfig {
    fn from(value: &FactSetConfig) -> Self {
        value.clone()
    }
}
impl FactSetConfig {
    pub fn builder() -> builder::FactSetConfig {
        Default::default()
    }
}
#[doc = "Parameters controlling the display of text in a fact set"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Parameters controlling the display of text in a fact set\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"description\": \"Color of font for fact set text\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"dark\","]
#[doc = "        \"light\","]
#[doc = "        \"accent\","]
#[doc = "        \"good\","]
#[doc = "        \"warning\","]
#[doc = "        \"attention\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fontType\": {"]
#[doc = "      \"description\": \"Font Type for fact set text\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"monospace\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isSubtle\": {"]
#[doc = "      \"description\": \"Indicates if fact set text should be subtle\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"maxWidth\": {"]
#[doc = "      \"description\": \"Maximum width of fact set text\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Size of font for fact set text\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"weight\": {"]
#[doc = "      \"description\": \"Weight of font for fact set text\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"lighter\","]
#[doc = "        \"bolder\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"wrap\": {"]
#[doc = "      \"description\": \"Indicates if fact set text should wrap\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FactSetTextConfig {
    #[doc = "Color of font for fact set text"]
    #[serde(default = "defaults::fact_set_text_config_color")]
    pub color: FactSetTextConfigColor,
    #[doc = "Font Type for fact set text"]
    #[serde(
        rename = "fontType",
        default = "defaults::fact_set_text_config_font_type"
    )]
    pub font_type: FactSetTextConfigFontType,
    #[doc = "Indicates if fact set text should be subtle"]
    #[serde(rename = "isSubtle", default)]
    pub is_subtle: bool,
    #[doc = "Maximum width of fact set text"]
    #[serde(rename = "maxWidth", default)]
    pub max_width: i64,
    #[doc = "Size of font for fact set text"]
    #[serde(default = "defaults::fact_set_text_config_size")]
    pub size: FactSetTextConfigSize,
    #[doc = "Weight of font for fact set text"]
    #[serde(default = "defaults::fact_set_text_config_weight")]
    pub weight: FactSetTextConfigWeight,
    #[doc = "Indicates if fact set text should wrap"]
    #[serde(default = "defaults::default_bool::<true>")]
    pub wrap: bool,
}
impl From<&FactSetTextConfig> for FactSetTextConfig {
    fn from(value: &FactSetTextConfig) -> Self {
        value.clone()
    }
}
impl FactSetTextConfig {
    pub fn builder() -> builder::FactSetTextConfig {
        Default::default()
    }
}
#[doc = "Color of font for fact set text"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Color of font for fact set text\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"dark\","]
#[doc = "    \"light\","]
#[doc = "    \"accent\","]
#[doc = "    \"good\","]
#[doc = "    \"warning\","]
#[doc = "    \"attention\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FactSetTextConfigColor {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "attention")]
    Attention,
}
impl From<&FactSetTextConfigColor> for FactSetTextConfigColor {
    fn from(value: &FactSetTextConfigColor) -> Self {
        value.clone()
    }
}
impl ToString for FactSetTextConfigColor {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Dark => "dark".to_string(),
            Self::Light => "light".to_string(),
            Self::Accent => "accent".to_string(),
            Self::Good => "good".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Attention => "attention".to_string(),
        }
    }
}
impl std::str::FromStr for FactSetTextConfigColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            "accent" => Ok(Self::Accent),
            "good" => Ok(Self::Good),
            "warning" => Ok(Self::Warning),
            "attention" => Ok(Self::Attention),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FactSetTextConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FactSetTextConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FactSetTextConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for FactSetTextConfigColor {
    fn default() -> Self {
        FactSetTextConfigColor::Default
    }
}
#[doc = "Font Type for fact set text"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Font Type for fact set text\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"monospace\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FactSetTextConfigFontType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "monospace")]
    Monospace,
}
impl From<&FactSetTextConfigFontType> for FactSetTextConfigFontType {
    fn from(value: &FactSetTextConfigFontType) -> Self {
        value.clone()
    }
}
impl ToString for FactSetTextConfigFontType {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Monospace => "monospace".to_string(),
        }
    }
}
impl std::str::FromStr for FactSetTextConfigFontType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "monospace" => Ok(Self::Monospace),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FactSetTextConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FactSetTextConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FactSetTextConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for FactSetTextConfigFontType {
    fn default() -> Self {
        FactSetTextConfigFontType::Default
    }
}
#[doc = "Size of font for fact set text"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Size of font for fact set text\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FactSetTextConfigSize {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
}
impl From<&FactSetTextConfigSize> for FactSetTextConfigSize {
    fn from(value: &FactSetTextConfigSize) -> Self {
        value.clone()
    }
}
impl ToString for FactSetTextConfigSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
        }
    }
}
impl std::str::FromStr for FactSetTextConfigSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FactSetTextConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FactSetTextConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FactSetTextConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for FactSetTextConfigSize {
    fn default() -> Self {
        FactSetTextConfigSize::Default
    }
}
#[doc = "Weight of font for fact set text"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Weight of font for fact set text\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"lighter\","]
#[doc = "    \"bolder\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum FactSetTextConfigWeight {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "bolder")]
    Bolder,
}
impl From<&FactSetTextConfigWeight> for FactSetTextConfigWeight {
    fn from(value: &FactSetTextConfigWeight) -> Self {
        value.clone()
    }
}
impl ToString for FactSetTextConfigWeight {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Lighter => "lighter".to_string(),
            Self::Bolder => "bolder".to_string(),
        }
    }
}
impl std::str::FromStr for FactSetTextConfigWeight {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "lighter" => Ok(Self::Lighter),
            "bolder" => Ok(Self::Bolder),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for FactSetTextConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FactSetTextConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FactSetTextConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for FactSetTextConfigWeight {
    fn default() -> Self {
        FactSetTextConfigWeight::Default
    }
}
#[doc = "FontColorConfig"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Color to use when displaying default text\","]
#[doc = "      \"default\": \"#ff252424\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"subtle\": {"]
#[doc = "      \"description\": \"Color to use when displaying subtle text\","]
#[doc = "      \"default\": \"#bf252424\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FontColorConfig {
    #[doc = "Color to use when displaying default text"]
    #[serde(default = "defaults::font_color_config_default")]
    pub default: Option<String>,
    #[doc = "Color to use when displaying subtle text"]
    #[serde(default = "defaults::font_color_config_subtle")]
    pub subtle: Option<String>,
}
impl From<&FontColorConfig> for FontColorConfig {
    fn from(value: &FontColorConfig) -> Self {
        value.clone()
    }
}
impl FontColorConfig {
    pub fn builder() -> builder::FontColorConfig {
        Default::default()
    }
}
#[doc = "Controls font size metrics for different text styles"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls font size metrics for different text styles\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Default font size\","]
#[doc = "      \"default\": 14,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extraLarge\": {"]
#[doc = "      \"description\": \"Extra large font size\","]
#[doc = "      \"default\": 24,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"large\": {"]
#[doc = "      \"description\": \"Large font size\","]
#[doc = "      \"default\": 18,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"description\": \"Medium font size\","]
#[doc = "      \"default\": 14,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"description\": \"Small font size\","]
#[doc = "      \"default\": 12,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FontSizesConfig {
    #[doc = "Default font size"]
    #[serde(default = "defaults::default_u64::<i64, 14>")]
    pub default: i64,
    #[doc = "Extra large font size"]
    #[serde(rename = "extraLarge", default = "defaults::default_u64::<i64, 24>")]
    pub extra_large: i64,
    #[doc = "Large font size"]
    #[serde(default = "defaults::default_u64::<i64, 18>")]
    pub large: i64,
    #[doc = "Medium font size"]
    #[serde(default = "defaults::default_u64::<i64, 14>")]
    pub medium: i64,
    #[doc = "Small font size"]
    #[serde(default = "defaults::default_u64::<i64, 12>")]
    pub small: i64,
}
impl From<&FontSizesConfig> for FontSizesConfig {
    fn from(value: &FontSizesConfig) -> Self {
        value.clone()
    }
}
impl FontSizesConfig {
    pub fn builder() -> builder::FontSizesConfig {
        Default::default()
    }
}
#[doc = "Controls font styles"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls font styles\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fontFamily\": {"]
#[doc = "      \"default\": \"sans-serif\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fontSizes\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": 14,"]
#[doc = "        \"extraLarge\": 24,"]
#[doc = "        \"large\": 18,"]
#[doc = "        \"medium\": 14,"]
#[doc = "        \"small\": 12"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontSizesConfig\""]
#[doc = "    },"]
#[doc = "    \"fontWeights\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"bolder\": 600,"]
#[doc = "        \"default\": 400,"]
#[doc = "        \"lighter\": 200"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontWeightsConfig\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FontTypeConfig {
    #[serde(
        rename = "fontFamily",
        default = "defaults::font_type_config_font_family"
    )]
    pub font_family: String,
    #[serde(
        rename = "fontSizes",
        default = "defaults::font_type_config_font_sizes"
    )]
    pub font_sizes: FontSizesConfig,
    #[serde(
        rename = "fontWeights",
        default = "defaults::font_type_config_font_weights"
    )]
    pub font_weights: FontWeightsConfig,
}
impl From<&FontTypeConfig> for FontTypeConfig {
    fn from(value: &FontTypeConfig) -> Self {
        value.clone()
    }
}
impl FontTypeConfig {
    pub fn builder() -> builder::FontTypeConfig {
        Default::default()
    }
}
#[doc = "Controls font styles"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls font styles\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Default font type\","]
#[doc = "      \"default\": {"]
#[doc = "        \"fontFamily\": \"sans-serif\","]
#[doc = "        \"fontSizes\": {"]
#[doc = "          \"default\": 14,"]
#[doc = "          \"extraLarge\": 24,"]
#[doc = "          \"large\": 18,"]
#[doc = "          \"medium\": 14,"]
#[doc = "          \"small\": 12"]
#[doc = "        },"]
#[doc = "        \"fontWeights\": {"]
#[doc = "          \"bolder\": 600,"]
#[doc = "          \"default\": 400,"]
#[doc = "          \"lighter\": 200"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontTypeConfig\","]
#[doc = "      \"unevaluatedProperties\": false"]
#[doc = "    },"]
#[doc = "    \"monospace\": {"]
#[doc = "      \"description\": \"Monospace font type\","]
#[doc = "      \"default\": {"]
#[doc = "        \"fontFamily\": \"monospace\","]
#[doc = "        \"fontSizes\": {"]
#[doc = "          \"default\": 14,"]
#[doc = "          \"extraLarge\": 24,"]
#[doc = "          \"large\": 18,"]
#[doc = "          \"medium\": 14,"]
#[doc = "          \"small\": 12"]
#[doc = "        },"]
#[doc = "        \"fontWeights\": {"]
#[doc = "          \"bolder\": 600,"]
#[doc = "          \"default\": 400,"]
#[doc = "          \"lighter\": 200"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontTypeConfig\","]
#[doc = "      \"unevaluatedProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FontTypesConfig {
    #[doc = "Default font type"]
    #[serde(default = "defaults::font_types_config_default")]
    pub default: FontTypeConfig,
    #[doc = "Monospace font type"]
    #[serde(default = "defaults::font_types_config_monospace")]
    pub monospace: FontTypeConfig,
}
impl From<&FontTypesConfig> for FontTypesConfig {
    fn from(value: &FontTypesConfig) -> Self {
        value.clone()
    }
}
impl FontTypesConfig {
    pub fn builder() -> builder::FontTypesConfig {
        Default::default()
    }
}
#[doc = "Controls font weight metrics"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls font weight metrics\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"bolder\": {"]
#[doc = "      \"default\": 600,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"default\": {"]
#[doc = "      \"default\": 400,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"lighter\": {"]
#[doc = "      \"default\": 200,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FontWeightsConfig {
    #[serde(default = "defaults::default_u64::<i64, 600>")]
    pub bolder: i64,
    #[serde(default = "defaults::default_u64::<i64, 400>")]
    pub default: i64,
    #[serde(default = "defaults::default_u64::<i64, 200>")]
    pub lighter: i64,
}
impl From<&FontWeightsConfig> for FontWeightsConfig {
    fn from(value: &FontWeightsConfig) -> Self {
        value.clone()
    }
}
impl FontWeightsConfig {
    pub fn builder() -> builder::FontWeightsConfig {
        Default::default()
    }
}
#[doc = "Controls various font colors"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls various font colors\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"accent\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#6264a7\","]
#[doc = "        \"subtle\": \"#8b8cc7\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"attention\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#c4314b\","]
#[doc = "        \"subtle\": \"#e5c4314b\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"dark\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#252424\","]
#[doc = "        \"subtle\": \"#bf252424\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"default\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#ff252424\","]
#[doc = "        \"subtle\": \"#bf252424\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"good\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#92c353\","]
#[doc = "        \"subtle\": \"#e592c353\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"light\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#ffffff\","]
#[doc = "        \"subtle\": \"#fff3f2f1\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    },"]
#[doc = "    \"warning\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": \"#f8d22a\","]
#[doc = "        \"subtle\": \"#e5f8d22a\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontColorConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForegroundColorsConfig {
    #[serde(default = "defaults::foreground_colors_config_accent")]
    pub accent: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_attention")]
    pub attention: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_dark")]
    pub dark: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_default")]
    pub default: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_good")]
    pub good: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_light")]
    pub light: FontColorConfig,
    #[serde(default = "defaults::foreground_colors_config_warning")]
    pub warning: FontColorConfig,
}
impl From<&ForegroundColorsConfig> for ForegroundColorsConfig {
    fn from(value: &ForegroundColorsConfig) -> Self {
        value.clone()
    }
}
impl ForegroundColorsConfig {
    pub fn builder() -> builder::ForegroundColorsConfig {
        Default::default()
    }
}
#[doc = "Contains host-configurable settings"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Contains host-configurable settings\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"$schema\": {"]
#[doc = "      \"description\": \"The Host Config schema.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"actions\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"actionAlignment\": \"stretch\","]
#[doc = "        \"actionsOrientation\": \"horizontal\","]
#[doc = "        \"buttonSpacing\": 10,"]
#[doc = "        \"iconPlacement\": \"aboveTitle\","]
#[doc = "        \"iconSize\": 30,"]
#[doc = "        \"maxActions\": 5,"]
#[doc = "        \"preExpandSingleShowCardAction\": false,"]
#[doc = "        \"showCard\": {"]
#[doc = "          \"actionMode\": \"inline\","]
#[doc = "          \"inlineTopMargin\": 16,"]
#[doc = "          \"style\": \"emphasis\""]
#[doc = "        },"]
#[doc = "        \"spacing\": \"default\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ActionsConfig\""]
#[doc = "    },"]
#[doc = "    \"adaptiveCard\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"allowCustomStyle\": true"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/AdaptiveCardConfig\""]
#[doc = "    },"]
#[doc = "    \"choiceSetInputValueSeparator\": {"]
#[doc = "      \"description\": \"Separator to use when displaying multiple values for a `ChoiceSet`\","]
#[doc = "      \"default\": \", \","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"containerStyles\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"accent\": {"]
#[doc = "          \"backgroundColor\": \"#C7DEF9\","]
#[doc = "          \"borderColor\": \"#62A8F7\","]
#[doc = "          \"borderThickness\": 1,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        },"]
#[doc = "        \"attention\": {"]
#[doc = "          \"backgroundColor\": \"#FFC5B2\","]
#[doc = "          \"borderColor\": \"#FF764C\","]
#[doc = "          \"borderThickness\": 1,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        },"]
#[doc = "        \"default\": {"]
#[doc = "          \"backgroundColor\": \"#ffffff\","]
#[doc = "          \"borderColor\": \"#CCCCCC\","]
#[doc = "          \"borderThickness\": 0,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        },"]
#[doc = "        \"emphasis\": {"]
#[doc = "          \"backgroundColor\": \"#fff9f8f7\","]
#[doc = "          \"borderColor\": \"#666666\","]
#[doc = "          \"borderThickness\": 1,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        },"]
#[doc = "        \"good\": {"]
#[doc = "          \"backgroundColor\": \"#CCFFCC\","]
#[doc = "          \"borderColor\": \"#69E569\","]
#[doc = "          \"borderThickness\": 1,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        },"]
#[doc = "        \"warning\": {"]
#[doc = "          \"backgroundColor\": \"#FFE2B2\","]
#[doc = "          \"borderColor\": \"#FFBC51\","]
#[doc = "          \"borderThickness\": 1,"]
#[doc = "          \"foregroundColors\": {"]
#[doc = "            \"accent\": {"]
#[doc = "              \"default\": \"#6264a7\","]
#[doc = "              \"subtle\": \"#8b8cc7\""]
#[doc = "            },"]
#[doc = "            \"attention\": {"]
#[doc = "              \"default\": \"#c4314b\","]
#[doc = "              \"subtle\": \"#e5c4314b\""]
#[doc = "            },"]
#[doc = "            \"dark\": {"]
#[doc = "              \"default\": \"#252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"default\": {"]
#[doc = "              \"default\": \"#ff252424\","]
#[doc = "              \"subtle\": \"#bf252424\""]
#[doc = "            },"]
#[doc = "            \"good\": {"]
#[doc = "              \"default\": \"#92c353\","]
#[doc = "              \"subtle\": \"#e592c353\""]
#[doc = "            },"]
#[doc = "            \"light\": {"]
#[doc = "              \"default\": \"#ffffff\","]
#[doc = "              \"subtle\": \"#fff3f2f1\""]
#[doc = "            },"]
#[doc = "            \"warning\": {"]
#[doc = "              \"default\": \"#f8d22a\","]
#[doc = "              \"subtle\": \"#e5f8d22a\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"tableGridLinesColor\": null"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ContainerStylesConfig\""]
#[doc = "    },"]
#[doc = "    \"factSet\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"spacing\": 16,"]
#[doc = "        \"title\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"fontType\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"maxWidth\": 150,"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"weight\": \"bolder\","]
#[doc = "          \"wrap\": true"]
#[doc = "        },"]
#[doc = "        \"value\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"fontType\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"maxWidth\": 0,"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"weight\": \"default\","]
#[doc = "          \"wrap\": true"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FactSetConfig\""]
#[doc = "    },"]
#[doc = "    \"fontFamily\": {"]
#[doc = "      \"description\": \"Font face to use when rendering text\","]
#[doc = "      \"deprecated\": true,"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fontSizes\": {"]
#[doc = "      \"deprecated\": true,"]
#[doc = "      \"$ref\": \"#/definitions/FontSizesConfig\""]
#[doc = "    },"]
#[doc = "    \"fontTypes\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": {"]
#[doc = "          \"fontFamily\": \"sans-serif\","]
#[doc = "          \"fontSizes\": {"]
#[doc = "            \"default\": 14,"]
#[doc = "            \"extraLarge\": 24,"]
#[doc = "            \"large\": 18,"]
#[doc = "            \"medium\": 14,"]
#[doc = "            \"small\": 12"]
#[doc = "          },"]
#[doc = "          \"fontWeights\": {"]
#[doc = "            \"bolder\": 600,"]
#[doc = "            \"default\": 400,"]
#[doc = "            \"lighter\": 200"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"monospace\": {"]
#[doc = "          \"fontFamily\": \"monospace\","]
#[doc = "          \"fontSizes\": {"]
#[doc = "            \"default\": 14,"]
#[doc = "            \"extraLarge\": 24,"]
#[doc = "            \"large\": 18,"]
#[doc = "            \"medium\": 14,"]
#[doc = "            \"small\": 12"]
#[doc = "          },"]
#[doc = "          \"fontWeights\": {"]
#[doc = "            \"bolder\": 600,"]
#[doc = "            \"default\": 400,"]
#[doc = "            \"lighter\": 200"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/FontTypesConfig\""]
#[doc = "    },"]
#[doc = "    \"fontWeights\": {"]
#[doc = "      \"deprecated\": true,"]
#[doc = "      \"$ref\": \"#/definitions/FontWeightsConfig\""]
#[doc = "    },"]
#[doc = "    \"imageBaseUrl\": {"]
#[doc = "      \"description\": \"Base URL to be used when loading resources\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"imageSet\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"imageSize\": \"medium\","]
#[doc = "        \"maxImageHeight\": 100"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ImageSetConfig\""]
#[doc = "    },"]
#[doc = "    \"imageSizes\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"large\": 100,"]
#[doc = "        \"medium\": 52,"]
#[doc = "        \"small\": 32"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ImageSizesConfig\""]
#[doc = "    },"]
#[doc = "    \"inputs\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"errorMessage\": {"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"spacing\": \"default\","]
#[doc = "          \"weight\": \"default\""]
#[doc = "        },"]
#[doc = "        \"label\": {"]
#[doc = "          \"inputSpacing\": \"default\","]
#[doc = "          \"optionalInputs\": {"]
#[doc = "            \"color\": \"default\","]
#[doc = "            \"isSubtle\": false,"]
#[doc = "            \"size\": \"default\","]
#[doc = "            \"weight\": \"default\""]
#[doc = "          },"]
#[doc = "          \"requiredInputs\": {"]
#[doc = "            \"color\": \"default\","]
#[doc = "            \"isSubtle\": false,"]
#[doc = "            \"size\": \"default\","]
#[doc = "            \"weight\": \"default\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/InputsConfig\""]
#[doc = "    },"]
#[doc = "    \"media\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"allowInlinePlayback\": true"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/MediaConfig\""]
#[doc = "    },"]
#[doc = "    \"separator\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"lineColor\": \"#EEEEEE\","]
#[doc = "        \"lineThickness\": 1"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/SeparatorConfig\""]
#[doc = "    },"]
#[doc = "    \"spacing\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"default\": 12,"]
#[doc = "        \"extraLarge\": 24,"]
#[doc = "        \"large\": 20,"]
#[doc = "        \"medium\": 16,"]
#[doc = "        \"padding\": 16,"]
#[doc = "        \"small\": 8"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/SpacingsConfig\""]
#[doc = "    },"]
#[doc = "    \"supportsInteractivity\": {"]
#[doc = "      \"description\": \"Control whether interactive `Action`s are allowed to be invoked\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"textBlock\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"headingLevel\": 2"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/TextBlockConfig\""]
#[doc = "    },"]
#[doc = "    \"textStyles\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"columnHeader\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"fontType\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"weight\": \"bolder\""]
#[doc = "        },"]
#[doc = "        \"heading\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"fontType\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"size\": \"large\","]
#[doc = "          \"weight\": \"bolder\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/TextStylesConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HostConfig {
    #[serde(default = "defaults::host_config_actions")]
    pub actions: ActionsConfig,
    #[serde(
        rename = "adaptiveCard",
        default = "defaults::host_config_adaptive_card"
    )]
    pub adaptive_card: AdaptiveCardConfig,
    #[doc = "Separator to use when displaying multiple values for a `ChoiceSet`"]
    #[serde(
        rename = "choiceSetInputValueSeparator",
        default = "defaults::host_config_choice_set_input_value_separator"
    )]
    pub choice_set_input_value_separator: String,
    #[serde(
        rename = "containerStyles",
        default = "defaults::host_config_container_styles"
    )]
    pub container_styles: ContainerStylesConfig,
    #[serde(rename = "factSet", default = "defaults::host_config_fact_set")]
    pub fact_set: FactSetConfig,
    #[doc = "Font face to use when rendering text"]
    #[serde(
        rename = "fontFamily",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub font_family: Option<String>,
    #[serde(rename = "fontSizes", default, skip_serializing_if = "Option::is_none")]
    pub font_sizes: Option<FontSizesConfig>,
    #[serde(rename = "fontTypes", default = "defaults::host_config_font_types")]
    pub font_types: FontTypesConfig,
    #[serde(
        rename = "fontWeights",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub font_weights: Option<FontWeightsConfig>,
    #[doc = "Base URL to be used when loading resources"]
    #[serde(
        rename = "imageBaseUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_base_url: Option<String>,
    #[serde(rename = "imageSet", default = "defaults::host_config_image_set")]
    pub image_set: ImageSetConfig,
    #[serde(rename = "imageSizes", default = "defaults::host_config_image_sizes")]
    pub image_sizes: ImageSizesConfig,
    #[serde(default = "defaults::host_config_inputs")]
    pub inputs: InputsConfig,
    #[serde(default = "defaults::host_config_media")]
    pub media: MediaConfig,
    #[doc = "The Host Config schema."]
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default = "defaults::host_config_separator")]
    pub separator: SeparatorConfig,
    #[serde(default = "defaults::host_config_spacing")]
    pub spacing: SpacingsConfig,
    #[doc = "Control whether interactive `Action`s are allowed to be invoked"]
    #[serde(rename = "supportsInteractivity", default)]
    pub supports_interactivity: bool,
    #[serde(rename = "textBlock", default = "defaults::host_config_text_block")]
    pub text_block: TextBlockConfig,
    #[serde(rename = "textStyles", default = "defaults::host_config_text_styles")]
    pub text_styles: TextStylesConfig,
}
impl From<&HostConfig> for HostConfig {
    fn from(value: &HostConfig) -> Self {
        value.clone()
    }
}
impl HostConfig {
    pub fn builder() -> builder::HostConfig {
        Default::default()
    }
}
#[doc = "Controls how `ImageSet`s are displayed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls how `ImageSet`s are displayed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"imageSize\": {"]
#[doc = "      \"description\": \"Controls individual image sizing\","]
#[doc = "      \"default\": \"medium\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"auto\","]
#[doc = "        \"stretch\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"maxImageHeight\": {"]
#[doc = "      \"description\": \"Constrain image height to this value\","]
#[doc = "      \"default\": 100,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSetConfig {
    #[doc = "Controls individual image sizing"]
    #[serde(
        rename = "imageSize",
        default = "defaults::image_set_config_image_size"
    )]
    pub image_size: ImageSetConfigImageSize,
    #[doc = "Constrain image height to this value"]
    #[serde(
        rename = "maxImageHeight",
        default = "defaults::default_u64::<i64, 100>"
    )]
    pub max_image_height: i64,
}
impl From<&ImageSetConfig> for ImageSetConfig {
    fn from(value: &ImageSetConfig) -> Self {
        value.clone()
    }
}
impl ImageSetConfig {
    pub fn builder() -> builder::ImageSetConfig {
        Default::default()
    }
}
#[doc = "Controls individual image sizing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls individual image sizing\","]
#[doc = "  \"default\": \"medium\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"auto\","]
#[doc = "    \"stretch\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ImageSetConfigImageSize {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "stretch")]
    Stretch,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}
impl From<&ImageSetConfigImageSize> for ImageSetConfigImageSize {
    fn from(value: &ImageSetConfigImageSize) -> Self {
        value.clone()
    }
}
impl ToString for ImageSetConfigImageSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Auto => "auto".to_string(),
            Self::Stretch => "stretch".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
        }
    }
}
impl std::str::FromStr for ImageSetConfigImageSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "auto" => Ok(Self::Auto),
            "stretch" => Ok(Self::Stretch),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ImageSetConfigImageSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ImageSetConfigImageSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ImageSetConfigImageSize {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ImageSetConfigImageSize {
    fn default() -> Self {
        ImageSetConfigImageSize::Medium
    }
}
#[doc = "Controls `Image` sizes"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls `Image` sizes\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"large\": {"]
#[doc = "      \"description\": \"Large image size value\","]
#[doc = "      \"default\": 100,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"description\": \"Medium image size value\","]
#[doc = "      \"default\": 52,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"description\": \"Small image size value\","]
#[doc = "      \"default\": 32,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSizesConfig {
    #[doc = "Large image size value"]
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub large: i64,
    #[doc = "Medium image size value"]
    #[serde(default = "defaults::default_u64::<i64, 52>")]
    pub medium: i64,
    #[doc = "Small image size value"]
    #[serde(default = "defaults::default_u64::<i64, 32>")]
    pub small: i64,
}
impl From<&ImageSizesConfig> for ImageSizesConfig {
    fn from(value: &ImageSizesConfig) -> Self {
        value.clone()
    }
}
impl ImageSizesConfig {
    pub fn builder() -> builder::ImageSizesConfig {
        Default::default()
    }
}
#[doc = "Controls display of input labels"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls display of input labels\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"description\": \"Color of the label\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"dark\","]
#[doc = "        \"light\","]
#[doc = "        \"accent\","]
#[doc = "        \"good\","]
#[doc = "        \"warning\","]
#[doc = "        \"attention\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isSubtle\": {"]
#[doc = "      \"description\": \"Whether the label should be displayed using a lighter weight font\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Size of the label text\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"small\","]
#[doc = "        \"default\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"suffix\": {"]
#[doc = "      \"description\": \"Suffix that should be appended to labels of this type\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"weight\": {"]
#[doc = "      \"description\": \"Font weight that should be used for this type of label\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"lighter\","]
#[doc = "        \"default\","]
#[doc = "        \"bolder\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"version\": \"1.3\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputLabelConfig {
    #[doc = "Color of the label"]
    #[serde(default = "defaults::input_label_config_color")]
    pub color: InputLabelConfigColor,
    #[doc = "Whether the label should be displayed using a lighter weight font"]
    #[serde(rename = "isSubtle", default)]
    pub is_subtle: bool,
    #[doc = "Size of the label text"]
    #[serde(default = "defaults::input_label_config_size")]
    pub size: InputLabelConfigSize,
    #[doc = "Suffix that should be appended to labels of this type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[doc = "Font weight that should be used for this type of label"]
    #[serde(default = "defaults::input_label_config_weight")]
    pub weight: InputLabelConfigWeight,
}
impl From<&InputLabelConfig> for InputLabelConfig {
    fn from(value: &InputLabelConfig) -> Self {
        value.clone()
    }
}
impl InputLabelConfig {
    pub fn builder() -> builder::InputLabelConfig {
        Default::default()
    }
}
#[doc = "Color of the label"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Color of the label\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"dark\","]
#[doc = "    \"light\","]
#[doc = "    \"accent\","]
#[doc = "    \"good\","]
#[doc = "    \"warning\","]
#[doc = "    \"attention\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InputLabelConfigColor {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "attention")]
    Attention,
}
impl From<&InputLabelConfigColor> for InputLabelConfigColor {
    fn from(value: &InputLabelConfigColor) -> Self {
        value.clone()
    }
}
impl ToString for InputLabelConfigColor {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Dark => "dark".to_string(),
            Self::Light => "light".to_string(),
            Self::Accent => "accent".to_string(),
            Self::Good => "good".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Attention => "attention".to_string(),
        }
    }
}
impl std::str::FromStr for InputLabelConfigColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            "accent" => Ok(Self::Accent),
            "good" => Ok(Self::Good),
            "warning" => Ok(Self::Warning),
            "attention" => Ok(Self::Attention),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InputLabelConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InputLabelConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InputLabelConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for InputLabelConfigColor {
    fn default() -> Self {
        InputLabelConfigColor::Default
    }
}
#[doc = "Size of the label text"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Size of the label text\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"small\","]
#[doc = "    \"default\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InputLabelConfigSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
}
impl From<&InputLabelConfigSize> for InputLabelConfigSize {
    fn from(value: &InputLabelConfigSize) -> Self {
        value.clone()
    }
}
impl ToString for InputLabelConfigSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Small => "small".to_string(),
            Self::Default => "default".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
        }
    }
}
impl std::str::FromStr for InputLabelConfigSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "small" => Ok(Self::Small),
            "default" => Ok(Self::Default),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InputLabelConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InputLabelConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InputLabelConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for InputLabelConfigSize {
    fn default() -> Self {
        InputLabelConfigSize::Default
    }
}
#[doc = "Font weight that should be used for this type of label"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Font weight that should be used for this type of label\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"lighter\","]
#[doc = "    \"default\","]
#[doc = "    \"bolder\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InputLabelConfigWeight {
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "bolder")]
    Bolder,
}
impl From<&InputLabelConfigWeight> for InputLabelConfigWeight {
    fn from(value: &InputLabelConfigWeight) -> Self {
        value.clone()
    }
}
impl ToString for InputLabelConfigWeight {
    fn to_string(&self) -> String {
        match *self {
            Self::Lighter => "lighter".to_string(),
            Self::Default => "default".to_string(),
            Self::Bolder => "bolder".to_string(),
        }
    }
}
impl std::str::FromStr for InputLabelConfigWeight {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "lighter" => Ok(Self::Lighter),
            "default" => Ok(Self::Default),
            "bolder" => Ok(Self::Bolder),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InputLabelConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InputLabelConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InputLabelConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for InputLabelConfigWeight {
    fn default() -> Self {
        InputLabelConfigWeight::Default
    }
}
#[doc = "Controls display and behavior of Input types"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls display and behavior of Input types\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"errorMessage\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"spacing\": \"default\","]
#[doc = "        \"weight\": \"default\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/ErrorMessageConfig\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"inputSpacing\": \"default\","]
#[doc = "        \"optionalInputs\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"weight\": \"default\""]
#[doc = "        },"]
#[doc = "        \"requiredInputs\": {"]
#[doc = "          \"color\": \"default\","]
#[doc = "          \"isSubtle\": false,"]
#[doc = "          \"size\": \"default\","]
#[doc = "          \"weight\": \"default\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/LabelConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"version\": \"1.3\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InputsConfig {
    #[serde(
        rename = "errorMessage",
        default = "defaults::inputs_config_error_message"
    )]
    pub error_message: ErrorMessageConfig,
    #[serde(default = "defaults::inputs_config_label")]
    pub label: LabelConfig,
}
impl From<&InputsConfig> for InputsConfig {
    fn from(value: &InputsConfig) -> Self {
        value.clone()
    }
}
impl InputsConfig {
    pub fn builder() -> builder::InputsConfig {
        Default::default()
    }
}
#[doc = "Controls display of input labels"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls display of input labels\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"inputSpacing\": {"]
#[doc = "      \"description\": \"Amount of spacing to be used between label and input\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"none\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\","]
#[doc = "        \"padding\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"optionalInputs\": {"]
#[doc = "      \"description\": \"Label config for optional Inputs\","]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"weight\": \"default\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/InputLabelConfig\""]
#[doc = "    },"]
#[doc = "    \"requiredInputs\": {"]
#[doc = "      \"description\": \"Label config for required Inputs\","]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"weight\": \"default\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/InputLabelConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"version\": \"1.3\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LabelConfig {
    #[doc = "Amount of spacing to be used between label and input"]
    #[serde(
        rename = "inputSpacing",
        default = "defaults::label_config_input_spacing"
    )]
    pub input_spacing: LabelConfigInputSpacing,
    #[doc = "Label config for optional Inputs"]
    #[serde(
        rename = "optionalInputs",
        default = "defaults::label_config_optional_inputs"
    )]
    pub optional_inputs: InputLabelConfig,
    #[doc = "Label config for required Inputs"]
    #[serde(
        rename = "requiredInputs",
        default = "defaults::label_config_required_inputs"
    )]
    pub required_inputs: InputLabelConfig,
}
impl From<&LabelConfig> for LabelConfig {
    fn from(value: &LabelConfig) -> Self {
        value.clone()
    }
}
impl LabelConfig {
    pub fn builder() -> builder::LabelConfig {
        Default::default()
    }
}
#[doc = "Amount of spacing to be used between label and input"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Amount of spacing to be used between label and input\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"none\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\","]
#[doc = "    \"padding\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LabelConfigInputSpacing {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
    #[serde(rename = "padding")]
    Padding,
}
impl From<&LabelConfigInputSpacing> for LabelConfigInputSpacing {
    fn from(value: &LabelConfigInputSpacing) -> Self {
        value.clone()
    }
}
impl ToString for LabelConfigInputSpacing {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::None => "none".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
            Self::Padding => "padding".to_string(),
        }
    }
}
impl std::str::FromStr for LabelConfigInputSpacing {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "none" => Ok(Self::None),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            "padding" => Ok(Self::Padding),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LabelConfigInputSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LabelConfigInputSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LabelConfigInputSpacing {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for LabelConfigInputSpacing {
    fn default() -> Self {
        LabelConfigInputSpacing::Default
    }
}
#[doc = "Controls the display and behavior of `Media` elements"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls the display and behavior of `Media` elements\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowInlinePlayback\": {"]
#[doc = "      \"description\": \"Whether to display media inline or invoke externally\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"defaultPoster\": {"]
#[doc = "      \"description\": \"URI to image to display when play button hasn't been invoked\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    },"]
#[doc = "    \"playButton\": {"]
#[doc = "      \"description\": \"Image to display as play button\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"version\": \"1.1\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MediaConfig {
    #[doc = "Whether to display media inline or invoke externally"]
    #[serde(
        rename = "allowInlinePlayback",
        default = "defaults::default_bool::<true>"
    )]
    pub allow_inline_playback: bool,
    #[doc = "URI to image to display when play button hasn't been invoked"]
    #[serde(
        rename = "defaultPoster",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_poster: Option<String>,
    #[doc = "Image to display as play button"]
    #[serde(
        rename = "playButton",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_button: Option<String>,
}
impl From<&MediaConfig> for MediaConfig {
    fn from(value: &MediaConfig) -> Self {
        value.clone()
    }
}
impl MediaConfig {
    pub fn builder() -> builder::MediaConfig {
        Default::default()
    }
}
#[doc = "Controls how separators are displayed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls how separators are displayed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"lineColor\": {"]
#[doc = "      \"description\": \"Color to use when drawing separator line\","]
#[doc = "      \"default\": \"#EEEEEE\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lineThickness\": {"]
#[doc = "      \"description\": \"Thickness of separator line\","]
#[doc = "      \"default\": 1,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SeparatorConfig {
    #[doc = "Color to use when drawing separator line"]
    #[serde(
        rename = "lineColor",
        default = "defaults::separator_config_line_color"
    )]
    pub line_color: String,
    #[doc = "Thickness of separator line"]
    #[serde(rename = "lineThickness", default = "defaults::default_u64::<i64, 1>")]
    pub line_thickness: i64,
}
impl From<&SeparatorConfig> for SeparatorConfig {
    fn from(value: &SeparatorConfig) -> Self {
        value.clone()
    }
}
impl SeparatorConfig {
    pub fn builder() -> builder::SeparatorConfig {
        Default::default()
    }
}
#[doc = "Controls behavior and styling of `Action.ShowCard`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls behavior and styling of `Action.ShowCard`\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"actionMode\": {"]
#[doc = "      \"description\": \"Controls how the card is displayed. Note: Popup show cards are not recommended for cards with input validation, and may be deprecated in the future.\","]
#[doc = "      \"default\": \"inline\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"inline\","]
#[doc = "        \"popup\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"inlineTopMargin\": {"]
#[doc = "      \"description\": \"Amount of margin to use when displaying the card\","]
#[doc = "      \"default\": 16,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"style\": {"]
#[doc = "      \"default\": \"emphasis\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"emphasis\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ShowCardConfig {
    #[doc = "Controls how the card is displayed. Note: Popup show cards are not recommended for cards with input validation, and may be deprecated in the future."]
    #[serde(
        rename = "actionMode",
        default = "defaults::show_card_config_action_mode"
    )]
    pub action_mode: ShowCardConfigActionMode,
    #[doc = "Amount of margin to use when displaying the card"]
    #[serde(
        rename = "inlineTopMargin",
        default = "defaults::default_u64::<i64, 16>"
    )]
    pub inline_top_margin: i64,
    #[serde(default = "defaults::show_card_config_style")]
    pub style: ShowCardConfigStyle,
}
impl From<&ShowCardConfig> for ShowCardConfig {
    fn from(value: &ShowCardConfig) -> Self {
        value.clone()
    }
}
impl ShowCardConfig {
    pub fn builder() -> builder::ShowCardConfig {
        Default::default()
    }
}
#[doc = "Controls how the card is displayed. Note: Popup show cards are not recommended for cards with input validation, and may be deprecated in the future."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls how the card is displayed. Note: Popup show cards are not recommended for cards with input validation, and may be deprecated in the future.\","]
#[doc = "  \"default\": \"inline\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"inline\","]
#[doc = "    \"popup\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ShowCardConfigActionMode {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "popup")]
    Popup,
}
impl From<&ShowCardConfigActionMode> for ShowCardConfigActionMode {
    fn from(value: &ShowCardConfigActionMode) -> Self {
        value.clone()
    }
}
impl ToString for ShowCardConfigActionMode {
    fn to_string(&self) -> String {
        match *self {
            Self::Inline => "inline".to_string(),
            Self::Popup => "popup".to_string(),
        }
    }
}
impl std::str::FromStr for ShowCardConfigActionMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "inline" => Ok(Self::Inline),
            "popup" => Ok(Self::Popup),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ShowCardConfigActionMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ShowCardConfigActionMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ShowCardConfigActionMode {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ShowCardConfigActionMode {
    fn default() -> Self {
        ShowCardConfigActionMode::Inline
    }
}
#[doc = "ShowCardConfigStyle"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"default\": \"emphasis\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"emphasis\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ShowCardConfigStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "emphasis")]
    Emphasis,
}
impl From<&ShowCardConfigStyle> for ShowCardConfigStyle {
    fn from(value: &ShowCardConfigStyle) -> Self {
        value.clone()
    }
}
impl ToString for ShowCardConfigStyle {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Emphasis => "emphasis".to_string(),
        }
    }
}
impl std::str::FromStr for ShowCardConfigStyle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "emphasis" => Ok(Self::Emphasis),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ShowCardConfigStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ShowCardConfigStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ShowCardConfigStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for ShowCardConfigStyle {
    fn default() -> Self {
        ShowCardConfigStyle::Emphasis
    }
}
#[doc = "Controls how elements are to be laid out"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Controls how elements are to be laid out\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"default\": {"]
#[doc = "      \"description\": \"Default spacing value\","]
#[doc = "      \"default\": 12,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"extraLarge\": {"]
#[doc = "      \"description\": \"Extra large spacing value\","]
#[doc = "      \"default\": 24,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"large\": {"]
#[doc = "      \"description\": \"Large spacing value\","]
#[doc = "      \"default\": 20,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"medium\": {"]
#[doc = "      \"description\": \"Medium spacing value\","]
#[doc = "      \"default\": 16,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"padding\": {"]
#[doc = "      \"description\": \"Padding value\","]
#[doc = "      \"default\": 16,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"small\": {"]
#[doc = "      \"description\": \"Small spacing value\","]
#[doc = "      \"default\": 8,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpacingsConfig {
    #[doc = "Default spacing value"]
    #[serde(default = "defaults::default_u64::<i64, 12>")]
    pub default: i64,
    #[doc = "Extra large spacing value"]
    #[serde(rename = "extraLarge", default = "defaults::default_u64::<i64, 24>")]
    pub extra_large: i64,
    #[doc = "Large spacing value"]
    #[serde(default = "defaults::default_u64::<i64, 20>")]
    pub large: i64,
    #[doc = "Medium spacing value"]
    #[serde(default = "defaults::default_u64::<i64, 16>")]
    pub medium: i64,
    #[doc = "Padding value"]
    #[serde(default = "defaults::default_u64::<i64, 16>")]
    pub padding: i64,
    #[doc = "Small spacing value"]
    #[serde(default = "defaults::default_u64::<i64, 8>")]
    pub small: i64,
}
impl From<&SpacingsConfig> for SpacingsConfig {
    fn from(value: &SpacingsConfig) -> Self {
        value.clone()
    }
}
impl SpacingsConfig {
    pub fn builder() -> builder::SpacingsConfig {
        Default::default()
    }
}
#[doc = "Configuration settings for TextBlocks"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Configuration settings for TextBlocks\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"headingLevel\": {"]
#[doc = "      \"description\": \"When displaying a `TextBlock` element with the `heading` style, this is the heading level exposed to accessibility tools.\","]
#[doc = "      \"default\": 2,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextBlockConfig {
    #[doc = "When displaying a `TextBlock` element with the `heading` style, this is the heading level exposed to accessibility tools."]
    #[serde(rename = "headingLevel", default = "defaults::default_u64::<i64, 2>")]
    pub heading_level: i64,
}
impl From<&TextBlockConfig> for TextBlockConfig {
    fn from(value: &TextBlockConfig) -> Self {
        value.clone()
    }
}
impl TextBlockConfig {
    pub fn builder() -> builder::TextBlockConfig {
        Default::default()
    }
}
#[doc = "Sets default properties for text of a given style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sets default properties for text of a given style\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"color\": {"]
#[doc = "      \"description\": \"Default font color for text of this style\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"dark\","]
#[doc = "        \"light\","]
#[doc = "        \"accent\","]
#[doc = "        \"good\","]
#[doc = "        \"warning\","]
#[doc = "        \"attention\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"fontType\": {"]
#[doc = "      \"description\": \"Default font type for text of this style\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"monospace\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"isSubtle\": {"]
#[doc = "      \"description\": \"Whether text of this style should be subtle by default\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"description\": \"Default font size for text of this style\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"small\","]
#[doc = "        \"medium\","]
#[doc = "        \"large\","]
#[doc = "        \"extraLarge\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"weight\": {"]
#[doc = "      \"description\": \"Default font weight for text of this style\","]
#[doc = "      \"default\": \"default\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"default\","]
#[doc = "        \"lighter\","]
#[doc = "        \"bolder\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextStyleConfig {
    #[doc = "Default font color for text of this style"]
    #[serde(default = "defaults::text_style_config_color")]
    pub color: TextStyleConfigColor,
    #[doc = "Default font type for text of this style"]
    #[serde(rename = "fontType", default = "defaults::text_style_config_font_type")]
    pub font_type: TextStyleConfigFontType,
    #[doc = "Whether text of this style should be subtle by default"]
    #[serde(rename = "isSubtle", default)]
    pub is_subtle: bool,
    #[doc = "Default font size for text of this style"]
    #[serde(default = "defaults::text_style_config_size")]
    pub size: TextStyleConfigSize,
    #[doc = "Default font weight for text of this style"]
    #[serde(default = "defaults::text_style_config_weight")]
    pub weight: TextStyleConfigWeight,
}
impl From<&TextStyleConfig> for TextStyleConfig {
    fn from(value: &TextStyleConfig) -> Self {
        value.clone()
    }
}
impl TextStyleConfig {
    pub fn builder() -> builder::TextStyleConfig {
        Default::default()
    }
}
#[doc = "Default font color for text of this style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Default font color for text of this style\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"dark\","]
#[doc = "    \"light\","]
#[doc = "    \"accent\","]
#[doc = "    \"good\","]
#[doc = "    \"warning\","]
#[doc = "    \"attention\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TextStyleConfigColor {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "dark")]
    Dark,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "accent")]
    Accent,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "attention")]
    Attention,
}
impl From<&TextStyleConfigColor> for TextStyleConfigColor {
    fn from(value: &TextStyleConfigColor) -> Self {
        value.clone()
    }
}
impl ToString for TextStyleConfigColor {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Dark => "dark".to_string(),
            Self::Light => "light".to_string(),
            Self::Accent => "accent".to_string(),
            Self::Good => "good".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Attention => "attention".to_string(),
        }
    }
}
impl std::str::FromStr for TextStyleConfigColor {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "dark" => Ok(Self::Dark),
            "light" => Ok(Self::Light),
            "accent" => Ok(Self::Accent),
            "good" => Ok(Self::Good),
            "warning" => Ok(Self::Warning),
            "attention" => Ok(Self::Attention),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TextStyleConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextStyleConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextStyleConfigColor {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for TextStyleConfigColor {
    fn default() -> Self {
        TextStyleConfigColor::Default
    }
}
#[doc = "Default font type for text of this style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Default font type for text of this style\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"monospace\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TextStyleConfigFontType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "monospace")]
    Monospace,
}
impl From<&TextStyleConfigFontType> for TextStyleConfigFontType {
    fn from(value: &TextStyleConfigFontType) -> Self {
        value.clone()
    }
}
impl ToString for TextStyleConfigFontType {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Monospace => "monospace".to_string(),
        }
    }
}
impl std::str::FromStr for TextStyleConfigFontType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "monospace" => Ok(Self::Monospace),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TextStyleConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextStyleConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextStyleConfigFontType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for TextStyleConfigFontType {
    fn default() -> Self {
        TextStyleConfigFontType::Default
    }
}
#[doc = "Default font size for text of this style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Default font size for text of this style\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"small\","]
#[doc = "    \"medium\","]
#[doc = "    \"large\","]
#[doc = "    \"extraLarge\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TextStyleConfigSize {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extraLarge")]
    ExtraLarge,
}
impl From<&TextStyleConfigSize> for TextStyleConfigSize {
    fn from(value: &TextStyleConfigSize) -> Self {
        value.clone()
    }
}
impl ToString for TextStyleConfigSize {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Small => "small".to_string(),
            Self::Medium => "medium".to_string(),
            Self::Large => "large".to_string(),
            Self::ExtraLarge => "extraLarge".to_string(),
        }
    }
}
impl std::str::FromStr for TextStyleConfigSize {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "small" => Ok(Self::Small),
            "medium" => Ok(Self::Medium),
            "large" => Ok(Self::Large),
            "extraLarge" => Ok(Self::ExtraLarge),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TextStyleConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextStyleConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextStyleConfigSize {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for TextStyleConfigSize {
    fn default() -> Self {
        TextStyleConfigSize::Default
    }
}
#[doc = "Default font weight for text of this style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Default font weight for text of this style\","]
#[doc = "  \"default\": \"default\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"default\","]
#[doc = "    \"lighter\","]
#[doc = "    \"bolder\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TextStyleConfigWeight {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "bolder")]
    Bolder,
}
impl From<&TextStyleConfigWeight> for TextStyleConfigWeight {
    fn from(value: &TextStyleConfigWeight) -> Self {
        value.clone()
    }
}
impl ToString for TextStyleConfigWeight {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Lighter => "lighter".to_string(),
            Self::Bolder => "bolder".to_string(),
        }
    }
}
impl std::str::FromStr for TextStyleConfigWeight {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "default" => Ok(Self::Default),
            "lighter" => Ok(Self::Lighter),
            "bolder" => Ok(Self::Bolder),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TextStyleConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TextStyleConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TextStyleConfigWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl Default for TextStyleConfigWeight {
    fn default() -> Self {
        TextStyleConfigWeight::Default
    }
}
#[doc = "Sets default properties for text of a given style"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Sets default properties for text of a given style\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"columnHeader\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"fontType\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"size\": \"default\","]
#[doc = "        \"weight\": \"bolder\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/TextStyleConfig\""]
#[doc = "    },"]
#[doc = "    \"heading\": {"]
#[doc = "      \"default\": {"]
#[doc = "        \"color\": \"default\","]
#[doc = "        \"fontType\": \"default\","]
#[doc = "        \"isSubtle\": false,"]
#[doc = "        \"size\": \"large\","]
#[doc = "        \"weight\": \"bolder\""]
#[doc = "      },"]
#[doc = "      \"$ref\": \"#/definitions/TextStyleConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextStylesConfig {
    #[serde(
        rename = "columnHeader",
        default = "defaults::text_styles_config_column_header"
    )]
    pub column_header: TextStyleConfig,
    #[serde(default = "defaults::text_styles_config_heading")]
    pub heading: TextStyleConfig,
}
impl From<&TextStylesConfig> for TextStylesConfig {
    fn from(value: &TextStylesConfig) -> Self {
        value.clone()
    }
}
impl TextStylesConfig {
    pub fn builder() -> builder::TextStylesConfig {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ActionsConfig {
        action_alignment: Result<super::ActionsConfigActionAlignment, String>,
        actions_orientation: Result<super::ActionsConfigActionsOrientation, String>,
        button_spacing: Result<i64, String>,
        icon_placement: Result<super::ActionsConfigIconPlacement, String>,
        icon_size: Result<i64, String>,
        max_actions: Result<i64, String>,
        pre_expand_single_show_card_action: Result<bool, String>,
        show_card: Result<super::ShowCardConfig, String>,
        spacing: Result<super::ActionsConfigSpacing, String>,
    }
    impl Default for ActionsConfig {
        fn default() -> Self {
            Self {
                action_alignment: Ok(super::defaults::actions_config_action_alignment()),
                actions_orientation: Ok(super::defaults::actions_config_actions_orientation()),
                button_spacing: Ok(super::defaults::default_u64::<i64, 10>()),
                icon_placement: Ok(super::defaults::actions_config_icon_placement()),
                icon_size: Ok(super::defaults::default_u64::<i64, 30>()),
                max_actions: Ok(super::defaults::default_u64::<i64, 5>()),
                pre_expand_single_show_card_action: Ok(Default::default()),
                show_card: Ok(super::defaults::actions_config_show_card()),
                spacing: Ok(super::defaults::actions_config_spacing()),
            }
        }
    }
    impl ActionsConfig {
        pub fn action_alignment<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ActionsConfigActionAlignment>,
            T::Error: std::fmt::Display,
        {
            self.action_alignment = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for action_alignment: {}",
                    e
                )
            });
            self
        }
        pub fn actions_orientation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ActionsConfigActionsOrientation>,
            T::Error: std::fmt::Display,
        {
            self.actions_orientation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for actions_orientation: {}",
                    e
                )
            });
            self
        }
        pub fn button_spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.button_spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for button_spacing: {}", e));
            self
        }
        pub fn icon_placement<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ActionsConfigIconPlacement>,
            T::Error: std::fmt::Display,
        {
            self.icon_placement = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_placement: {}", e));
            self
        }
        pub fn icon_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.icon_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_size: {}", e));
            self
        }
        pub fn max_actions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max_actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_actions: {}", e));
            self
        }
        pub fn pre_expand_single_show_card_action<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.pre_expand_single_show_card_action = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for pre_expand_single_show_card_action: {}",
                    e
                )
            });
            self
        }
        pub fn show_card<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShowCardConfig>,
            T::Error: std::fmt::Display,
        {
            self.show_card = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show_card: {}", e));
            self
        }
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ActionsConfigSpacing>,
            T::Error: std::fmt::Display,
        {
            self.spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spacing: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ActionsConfig> for super::ActionsConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ActionsConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_alignment: value.action_alignment?,
                actions_orientation: value.actions_orientation?,
                button_spacing: value.button_spacing?,
                icon_placement: value.icon_placement?,
                icon_size: value.icon_size?,
                max_actions: value.max_actions?,
                pre_expand_single_show_card_action: value.pre_expand_single_show_card_action?,
                show_card: value.show_card?,
                spacing: value.spacing?,
            })
        }
    }
    impl From<super::ActionsConfig> for ActionsConfig {
        fn from(value: super::ActionsConfig) -> Self {
            Self {
                action_alignment: Ok(value.action_alignment),
                actions_orientation: Ok(value.actions_orientation),
                button_spacing: Ok(value.button_spacing),
                icon_placement: Ok(value.icon_placement),
                icon_size: Ok(value.icon_size),
                max_actions: Ok(value.max_actions),
                pre_expand_single_show_card_action: Ok(value.pre_expand_single_show_card_action),
                show_card: Ok(value.show_card),
                spacing: Ok(value.spacing),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AdaptiveCardConfig {
        allow_custom_style: Result<bool, String>,
    }
    impl Default for AdaptiveCardConfig {
        fn default() -> Self {
            Self {
                allow_custom_style: Ok(super::defaults::default_bool::<true>()),
            }
        }
    }
    impl AdaptiveCardConfig {
        pub fn allow_custom_style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.allow_custom_style = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allow_custom_style: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<AdaptiveCardConfig> for super::AdaptiveCardConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: AdaptiveCardConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                allow_custom_style: value.allow_custom_style?,
            })
        }
    }
    impl From<super::AdaptiveCardConfig> for AdaptiveCardConfig {
        fn from(value: super::AdaptiveCardConfig) -> Self {
            Self {
                allow_custom_style: Ok(value.allow_custom_style),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerStyleConfig {
        background_color: Result<Option<String>, String>,
        border_color: Result<Option<String>, String>,
        border_thickness: Result<Option<i64>, String>,
        foreground_colors: Result<super::ForegroundColorsConfig, String>,
        table_grid_lines_color: Result<Option<String>, String>,
        table_grid_lines_thickness: Result<Option<i64>, String>,
    }
    impl Default for ContainerStyleConfig {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                border_color: Ok(Default::default()),
                border_thickness: Ok(Default::default()),
                foreground_colors: Ok(super::defaults::container_style_config_foreground_colors()),
                table_grid_lines_color: Ok(Default::default()),
                table_grid_lines_thickness: Ok(Default::default()),
            }
        }
    }
    impl ContainerStyleConfig {
        pub fn background_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.background_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for background_color: {}",
                    e
                )
            });
            self
        }
        pub fn border_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.border_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border_color: {}", e));
            self
        }
        pub fn border_thickness<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.border_thickness = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for border_thickness: {}",
                    e
                )
            });
            self
        }
        pub fn foreground_colors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ForegroundColorsConfig>,
            T::Error: std::fmt::Display,
        {
            self.foreground_colors = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for foreground_colors: {}",
                    e
                )
            });
            self
        }
        pub fn table_grid_lines_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.table_grid_lines_color = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for table_grid_lines_color: {}",
                    e
                )
            });
            self
        }
        pub fn table_grid_lines_thickness<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.table_grid_lines_thickness = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for table_grid_lines_thickness: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<ContainerStyleConfig> for super::ContainerStyleConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ContainerStyleConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                border_color: value.border_color?,
                border_thickness: value.border_thickness?,
                foreground_colors: value.foreground_colors?,
                table_grid_lines_color: value.table_grid_lines_color?,
                table_grid_lines_thickness: value.table_grid_lines_thickness?,
            })
        }
    }
    impl From<super::ContainerStyleConfig> for ContainerStyleConfig {
        fn from(value: super::ContainerStyleConfig) -> Self {
            Self {
                background_color: Ok(value.background_color),
                border_color: Ok(value.border_color),
                border_thickness: Ok(value.border_thickness),
                foreground_colors: Ok(value.foreground_colors),
                table_grid_lines_color: Ok(value.table_grid_lines_color),
                table_grid_lines_thickness: Ok(value.table_grid_lines_thickness),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContainerStylesConfig {
        accent: Result<super::ContainerStyleConfig, String>,
        attention: Result<super::ContainerStyleConfig, String>,
        default: Result<super::ContainerStyleConfig, String>,
        emphasis: Result<super::ContainerStyleConfig, String>,
        good: Result<super::ContainerStyleConfig, String>,
        warning: Result<super::ContainerStyleConfig, String>,
    }
    impl Default for ContainerStylesConfig {
        fn default() -> Self {
            Self {
                accent: Ok(super::defaults::container_styles_config_accent()),
                attention: Ok(super::defaults::container_styles_config_attention()),
                default: Ok(super::defaults::container_styles_config_default()),
                emphasis: Ok(super::defaults::container_styles_config_emphasis()),
                good: Ok(super::defaults::container_styles_config_good()),
                warning: Ok(super::defaults::container_styles_config_warning()),
            }
        }
    }
    impl ContainerStylesConfig {
        pub fn accent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accent: {}", e));
            self
        }
        pub fn attention<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.attention = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attention: {}", e));
            self
        }
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn emphasis<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.emphasis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for emphasis: {}", e));
            self
        }
        pub fn good<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.good = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for good: {}", e));
            self
        }
        pub fn warning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.warning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ContainerStylesConfig> for super::ContainerStylesConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ContainerStylesConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accent: value.accent?,
                attention: value.attention?,
                default: value.default?,
                emphasis: value.emphasis?,
                good: value.good?,
                warning: value.warning?,
            })
        }
    }
    impl From<super::ContainerStylesConfig> for ContainerStylesConfig {
        fn from(value: super::ContainerStylesConfig) -> Self {
            Self {
                accent: Ok(value.accent),
                attention: Ok(value.attention),
                default: Ok(value.default),
                emphasis: Ok(value.emphasis),
                good: Ok(value.good),
                warning: Ok(value.warning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ErrorMessageConfig {
        size: Result<super::ErrorMessageConfigSize, String>,
        spacing: Result<super::ErrorMessageConfigSpacing, String>,
        weight: Result<super::ErrorMessageConfigWeight, String>,
    }
    impl Default for ErrorMessageConfig {
        fn default() -> Self {
            Self {
                size: Ok(super::defaults::error_message_config_size()),
                spacing: Ok(super::defaults::error_message_config_spacing()),
                weight: Ok(super::defaults::error_message_config_weight()),
            }
        }
    }
    impl ErrorMessageConfig {
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ErrorMessageConfigSize>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ErrorMessageConfigSpacing>,
            T::Error: std::fmt::Display,
        {
            self.spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spacing: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ErrorMessageConfigWeight>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ErrorMessageConfig> for super::ErrorMessageConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ErrorMessageConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                size: value.size?,
                spacing: value.spacing?,
                weight: value.weight?,
            })
        }
    }
    impl From<super::ErrorMessageConfig> for ErrorMessageConfig {
        fn from(value: super::ErrorMessageConfig) -> Self {
            Self {
                size: Ok(value.size),
                spacing: Ok(value.spacing),
                weight: Ok(value.weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FactSetConfig {
        spacing: Result<i64, String>,
        title: Result<super::FactSetTextConfig, String>,
        value: Result<super::FactSetTextConfig, String>,
    }
    impl Default for FactSetConfig {
        fn default() -> Self {
            Self {
                spacing: Ok(super::defaults::default_u64::<i64, 16>()),
                title: Ok(super::defaults::fact_set_config_title()),
                value: Ok(super::defaults::fact_set_config_value()),
            }
        }
    }
    impl FactSetConfig {
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spacing: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfig>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfig>,
            T::Error: std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FactSetConfig> for super::FactSetConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FactSetConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                spacing: value.spacing?,
                title: value.title?,
                value: value.value?,
            })
        }
    }
    impl From<super::FactSetConfig> for FactSetConfig {
        fn from(value: super::FactSetConfig) -> Self {
            Self {
                spacing: Ok(value.spacing),
                title: Ok(value.title),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FactSetTextConfig {
        color: Result<super::FactSetTextConfigColor, String>,
        font_type: Result<super::FactSetTextConfigFontType, String>,
        is_subtle: Result<bool, String>,
        max_width: Result<i64, String>,
        size: Result<super::FactSetTextConfigSize, String>,
        weight: Result<super::FactSetTextConfigWeight, String>,
        wrap: Result<bool, String>,
    }
    impl Default for FactSetTextConfig {
        fn default() -> Self {
            Self {
                color: Ok(super::defaults::fact_set_text_config_color()),
                font_type: Ok(super::defaults::fact_set_text_config_font_type()),
                is_subtle: Ok(Default::default()),
                max_width: Ok(Default::default()),
                size: Ok(super::defaults::fact_set_text_config_size()),
                weight: Ok(super::defaults::fact_set_text_config_weight()),
                wrap: Ok(super::defaults::default_bool::<true>()),
            }
        }
    }
    impl FactSetTextConfig {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfigColor>,
            T::Error: std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn font_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfigFontType>,
            T::Error: std::fmt::Display,
        {
            self.font_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_type: {}", e));
            self
        }
        pub fn is_subtle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_subtle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subtle: {}", e));
            self
        }
        pub fn max_width<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max_width = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_width: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfigSize>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetTextConfigWeight>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
        pub fn wrap<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.wrap = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wrap: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FactSetTextConfig> for super::FactSetTextConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FactSetTextConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                font_type: value.font_type?,
                is_subtle: value.is_subtle?,
                max_width: value.max_width?,
                size: value.size?,
                weight: value.weight?,
                wrap: value.wrap?,
            })
        }
    }
    impl From<super::FactSetTextConfig> for FactSetTextConfig {
        fn from(value: super::FactSetTextConfig) -> Self {
            Self {
                color: Ok(value.color),
                font_type: Ok(value.font_type),
                is_subtle: Ok(value.is_subtle),
                max_width: Ok(value.max_width),
                size: Ok(value.size),
                weight: Ok(value.weight),
                wrap: Ok(value.wrap),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FontColorConfig {
        default: Result<Option<String>, String>,
        subtle: Result<Option<String>, String>,
    }
    impl Default for FontColorConfig {
        fn default() -> Self {
            Self {
                default: Ok(super::defaults::font_color_config_default()),
                subtle: Ok(super::defaults::font_color_config_subtle()),
            }
        }
    }
    impl FontColorConfig {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn subtle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.subtle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtle: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FontColorConfig> for super::FontColorConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FontColorConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                subtle: value.subtle?,
            })
        }
    }
    impl From<super::FontColorConfig> for FontColorConfig {
        fn from(value: super::FontColorConfig) -> Self {
            Self {
                default: Ok(value.default),
                subtle: Ok(value.subtle),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FontSizesConfig {
        default: Result<i64, String>,
        extra_large: Result<i64, String>,
        large: Result<i64, String>,
        medium: Result<i64, String>,
        small: Result<i64, String>,
    }
    impl Default for FontSizesConfig {
        fn default() -> Self {
            Self {
                default: Ok(super::defaults::default_u64::<i64, 14>()),
                extra_large: Ok(super::defaults::default_u64::<i64, 24>()),
                large: Ok(super::defaults::default_u64::<i64, 18>()),
                medium: Ok(super::defaults::default_u64::<i64, 14>()),
                small: Ok(super::defaults::default_u64::<i64, 12>()),
            }
        }
    }
    impl FontSizesConfig {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn extra_large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.extra_large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra_large: {}", e));
            self
        }
        pub fn large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for large: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn small<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.small = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for small: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FontSizesConfig> for super::FontSizesConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FontSizesConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                extra_large: value.extra_large?,
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
            })
        }
    }
    impl From<super::FontSizesConfig> for FontSizesConfig {
        fn from(value: super::FontSizesConfig) -> Self {
            Self {
                default: Ok(value.default),
                extra_large: Ok(value.extra_large),
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FontTypeConfig {
        font_family: Result<String, String>,
        font_sizes: Result<super::FontSizesConfig, String>,
        font_weights: Result<super::FontWeightsConfig, String>,
    }
    impl Default for FontTypeConfig {
        fn default() -> Self {
            Self {
                font_family: Ok(super::defaults::font_type_config_font_family()),
                font_sizes: Ok(super::defaults::font_type_config_font_sizes()),
                font_weights: Ok(super::defaults::font_type_config_font_weights()),
            }
        }
    }
    impl FontTypeConfig {
        pub fn font_family<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.font_family = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_family: {}", e));
            self
        }
        pub fn font_sizes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontSizesConfig>,
            T::Error: std::fmt::Display,
        {
            self.font_sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_sizes: {}", e));
            self
        }
        pub fn font_weights<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontWeightsConfig>,
            T::Error: std::fmt::Display,
        {
            self.font_weights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_weights: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FontTypeConfig> for super::FontTypeConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FontTypeConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                font_family: value.font_family?,
                font_sizes: value.font_sizes?,
                font_weights: value.font_weights?,
            })
        }
    }
    impl From<super::FontTypeConfig> for FontTypeConfig {
        fn from(value: super::FontTypeConfig) -> Self {
            Self {
                font_family: Ok(value.font_family),
                font_sizes: Ok(value.font_sizes),
                font_weights: Ok(value.font_weights),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FontTypesConfig {
        default: Result<super::FontTypeConfig, String>,
        monospace: Result<super::FontTypeConfig, String>,
    }
    impl Default for FontTypesConfig {
        fn default() -> Self {
            Self {
                default: Ok(super::defaults::font_types_config_default()),
                monospace: Ok(super::defaults::font_types_config_monospace()),
            }
        }
    }
    impl FontTypesConfig {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontTypeConfig>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn monospace<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontTypeConfig>,
            T::Error: std::fmt::Display,
        {
            self.monospace = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for monospace: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FontTypesConfig> for super::FontTypesConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FontTypesConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                monospace: value.monospace?,
            })
        }
    }
    impl From<super::FontTypesConfig> for FontTypesConfig {
        fn from(value: super::FontTypesConfig) -> Self {
            Self {
                default: Ok(value.default),
                monospace: Ok(value.monospace),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FontWeightsConfig {
        bolder: Result<i64, String>,
        default: Result<i64, String>,
        lighter: Result<i64, String>,
    }
    impl Default for FontWeightsConfig {
        fn default() -> Self {
            Self {
                bolder: Ok(super::defaults::default_u64::<i64, 600>()),
                default: Ok(super::defaults::default_u64::<i64, 400>()),
                lighter: Ok(super::defaults::default_u64::<i64, 200>()),
            }
        }
    }
    impl FontWeightsConfig {
        pub fn bolder<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.bolder = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bolder: {}", e));
            self
        }
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn lighter<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.lighter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lighter: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FontWeightsConfig> for super::FontWeightsConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: FontWeightsConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                bolder: value.bolder?,
                default: value.default?,
                lighter: value.lighter?,
            })
        }
    }
    impl From<super::FontWeightsConfig> for FontWeightsConfig {
        fn from(value: super::FontWeightsConfig) -> Self {
            Self {
                bolder: Ok(value.bolder),
                default: Ok(value.default),
                lighter: Ok(value.lighter),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ForegroundColorsConfig {
        accent: Result<super::FontColorConfig, String>,
        attention: Result<super::FontColorConfig, String>,
        dark: Result<super::FontColorConfig, String>,
        default: Result<super::FontColorConfig, String>,
        good: Result<super::FontColorConfig, String>,
        light: Result<super::FontColorConfig, String>,
        warning: Result<super::FontColorConfig, String>,
    }
    impl Default for ForegroundColorsConfig {
        fn default() -> Self {
            Self {
                accent: Ok(super::defaults::foreground_colors_config_accent()),
                attention: Ok(super::defaults::foreground_colors_config_attention()),
                dark: Ok(super::defaults::foreground_colors_config_dark()),
                default: Ok(super::defaults::foreground_colors_config_default()),
                good: Ok(super::defaults::foreground_colors_config_good()),
                light: Ok(super::defaults::foreground_colors_config_light()),
                warning: Ok(super::defaults::foreground_colors_config_warning()),
            }
        }
    }
    impl ForegroundColorsConfig {
        pub fn accent<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accent: {}", e));
            self
        }
        pub fn attention<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.attention = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for attention: {}", e));
            self
        }
        pub fn dark<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.dark = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dark: {}", e));
            self
        }
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn good<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.good = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for good: {}", e));
            self
        }
        pub fn light<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.light = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for light: {}", e));
            self
        }
        pub fn warning<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontColorConfig>,
            T::Error: std::fmt::Display,
        {
            self.warning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ForegroundColorsConfig> for super::ForegroundColorsConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ForegroundColorsConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                accent: value.accent?,
                attention: value.attention?,
                dark: value.dark?,
                default: value.default?,
                good: value.good?,
                light: value.light?,
                warning: value.warning?,
            })
        }
    }
    impl From<super::ForegroundColorsConfig> for ForegroundColorsConfig {
        fn from(value: super::ForegroundColorsConfig) -> Self {
            Self {
                accent: Ok(value.accent),
                attention: Ok(value.attention),
                dark: Ok(value.dark),
                default: Ok(value.default),
                good: Ok(value.good),
                light: Ok(value.light),
                warning: Ok(value.warning),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HostConfig {
        actions: Result<super::ActionsConfig, String>,
        adaptive_card: Result<super::AdaptiveCardConfig, String>,
        choice_set_input_value_separator: Result<String, String>,
        container_styles: Result<super::ContainerStylesConfig, String>,
        fact_set: Result<super::FactSetConfig, String>,
        font_family: Result<Option<String>, String>,
        font_sizes: Result<Option<super::FontSizesConfig>, String>,
        font_types: Result<super::FontTypesConfig, String>,
        font_weights: Result<Option<super::FontWeightsConfig>, String>,
        image_base_url: Result<Option<String>, String>,
        image_set: Result<super::ImageSetConfig, String>,
        image_sizes: Result<super::ImageSizesConfig, String>,
        inputs: Result<super::InputsConfig, String>,
        media: Result<super::MediaConfig, String>,
        schema: Result<Option<String>, String>,
        separator: Result<super::SeparatorConfig, String>,
        spacing: Result<super::SpacingsConfig, String>,
        supports_interactivity: Result<bool, String>,
        text_block: Result<super::TextBlockConfig, String>,
        text_styles: Result<super::TextStylesConfig, String>,
    }
    impl Default for HostConfig {
        fn default() -> Self {
            Self {
                actions: Ok(super::defaults::host_config_actions()),
                adaptive_card: Ok(super::defaults::host_config_adaptive_card()),
                choice_set_input_value_separator: Ok(
                    super::defaults::host_config_choice_set_input_value_separator(),
                ),
                container_styles: Ok(super::defaults::host_config_container_styles()),
                fact_set: Ok(super::defaults::host_config_fact_set()),
                font_family: Ok(Default::default()),
                font_sizes: Ok(Default::default()),
                font_types: Ok(super::defaults::host_config_font_types()),
                font_weights: Ok(Default::default()),
                image_base_url: Ok(Default::default()),
                image_set: Ok(super::defaults::host_config_image_set()),
                image_sizes: Ok(super::defaults::host_config_image_sizes()),
                inputs: Ok(super::defaults::host_config_inputs()),
                media: Ok(super::defaults::host_config_media()),
                schema: Ok(Default::default()),
                separator: Ok(super::defaults::host_config_separator()),
                spacing: Ok(super::defaults::host_config_spacing()),
                supports_interactivity: Ok(Default::default()),
                text_block: Ok(super::defaults::host_config_text_block()),
                text_styles: Ok(super::defaults::host_config_text_styles()),
            }
        }
    }
    impl HostConfig {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ActionsConfig>,
            T::Error: std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {}", e));
            self
        }
        pub fn adaptive_card<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AdaptiveCardConfig>,
            T::Error: std::fmt::Display,
        {
            self.adaptive_card = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for adaptive_card: {}", e));
            self
        }
        pub fn choice_set_input_value_separator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.choice_set_input_value_separator = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for choice_set_input_value_separator: {}",
                    e
                )
            });
            self
        }
        pub fn container_styles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ContainerStylesConfig>,
            T::Error: std::fmt::Display,
        {
            self.container_styles = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for container_styles: {}",
                    e
                )
            });
            self
        }
        pub fn fact_set<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FactSetConfig>,
            T::Error: std::fmt::Display,
        {
            self.fact_set = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fact_set: {}", e));
            self
        }
        pub fn font_family<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.font_family = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_family: {}", e));
            self
        }
        pub fn font_sizes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FontSizesConfig>>,
            T::Error: std::fmt::Display,
        {
            self.font_sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_sizes: {}", e));
            self
        }
        pub fn font_types<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::FontTypesConfig>,
            T::Error: std::fmt::Display,
        {
            self.font_types = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_types: {}", e));
            self
        }
        pub fn font_weights<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FontWeightsConfig>>,
            T::Error: std::fmt::Display,
        {
            self.font_weights = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_weights: {}", e));
            self
        }
        pub fn image_base_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image_base_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_base_url: {}", e));
            self
        }
        pub fn image_set<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ImageSetConfig>,
            T::Error: std::fmt::Display,
        {
            self.image_set = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_set: {}", e));
            self
        }
        pub fn image_sizes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ImageSizesConfig>,
            T::Error: std::fmt::Display,
        {
            self.image_sizes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_sizes: {}", e));
            self
        }
        pub fn inputs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputsConfig>,
            T::Error: std::fmt::Display,
        {
            self.inputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for inputs: {}", e));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MediaConfig>,
            T::Error: std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
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
        pub fn separator<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SeparatorConfig>,
            T::Error: std::fmt::Display,
        {
            self.separator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for separator: {}", e));
            self
        }
        pub fn spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SpacingsConfig>,
            T::Error: std::fmt::Display,
        {
            self.spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spacing: {}", e));
            self
        }
        pub fn supports_interactivity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.supports_interactivity = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for supports_interactivity: {}",
                    e
                )
            });
            self
        }
        pub fn text_block<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextBlockConfig>,
            T::Error: std::fmt::Display,
        {
            self.text_block = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_block: {}", e));
            self
        }
        pub fn text_styles<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStylesConfig>,
            T::Error: std::fmt::Display,
        {
            self.text_styles = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_styles: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HostConfig> for super::HostConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: HostConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                actions: value.actions?,
                adaptive_card: value.adaptive_card?,
                choice_set_input_value_separator: value.choice_set_input_value_separator?,
                container_styles: value.container_styles?,
                fact_set: value.fact_set?,
                font_family: value.font_family?,
                font_sizes: value.font_sizes?,
                font_types: value.font_types?,
                font_weights: value.font_weights?,
                image_base_url: value.image_base_url?,
                image_set: value.image_set?,
                image_sizes: value.image_sizes?,
                inputs: value.inputs?,
                media: value.media?,
                schema: value.schema?,
                separator: value.separator?,
                spacing: value.spacing?,
                supports_interactivity: value.supports_interactivity?,
                text_block: value.text_block?,
                text_styles: value.text_styles?,
            })
        }
    }
    impl From<super::HostConfig> for HostConfig {
        fn from(value: super::HostConfig) -> Self {
            Self {
                actions: Ok(value.actions),
                adaptive_card: Ok(value.adaptive_card),
                choice_set_input_value_separator: Ok(value.choice_set_input_value_separator),
                container_styles: Ok(value.container_styles),
                fact_set: Ok(value.fact_set),
                font_family: Ok(value.font_family),
                font_sizes: Ok(value.font_sizes),
                font_types: Ok(value.font_types),
                font_weights: Ok(value.font_weights),
                image_base_url: Ok(value.image_base_url),
                image_set: Ok(value.image_set),
                image_sizes: Ok(value.image_sizes),
                inputs: Ok(value.inputs),
                media: Ok(value.media),
                schema: Ok(value.schema),
                separator: Ok(value.separator),
                spacing: Ok(value.spacing),
                supports_interactivity: Ok(value.supports_interactivity),
                text_block: Ok(value.text_block),
                text_styles: Ok(value.text_styles),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageSetConfig {
        image_size: Result<super::ImageSetConfigImageSize, String>,
        max_image_height: Result<i64, String>,
    }
    impl Default for ImageSetConfig {
        fn default() -> Self {
            Self {
                image_size: Ok(super::defaults::image_set_config_image_size()),
                max_image_height: Ok(super::defaults::default_u64::<i64, 100>()),
            }
        }
    }
    impl ImageSetConfig {
        pub fn image_size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ImageSetConfigImageSize>,
            T::Error: std::fmt::Display,
        {
            self.image_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image_size: {}", e));
            self
        }
        pub fn max_image_height<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.max_image_height = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for max_image_height: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<ImageSetConfig> for super::ImageSetConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ImageSetConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                image_size: value.image_size?,
                max_image_height: value.max_image_height?,
            })
        }
    }
    impl From<super::ImageSetConfig> for ImageSetConfig {
        fn from(value: super::ImageSetConfig) -> Self {
            Self {
                image_size: Ok(value.image_size),
                max_image_height: Ok(value.max_image_height),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImageSizesConfig {
        large: Result<i64, String>,
        medium: Result<i64, String>,
        small: Result<i64, String>,
    }
    impl Default for ImageSizesConfig {
        fn default() -> Self {
            Self {
                large: Ok(super::defaults::default_u64::<i64, 100>()),
                medium: Ok(super::defaults::default_u64::<i64, 52>()),
                small: Ok(super::defaults::default_u64::<i64, 32>()),
            }
        }
    }
    impl ImageSizesConfig {
        pub fn large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for large: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn small<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.small = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for small: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ImageSizesConfig> for super::ImageSizesConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ImageSizesConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                large: value.large?,
                medium: value.medium?,
                small: value.small?,
            })
        }
    }
    impl From<super::ImageSizesConfig> for ImageSizesConfig {
        fn from(value: super::ImageSizesConfig) -> Self {
            Self {
                large: Ok(value.large),
                medium: Ok(value.medium),
                small: Ok(value.small),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputLabelConfig {
        color: Result<super::InputLabelConfigColor, String>,
        is_subtle: Result<bool, String>,
        size: Result<super::InputLabelConfigSize, String>,
        suffix: Result<Option<String>, String>,
        weight: Result<super::InputLabelConfigWeight, String>,
    }
    impl Default for InputLabelConfig {
        fn default() -> Self {
            Self {
                color: Ok(super::defaults::input_label_config_color()),
                is_subtle: Ok(Default::default()),
                size: Ok(super::defaults::input_label_config_size()),
                suffix: Ok(Default::default()),
                weight: Ok(super::defaults::input_label_config_weight()),
            }
        }
    }
    impl InputLabelConfig {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputLabelConfigColor>,
            T::Error: std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn is_subtle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_subtle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subtle: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputLabelConfigSize>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn suffix<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.suffix = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suffix: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputLabelConfigWeight>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InputLabelConfig> for super::InputLabelConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: InputLabelConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                is_subtle: value.is_subtle?,
                size: value.size?,
                suffix: value.suffix?,
                weight: value.weight?,
            })
        }
    }
    impl From<super::InputLabelConfig> for InputLabelConfig {
        fn from(value: super::InputLabelConfig) -> Self {
            Self {
                color: Ok(value.color),
                is_subtle: Ok(value.is_subtle),
                size: Ok(value.size),
                suffix: Ok(value.suffix),
                weight: Ok(value.weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InputsConfig {
        error_message: Result<super::ErrorMessageConfig, String>,
        label: Result<super::LabelConfig, String>,
    }
    impl Default for InputsConfig {
        fn default() -> Self {
            Self {
                error_message: Ok(super::defaults::inputs_config_error_message()),
                label: Ok(super::defaults::inputs_config_label()),
            }
        }
    }
    impl InputsConfig {
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ErrorMessageConfig>,
            T::Error: std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LabelConfig>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<InputsConfig> for super::InputsConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: InputsConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                error_message: value.error_message?,
                label: value.label?,
            })
        }
    }
    impl From<super::InputsConfig> for InputsConfig {
        fn from(value: super::InputsConfig) -> Self {
            Self {
                error_message: Ok(value.error_message),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LabelConfig {
        input_spacing: Result<super::LabelConfigInputSpacing, String>,
        optional_inputs: Result<super::InputLabelConfig, String>,
        required_inputs: Result<super::InputLabelConfig, String>,
    }
    impl Default for LabelConfig {
        fn default() -> Self {
            Self {
                input_spacing: Ok(super::defaults::label_config_input_spacing()),
                optional_inputs: Ok(super::defaults::label_config_optional_inputs()),
                required_inputs: Ok(super::defaults::label_config_required_inputs()),
            }
        }
    }
    impl LabelConfig {
        pub fn input_spacing<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LabelConfigInputSpacing>,
            T::Error: std::fmt::Display,
        {
            self.input_spacing = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_spacing: {}", e));
            self
        }
        pub fn optional_inputs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputLabelConfig>,
            T::Error: std::fmt::Display,
        {
            self.optional_inputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for optional_inputs: {}", e));
            self
        }
        pub fn required_inputs<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::InputLabelConfig>,
            T::Error: std::fmt::Display,
        {
            self.required_inputs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required_inputs: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LabelConfig> for super::LabelConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: LabelConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                input_spacing: value.input_spacing?,
                optional_inputs: value.optional_inputs?,
                required_inputs: value.required_inputs?,
            })
        }
    }
    impl From<super::LabelConfig> for LabelConfig {
        fn from(value: super::LabelConfig) -> Self {
            Self {
                input_spacing: Ok(value.input_spacing),
                optional_inputs: Ok(value.optional_inputs),
                required_inputs: Ok(value.required_inputs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MediaConfig {
        allow_inline_playback: Result<bool, String>,
        default_poster: Result<Option<String>, String>,
        play_button: Result<Option<String>, String>,
    }
    impl Default for MediaConfig {
        fn default() -> Self {
            Self {
                allow_inline_playback: Ok(super::defaults::default_bool::<true>()),
                default_poster: Ok(Default::default()),
                play_button: Ok(Default::default()),
            }
        }
    }
    impl MediaConfig {
        pub fn allow_inline_playback<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.allow_inline_playback = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for allow_inline_playback: {}",
                    e
                )
            });
            self
        }
        pub fn default_poster<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.default_poster = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_poster: {}", e));
            self
        }
        pub fn play_button<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.play_button = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for play_button: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<MediaConfig> for super::MediaConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: MediaConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                allow_inline_playback: value.allow_inline_playback?,
                default_poster: value.default_poster?,
                play_button: value.play_button?,
            })
        }
    }
    impl From<super::MediaConfig> for MediaConfig {
        fn from(value: super::MediaConfig) -> Self {
            Self {
                allow_inline_playback: Ok(value.allow_inline_playback),
                default_poster: Ok(value.default_poster),
                play_button: Ok(value.play_button),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SeparatorConfig {
        line_color: Result<String, String>,
        line_thickness: Result<i64, String>,
    }
    impl Default for SeparatorConfig {
        fn default() -> Self {
            Self {
                line_color: Ok(super::defaults::separator_config_line_color()),
                line_thickness: Ok(super::defaults::default_u64::<i64, 1>()),
            }
        }
    }
    impl SeparatorConfig {
        pub fn line_color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.line_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_color: {}", e));
            self
        }
        pub fn line_thickness<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.line_thickness = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for line_thickness: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SeparatorConfig> for super::SeparatorConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: SeparatorConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                line_color: value.line_color?,
                line_thickness: value.line_thickness?,
            })
        }
    }
    impl From<super::SeparatorConfig> for SeparatorConfig {
        fn from(value: super::SeparatorConfig) -> Self {
            Self {
                line_color: Ok(value.line_color),
                line_thickness: Ok(value.line_thickness),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShowCardConfig {
        action_mode: Result<super::ShowCardConfigActionMode, String>,
        inline_top_margin: Result<i64, String>,
        style: Result<super::ShowCardConfigStyle, String>,
    }
    impl Default for ShowCardConfig {
        fn default() -> Self {
            Self {
                action_mode: Ok(super::defaults::show_card_config_action_mode()),
                inline_top_margin: Ok(super::defaults::default_u64::<i64, 16>()),
                style: Ok(super::defaults::show_card_config_style()),
            }
        }
    }
    impl ShowCardConfig {
        pub fn action_mode<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShowCardConfigActionMode>,
            T::Error: std::fmt::Display,
        {
            self.action_mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action_mode: {}", e));
            self
        }
        pub fn inline_top_margin<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.inline_top_margin = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for inline_top_margin: {}",
                    e
                )
            });
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShowCardConfigStyle>,
            T::Error: std::fmt::Display,
        {
            self.style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ShowCardConfig> for super::ShowCardConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: ShowCardConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_mode: value.action_mode?,
                inline_top_margin: value.inline_top_margin?,
                style: value.style?,
            })
        }
    }
    impl From<super::ShowCardConfig> for ShowCardConfig {
        fn from(value: super::ShowCardConfig) -> Self {
            Self {
                action_mode: Ok(value.action_mode),
                inline_top_margin: Ok(value.inline_top_margin),
                style: Ok(value.style),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SpacingsConfig {
        default: Result<i64, String>,
        extra_large: Result<i64, String>,
        large: Result<i64, String>,
        medium: Result<i64, String>,
        padding: Result<i64, String>,
        small: Result<i64, String>,
    }
    impl Default for SpacingsConfig {
        fn default() -> Self {
            Self {
                default: Ok(super::defaults::default_u64::<i64, 12>()),
                extra_large: Ok(super::defaults::default_u64::<i64, 24>()),
                large: Ok(super::defaults::default_u64::<i64, 20>()),
                medium: Ok(super::defaults::default_u64::<i64, 16>()),
                padding: Ok(super::defaults::default_u64::<i64, 16>()),
                small: Ok(super::defaults::default_u64::<i64, 8>()),
            }
        }
    }
    impl SpacingsConfig {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default: {}", e));
            self
        }
        pub fn extra_large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.extra_large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra_large: {}", e));
            self
        }
        pub fn large<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.large = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for large: {}", e));
            self
        }
        pub fn medium<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.medium = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medium: {}", e));
            self
        }
        pub fn padding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.padding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for padding: {}", e));
            self
        }
        pub fn small<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.small = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for small: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SpacingsConfig> for super::SpacingsConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: SpacingsConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                default: value.default?,
                extra_large: value.extra_large?,
                large: value.large?,
                medium: value.medium?,
                padding: value.padding?,
                small: value.small?,
            })
        }
    }
    impl From<super::SpacingsConfig> for SpacingsConfig {
        fn from(value: super::SpacingsConfig) -> Self {
            Self {
                default: Ok(value.default),
                extra_large: Ok(value.extra_large),
                large: Ok(value.large),
                medium: Ok(value.medium),
                padding: Ok(value.padding),
                small: Ok(value.small),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextBlockConfig {
        heading_level: Result<i64, String>,
    }
    impl Default for TextBlockConfig {
        fn default() -> Self {
            Self {
                heading_level: Ok(super::defaults::default_u64::<i64, 2>()),
            }
        }
    }
    impl TextBlockConfig {
        pub fn heading_level<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.heading_level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heading_level: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextBlockConfig> for super::TextBlockConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: TextBlockConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                heading_level: value.heading_level?,
            })
        }
    }
    impl From<super::TextBlockConfig> for TextBlockConfig {
        fn from(value: super::TextBlockConfig) -> Self {
            Self {
                heading_level: Ok(value.heading_level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextStyleConfig {
        color: Result<super::TextStyleConfigColor, String>,
        font_type: Result<super::TextStyleConfigFontType, String>,
        is_subtle: Result<bool, String>,
        size: Result<super::TextStyleConfigSize, String>,
        weight: Result<super::TextStyleConfigWeight, String>,
    }
    impl Default for TextStyleConfig {
        fn default() -> Self {
            Self {
                color: Ok(super::defaults::text_style_config_color()),
                font_type: Ok(super::defaults::text_style_config_font_type()),
                is_subtle: Ok(Default::default()),
                size: Ok(super::defaults::text_style_config_size()),
                weight: Ok(super::defaults::text_style_config_weight()),
            }
        }
    }
    impl TextStyleConfig {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfigColor>,
            T::Error: std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {}", e));
            self
        }
        pub fn font_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfigFontType>,
            T::Error: std::fmt::Display,
        {
            self.font_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_type: {}", e));
            self
        }
        pub fn is_subtle<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.is_subtle = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_subtle: {}", e));
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfigSize>,
            T::Error: std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfigWeight>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextStyleConfig> for super::TextStyleConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: TextStyleConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                font_type: value.font_type?,
                is_subtle: value.is_subtle?,
                size: value.size?,
                weight: value.weight?,
            })
        }
    }
    impl From<super::TextStyleConfig> for TextStyleConfig {
        fn from(value: super::TextStyleConfig) -> Self {
            Self {
                color: Ok(value.color),
                font_type: Ok(value.font_type),
                is_subtle: Ok(value.is_subtle),
                size: Ok(value.size),
                weight: Ok(value.weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextStylesConfig {
        column_header: Result<super::TextStyleConfig, String>,
        heading: Result<super::TextStyleConfig, String>,
    }
    impl Default for TextStylesConfig {
        fn default() -> Self {
            Self {
                column_header: Ok(super::defaults::text_styles_config_column_header()),
                heading: Ok(super::defaults::text_styles_config_heading()),
            }
        }
    }
    impl TextStylesConfig {
        pub fn column_header<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.column_header = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for column_header: {}", e));
            self
        }
        pub fn heading<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextStyleConfig>,
            T::Error: std::fmt::Display,
        {
            self.heading = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for heading: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextStylesConfig> for super::TextStylesConfig {
        type Error = super::error::ConversionError;
        fn try_from(value: TextStylesConfig) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                column_header: value.column_header?,
                heading: value.heading?,
            })
        }
    }
    impl From<super::TextStylesConfig> for TextStylesConfig {
        fn from(value: super::TextStylesConfig) -> Self {
            Self {
                column_header: Ok(value.column_header),
                heading: Ok(value.heading),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn actions_config_action_alignment() -> super::ActionsConfigActionAlignment {
        super::ActionsConfigActionAlignment::Stretch
    }
    pub(super) fn actions_config_actions_orientation() -> super::ActionsConfigActionsOrientation {
        super::ActionsConfigActionsOrientation::Horizontal
    }
    pub(super) fn actions_config_icon_placement() -> super::ActionsConfigIconPlacement {
        super::ActionsConfigIconPlacement::AboveTitle
    }
    pub(super) fn actions_config_show_card() -> super::ShowCardConfig {
        super::ShowCardConfig {
            action_mode: super::ShowCardConfigActionMode::Inline,
            inline_top_margin: 16_i64,
            style: super::ShowCardConfigStyle::Emphasis,
        }
    }
    pub(super) fn actions_config_spacing() -> super::ActionsConfigSpacing {
        super::ActionsConfigSpacing::Default
    }
    pub(super) fn container_style_config_foreground_colors() -> super::ForegroundColorsConfig {
        super::ForegroundColorsConfig {
            accent: super::FontColorConfig {
                default: Some("#6264a7".to_string()),
                subtle: Some("#8b8cc7".to_string()),
            },
            attention: super::FontColorConfig {
                default: Some("#c4314b".to_string()),
                subtle: Some("#e5c4314b".to_string()),
            },
            dark: super::FontColorConfig {
                default: Some("#252424".to_string()),
                subtle: Some("#bf252424".to_string()),
            },
            default: super::FontColorConfig {
                default: Some("#ff252424".to_string()),
                subtle: Some("#bf252424".to_string()),
            },
            good: super::FontColorConfig {
                default: Some("#92c353".to_string()),
                subtle: Some("#e592c353".to_string()),
            },
            light: super::FontColorConfig {
                default: Some("#ffffff".to_string()),
                subtle: Some("#fff3f2f1".to_string()),
            },
            warning: super::FontColorConfig {
                default: Some("#f8d22a".to_string()),
                subtle: Some("#e5f8d22a".to_string()),
            },
        }
    }
    pub(super) fn container_styles_config_accent() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#C7DEF9".to_string()),
            border_color: Some("#62A8F7".to_string()),
            border_thickness: Some(1_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn container_styles_config_attention() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#FFC5B2".to_string()),
            border_color: Some("#FF764C".to_string()),
            border_thickness: Some(1_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn container_styles_config_default() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#ffffff".to_string()),
            border_color: Some("#CCCCCC".to_string()),
            border_thickness: Some(0_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn container_styles_config_emphasis() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#fff9f8f7".to_string()),
            border_color: Some("#666666".to_string()),
            border_thickness: Some(1_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn container_styles_config_good() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#CCFFCC".to_string()),
            border_color: Some("#69E569".to_string()),
            border_thickness: Some(1_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn container_styles_config_warning() -> super::ContainerStyleConfig {
        super::ContainerStyleConfig {
            background_color: Some("#FFE2B2".to_string()),
            border_color: Some("#FFBC51".to_string()),
            border_thickness: Some(1_i64),
            foreground_colors: super::ForegroundColorsConfig {
                accent: super::FontColorConfig {
                    default: Some("#6264a7".to_string()),
                    subtle: Some("#8b8cc7".to_string()),
                },
                attention: super::FontColorConfig {
                    default: Some("#c4314b".to_string()),
                    subtle: Some("#e5c4314b".to_string()),
                },
                dark: super::FontColorConfig {
                    default: Some("#252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                default: super::FontColorConfig {
                    default: Some("#ff252424".to_string()),
                    subtle: Some("#bf252424".to_string()),
                },
                good: super::FontColorConfig {
                    default: Some("#92c353".to_string()),
                    subtle: Some("#e592c353".to_string()),
                },
                light: super::FontColorConfig {
                    default: Some("#ffffff".to_string()),
                    subtle: Some("#fff3f2f1".to_string()),
                },
                warning: super::FontColorConfig {
                    default: Some("#f8d22a".to_string()),
                    subtle: Some("#e5f8d22a".to_string()),
                },
            },
            table_grid_lines_color: None,
            table_grid_lines_thickness: Default::default(),
        }
    }
    pub(super) fn error_message_config_size() -> super::ErrorMessageConfigSize {
        super::ErrorMessageConfigSize::Default
    }
    pub(super) fn error_message_config_spacing() -> super::ErrorMessageConfigSpacing {
        super::ErrorMessageConfigSpacing::Default
    }
    pub(super) fn error_message_config_weight() -> super::ErrorMessageConfigWeight {
        super::ErrorMessageConfigWeight::Default
    }
    pub(super) fn fact_set_config_title() -> super::FactSetTextConfig {
        super::FactSetTextConfig {
            color: super::FactSetTextConfigColor::Default,
            font_type: super::FactSetTextConfigFontType::Default,
            is_subtle: false,
            max_width: 150_i64,
            size: super::FactSetTextConfigSize::Default,
            weight: super::FactSetTextConfigWeight::Bolder,
            wrap: true,
        }
    }
    pub(super) fn fact_set_config_value() -> super::FactSetTextConfig {
        super::FactSetTextConfig {
            color: super::FactSetTextConfigColor::Default,
            font_type: super::FactSetTextConfigFontType::Default,
            is_subtle: false,
            max_width: 0_i64,
            size: super::FactSetTextConfigSize::Default,
            weight: super::FactSetTextConfigWeight::Default,
            wrap: true,
        }
    }
    pub(super) fn fact_set_text_config_color() -> super::FactSetTextConfigColor {
        super::FactSetTextConfigColor::Default
    }
    pub(super) fn fact_set_text_config_font_type() -> super::FactSetTextConfigFontType {
        super::FactSetTextConfigFontType::Default
    }
    pub(super) fn fact_set_text_config_size() -> super::FactSetTextConfigSize {
        super::FactSetTextConfigSize::Default
    }
    pub(super) fn fact_set_text_config_weight() -> super::FactSetTextConfigWeight {
        super::FactSetTextConfigWeight::Default
    }
    pub(super) fn font_color_config_default() -> Option<String> {
        Some("#ff252424".to_string())
    }
    pub(super) fn font_color_config_subtle() -> Option<String> {
        Some("#bf252424".to_string())
    }
    pub(super) fn font_type_config_font_family() -> String {
        "sans-serif".to_string()
    }
    pub(super) fn font_type_config_font_sizes() -> super::FontSizesConfig {
        super::FontSizesConfig {
            default: 14_i64,
            extra_large: 24_i64,
            large: 18_i64,
            medium: 14_i64,
            small: 12_i64,
        }
    }
    pub(super) fn font_type_config_font_weights() -> super::FontWeightsConfig {
        super::FontWeightsConfig {
            bolder: 600_i64,
            default: 400_i64,
            lighter: 200_i64,
        }
    }
    pub(super) fn font_types_config_default() -> super::FontTypeConfig {
        super::FontTypeConfig {
            font_family: "sans-serif".to_string(),
            font_sizes: super::FontSizesConfig {
                default: 14_i64,
                extra_large: 24_i64,
                large: 18_i64,
                medium: 14_i64,
                small: 12_i64,
            },
            font_weights: super::FontWeightsConfig {
                bolder: 600_i64,
                default: 400_i64,
                lighter: 200_i64,
            },
        }
    }
    pub(super) fn font_types_config_monospace() -> super::FontTypeConfig {
        super::FontTypeConfig {
            font_family: "monospace".to_string(),
            font_sizes: super::FontSizesConfig {
                default: 14_i64,
                extra_large: 24_i64,
                large: 18_i64,
                medium: 14_i64,
                small: 12_i64,
            },
            font_weights: super::FontWeightsConfig {
                bolder: 600_i64,
                default: 400_i64,
                lighter: 200_i64,
            },
        }
    }
    pub(super) fn foreground_colors_config_accent() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#6264a7".to_string()),
            subtle: Some("#8b8cc7".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_attention() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#c4314b".to_string()),
            subtle: Some("#e5c4314b".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_dark() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#252424".to_string()),
            subtle: Some("#bf252424".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_default() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#ff252424".to_string()),
            subtle: Some("#bf252424".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_good() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#92c353".to_string()),
            subtle: Some("#e592c353".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_light() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#ffffff".to_string()),
            subtle: Some("#fff3f2f1".to_string()),
        }
    }
    pub(super) fn foreground_colors_config_warning() -> super::FontColorConfig {
        super::FontColorConfig {
            default: Some("#f8d22a".to_string()),
            subtle: Some("#e5f8d22a".to_string()),
        }
    }
    pub(super) fn host_config_actions() -> super::ActionsConfig {
        super::ActionsConfig {
            action_alignment: super::ActionsConfigActionAlignment::Stretch,
            actions_orientation: super::ActionsConfigActionsOrientation::Horizontal,
            button_spacing: 10_i64,
            icon_placement: super::ActionsConfigIconPlacement::AboveTitle,
            icon_size: 30_i64,
            max_actions: 5_i64,
            pre_expand_single_show_card_action: false,
            show_card: super::ShowCardConfig {
                action_mode: super::ShowCardConfigActionMode::Inline,
                inline_top_margin: 16_i64,
                style: super::ShowCardConfigStyle::Emphasis,
            },
            spacing: super::ActionsConfigSpacing::Default,
        }
    }
    pub(super) fn host_config_adaptive_card() -> super::AdaptiveCardConfig {
        super::AdaptiveCardConfig {
            allow_custom_style: true,
        }
    }
    pub(super) fn host_config_choice_set_input_value_separator() -> String {
        ", ".to_string()
    }
    pub(super) fn host_config_container_styles() -> super::ContainerStylesConfig {
        super::ContainerStylesConfig {
            accent: super::ContainerStyleConfig {
                background_color: Some("#C7DEF9".to_string()),
                border_color: Some("#62A8F7".to_string()),
                border_thickness: Some(1_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
            attention: super::ContainerStyleConfig {
                background_color: Some("#FFC5B2".to_string()),
                border_color: Some("#FF764C".to_string()),
                border_thickness: Some(1_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
            default: super::ContainerStyleConfig {
                background_color: Some("#ffffff".to_string()),
                border_color: Some("#CCCCCC".to_string()),
                border_thickness: Some(0_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
            emphasis: super::ContainerStyleConfig {
                background_color: Some("#fff9f8f7".to_string()),
                border_color: Some("#666666".to_string()),
                border_thickness: Some(1_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
            good: super::ContainerStyleConfig {
                background_color: Some("#CCFFCC".to_string()),
                border_color: Some("#69E569".to_string()),
                border_thickness: Some(1_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
            warning: super::ContainerStyleConfig {
                background_color: Some("#FFE2B2".to_string()),
                border_color: Some("#FFBC51".to_string()),
                border_thickness: Some(1_i64),
                foreground_colors: super::ForegroundColorsConfig {
                    accent: super::FontColorConfig {
                        default: Some("#6264a7".to_string()),
                        subtle: Some("#8b8cc7".to_string()),
                    },
                    attention: super::FontColorConfig {
                        default: Some("#c4314b".to_string()),
                        subtle: Some("#e5c4314b".to_string()),
                    },
                    dark: super::FontColorConfig {
                        default: Some("#252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    default: super::FontColorConfig {
                        default: Some("#ff252424".to_string()),
                        subtle: Some("#bf252424".to_string()),
                    },
                    good: super::FontColorConfig {
                        default: Some("#92c353".to_string()),
                        subtle: Some("#e592c353".to_string()),
                    },
                    light: super::FontColorConfig {
                        default: Some("#ffffff".to_string()),
                        subtle: Some("#fff3f2f1".to_string()),
                    },
                    warning: super::FontColorConfig {
                        default: Some("#f8d22a".to_string()),
                        subtle: Some("#e5f8d22a".to_string()),
                    },
                },
                table_grid_lines_color: None,
                table_grid_lines_thickness: Default::default(),
            },
        }
    }
    pub(super) fn host_config_fact_set() -> super::FactSetConfig {
        super::FactSetConfig {
            spacing: 16_i64,
            title: super::FactSetTextConfig {
                color: super::FactSetTextConfigColor::Default,
                font_type: super::FactSetTextConfigFontType::Default,
                is_subtle: false,
                max_width: 150_i64,
                size: super::FactSetTextConfigSize::Default,
                weight: super::FactSetTextConfigWeight::Bolder,
                wrap: true,
            },
            value: super::FactSetTextConfig {
                color: super::FactSetTextConfigColor::Default,
                font_type: super::FactSetTextConfigFontType::Default,
                is_subtle: false,
                max_width: 0_i64,
                size: super::FactSetTextConfigSize::Default,
                weight: super::FactSetTextConfigWeight::Default,
                wrap: true,
            },
        }
    }
    pub(super) fn host_config_font_types() -> super::FontTypesConfig {
        super::FontTypesConfig {
            default: super::FontTypeConfig {
                font_family: "sans-serif".to_string(),
                font_sizes: super::FontSizesConfig {
                    default: 14_i64,
                    extra_large: 24_i64,
                    large: 18_i64,
                    medium: 14_i64,
                    small: 12_i64,
                },
                font_weights: super::FontWeightsConfig {
                    bolder: 600_i64,
                    default: 400_i64,
                    lighter: 200_i64,
                },
            },
            monospace: super::FontTypeConfig {
                font_family: "monospace".to_string(),
                font_sizes: super::FontSizesConfig {
                    default: 14_i64,
                    extra_large: 24_i64,
                    large: 18_i64,
                    medium: 14_i64,
                    small: 12_i64,
                },
                font_weights: super::FontWeightsConfig {
                    bolder: 600_i64,
                    default: 400_i64,
                    lighter: 200_i64,
                },
            },
        }
    }
    pub(super) fn host_config_image_set() -> super::ImageSetConfig {
        super::ImageSetConfig {
            image_size: super::ImageSetConfigImageSize::Medium,
            max_image_height: 100_i64,
        }
    }
    pub(super) fn host_config_image_sizes() -> super::ImageSizesConfig {
        super::ImageSizesConfig {
            large: 100_i64,
            medium: 52_i64,
            small: 32_i64,
        }
    }
    pub(super) fn host_config_inputs() -> super::InputsConfig {
        super::InputsConfig {
            error_message: super::ErrorMessageConfig {
                size: super::ErrorMessageConfigSize::Default,
                spacing: super::ErrorMessageConfigSpacing::Default,
                weight: super::ErrorMessageConfigWeight::Default,
            },
            label: super::LabelConfig {
                input_spacing: super::LabelConfigInputSpacing::Default,
                optional_inputs: super::InputLabelConfig {
                    color: super::InputLabelConfigColor::Default,
                    is_subtle: false,
                    size: super::InputLabelConfigSize::Default,
                    suffix: Default::default(),
                    weight: super::InputLabelConfigWeight::Default,
                },
                required_inputs: super::InputLabelConfig {
                    color: super::InputLabelConfigColor::Default,
                    is_subtle: false,
                    size: super::InputLabelConfigSize::Default,
                    suffix: Default::default(),
                    weight: super::InputLabelConfigWeight::Default,
                },
            },
        }
    }
    pub(super) fn host_config_media() -> super::MediaConfig {
        super::MediaConfig {
            allow_inline_playback: true,
            default_poster: Default::default(),
            play_button: Default::default(),
        }
    }
    pub(super) fn host_config_separator() -> super::SeparatorConfig {
        super::SeparatorConfig {
            line_color: "#EEEEEE".to_string(),
            line_thickness: 1_i64,
        }
    }
    pub(super) fn host_config_spacing() -> super::SpacingsConfig {
        super::SpacingsConfig {
            default: 12_i64,
            extra_large: 24_i64,
            large: 20_i64,
            medium: 16_i64,
            padding: 16_i64,
            small: 8_i64,
        }
    }
    pub(super) fn host_config_text_block() -> super::TextBlockConfig {
        super::TextBlockConfig {
            heading_level: 2_i64,
        }
    }
    pub(super) fn host_config_text_styles() -> super::TextStylesConfig {
        super::TextStylesConfig {
            column_header: super::TextStyleConfig {
                color: super::TextStyleConfigColor::Default,
                font_type: super::TextStyleConfigFontType::Default,
                is_subtle: false,
                size: super::TextStyleConfigSize::Default,
                weight: super::TextStyleConfigWeight::Bolder,
            },
            heading: super::TextStyleConfig {
                color: super::TextStyleConfigColor::Default,
                font_type: super::TextStyleConfigFontType::Default,
                is_subtle: false,
                size: super::TextStyleConfigSize::Large,
                weight: super::TextStyleConfigWeight::Bolder,
            },
        }
    }
    pub(super) fn image_set_config_image_size() -> super::ImageSetConfigImageSize {
        super::ImageSetConfigImageSize::Medium
    }
    pub(super) fn input_label_config_color() -> super::InputLabelConfigColor {
        super::InputLabelConfigColor::Default
    }
    pub(super) fn input_label_config_size() -> super::InputLabelConfigSize {
        super::InputLabelConfigSize::Default
    }
    pub(super) fn input_label_config_weight() -> super::InputLabelConfigWeight {
        super::InputLabelConfigWeight::Default
    }
    pub(super) fn inputs_config_error_message() -> super::ErrorMessageConfig {
        super::ErrorMessageConfig {
            size: super::ErrorMessageConfigSize::Default,
            spacing: super::ErrorMessageConfigSpacing::Default,
            weight: super::ErrorMessageConfigWeight::Default,
        }
    }
    pub(super) fn inputs_config_label() -> super::LabelConfig {
        super::LabelConfig {
            input_spacing: super::LabelConfigInputSpacing::Default,
            optional_inputs: super::InputLabelConfig {
                color: super::InputLabelConfigColor::Default,
                is_subtle: false,
                size: super::InputLabelConfigSize::Default,
                suffix: Default::default(),
                weight: super::InputLabelConfigWeight::Default,
            },
            required_inputs: super::InputLabelConfig {
                color: super::InputLabelConfigColor::Default,
                is_subtle: false,
                size: super::InputLabelConfigSize::Default,
                suffix: Default::default(),
                weight: super::InputLabelConfigWeight::Default,
            },
        }
    }
    pub(super) fn label_config_input_spacing() -> super::LabelConfigInputSpacing {
        super::LabelConfigInputSpacing::Default
    }
    pub(super) fn label_config_optional_inputs() -> super::InputLabelConfig {
        super::InputLabelConfig {
            color: super::InputLabelConfigColor::Default,
            is_subtle: false,
            size: super::InputLabelConfigSize::Default,
            suffix: Default::default(),
            weight: super::InputLabelConfigWeight::Default,
        }
    }
    pub(super) fn label_config_required_inputs() -> super::InputLabelConfig {
        super::InputLabelConfig {
            color: super::InputLabelConfigColor::Default,
            is_subtle: false,
            size: super::InputLabelConfigSize::Default,
            suffix: Default::default(),
            weight: super::InputLabelConfigWeight::Default,
        }
    }
    pub(super) fn separator_config_line_color() -> String {
        "#EEEEEE".to_string()
    }
    pub(super) fn show_card_config_action_mode() -> super::ShowCardConfigActionMode {
        super::ShowCardConfigActionMode::Inline
    }
    pub(super) fn show_card_config_style() -> super::ShowCardConfigStyle {
        super::ShowCardConfigStyle::Emphasis
    }
    pub(super) fn text_style_config_color() -> super::TextStyleConfigColor {
        super::TextStyleConfigColor::Default
    }
    pub(super) fn text_style_config_font_type() -> super::TextStyleConfigFontType {
        super::TextStyleConfigFontType::Default
    }
    pub(super) fn text_style_config_size() -> super::TextStyleConfigSize {
        super::TextStyleConfigSize::Default
    }
    pub(super) fn text_style_config_weight() -> super::TextStyleConfigWeight {
        super::TextStyleConfigWeight::Default
    }
    pub(super) fn text_styles_config_column_header() -> super::TextStyleConfig {
        super::TextStyleConfig {
            color: super::TextStyleConfigColor::Default,
            font_type: super::TextStyleConfigFontType::Default,
            is_subtle: false,
            size: super::TextStyleConfigSize::Default,
            weight: super::TextStyleConfigWeight::Bolder,
        }
    }
    pub(super) fn text_styles_config_heading() -> super::TextStyleConfig {
        super::TextStyleConfig {
            color: super::TextStyleConfigColor::Default,
            font_type: super::TextStyleConfigFontType::Default,
            is_subtle: false,
            size: super::TextStyleConfigSize::Large,
            weight: super::TextStyleConfigWeight::Bolder,
        }
    }
}
