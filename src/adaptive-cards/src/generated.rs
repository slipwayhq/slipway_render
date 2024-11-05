#![allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionMode {
    #[serde(rename = "primary", alias = "Primary")]
    Primary,
    #[serde(rename = "secondary", alias = "Secondary")]
    Secondary,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ActionStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "positive", alias = "Positive")]
    Positive,
    #[serde(rename = "destructive", alias = "Destructive")]
    Destructive,
}
impl Default for ActionStyle {
    fn default() -> Self {
        ActionStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AssociatedInputs {
    #[serde(rename = "Auto", alias = "auto")]
    Auto,
    #[serde(rename = "None", alias = "none")]
    None,
}
impl Default for AssociatedInputs {
    fn default() -> Self {
        AssociatedInputs::Auto
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockElementHeight {
    #[serde(rename = "auto", alias = "Auto")]
    Auto,
    #[serde(rename = "stretch", alias = "Stretch")]
    Stretch,
}
impl Default for BlockElementHeight {
    fn default() -> Self {
        BlockElementHeight::Auto
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChoiceInputStyle {
    #[serde(rename = "compact", alias = "Compact")]
    Compact,
    #[serde(rename = "expanded", alias = "Expanded")]
    Expanded,
    #[serde(rename = "filtered", alias = "Filtered")]
    Filtered,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Colors {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "dark", alias = "Dark")]
    Dark,
    #[serde(rename = "light", alias = "Light")]
    Light,
    #[serde(rename = "accent", alias = "Accent")]
    Accent,
    #[serde(rename = "good", alias = "Good")]
    Good,
    #[serde(rename = "warning", alias = "Warning")]
    Warning,
    #[serde(rename = "attention", alias = "Attention")]
    Attention,
}
impl Default for Colors {
    fn default() -> Self {
        Colors::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContainerStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "emphasis", alias = "Emphasis")]
    Emphasis,
    #[serde(rename = "good", alias = "Good")]
    Good,
    #[serde(rename = "attention", alias = "Attention")]
    Attention,
    #[serde(rename = "warning", alias = "Warning")]
    Warning,
    #[serde(rename = "accent", alias = "Accent")]
    Accent,
}
impl Default for ContainerStyle {
    fn default() -> Self {
        ContainerStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FallbackOption {
    #[serde(rename = "drop", alias = "Drop")]
    Drop,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FontSize {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "small", alias = "Small")]
    Small,
    #[serde(rename = "medium", alias = "Medium")]
    Medium,
    #[serde(rename = "large", alias = "Large")]
    Large,
    #[serde(rename = "extraLarge", alias = "ExtraLarge")]
    ExtraLarge,
}
impl Default for FontSize {
    fn default() -> Self {
        FontSize::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FontType {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "monospace", alias = "Monospace")]
    Monospace,
}
impl Default for FontType {
    fn default() -> Self {
        FontType::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FontWeight {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "lighter", alias = "Lighter")]
    Lighter,
    #[serde(rename = "bolder", alias = "Bolder")]
    Bolder,
}
impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum HorizontalAlignment {
    #[serde(rename = "left", alias = "Left")]
    Left,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "right", alias = "Right")]
    Right,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageFillMode {
    #[serde(rename = "cover", alias = "Cover")]
    Cover,
    #[serde(rename = "repeatHorizontally", alias = "RepeatHorizontally")]
    RepeatHorizontally,
    #[serde(rename = "repeatVertically", alias = "RepeatVertically")]
    RepeatVertically,
    #[serde(rename = "repeat", alias = "Repeat")]
    Repeat,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageSetStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "stacked", alias = "Stacked")]
    Stacked,
    #[serde(rename = "grid", alias = "Grid")]
    Grid,
}
impl Default for ImageSetStyle {
    fn default() -> Self {
        ImageSetStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageSize {
    #[serde(rename = "auto", alias = "Auto")]
    Auto,
    #[serde(rename = "stretch", alias = "Stretch")]
    Stretch,
    #[serde(rename = "small", alias = "Small")]
    Small,
    #[serde(rename = "medium", alias = "Medium")]
    Medium,
    #[serde(rename = "large", alias = "Large")]
    Large,
}
impl Default for ImageSize {
    fn default() -> Self {
        ImageSize::Auto
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImageStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "person", alias = "Person")]
    Person,
}
impl Default for ImageStyle {
    fn default() -> Self {
        ImageStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputLabelPosition {
    #[serde(rename = "inline", alias = "Inline")]
    Inline,
    #[serde(rename = "above", alias = "Above")]
    Above,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputStyle {
    #[serde(rename = "revealOnHover", alias = "RevealOnHover")]
    RevealOnHover,
    #[serde(rename = "default", alias = "Default")]
    Default,
}
impl Default for InputStyle {
    fn default() -> Self {
        InputStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Spacing {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "none", alias = "None")]
    None,
    #[serde(rename = "small", alias = "Small")]
    Small,
    #[serde(rename = "medium", alias = "Medium")]
    Medium,
    #[serde(rename = "large", alias = "Large")]
    Large,
    #[serde(rename = "extraLarge", alias = "ExtraLarge")]
    ExtraLarge,
    #[serde(rename = "padding", alias = "Padding")]
    Padding,
}
impl Default for Spacing {
    fn default() -> Self {
        Spacing::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextBlockStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "heading", alias = "Heading")]
    Heading,
}
impl Default for TextBlockStyle {
    fn default() -> Self {
        TextBlockStyle::Default
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextInputStyle {
    #[serde(rename = "text", alias = "Text")]
    Text,
    #[serde(rename = "tel", alias = "Tel")]
    Tel,
    #[serde(rename = "url", alias = "Url")]
    Url,
    #[serde(rename = "email", alias = "Email")]
    Email,
    #[serde(rename = "password", alias = "Password")]
    Password,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
}
impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum VerticalContentAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
}
impl Default for VerticalContentAlignment {
    fn default() -> Self {
        VerticalContentAlignment::Top
    }
}
#[derive(serde::Serialize, Clone)]
pub enum StringOrObject {
    String(String),
    Object(serde_json::Value),
}
impl From<String> for StringOrObject {
    fn from(value: String) -> Self {
        StringOrObject::String(value)
    }
}
impl From<serde_json::Value> for StringOrObject {
    fn from(value: serde_json::Value) -> Self {
        StringOrObject::Object(value)
    }
}
#[derive(serde::Serialize, Clone)]
pub enum ActionOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    Action(Box<Action<TLayoutData>>),
    FallbackOption(FallbackOption),
    _Phantom(std::marker::PhantomData<TLayoutData>),
}
impl<TLayoutData> From<Action<TLayoutData>> for ActionOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: Action<TLayoutData>) -> Self {
        ActionOrFallbackOption::<TLayoutData>::Action(Box::new(value))
    }
}
impl<TLayoutData> From<FallbackOption> for ActionOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: FallbackOption) -> Self {
        ActionOrFallbackOption::<TLayoutData>::FallbackOption(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionExecute<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(
        rename = "associatedInputs",
        default = "ActionExecute::<TLayoutData>::default_value_for_associated_inputs",
        skip_serializing_if = "ActionExecute::<TLayoutData>::is_default_value_for_associated_inputs"
    )]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption<TLayoutData>>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionExecute::<TLayoutData>::default_value_for_is_enabled",
        skip_serializing_if = "ActionExecute::<TLayoutData>::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionExecute::<TLayoutData>::default_value_for_mode",
        skip_serializing_if = "ActionExecute::<TLayoutData>::is_default_value_for_mode"
    )]
    pub mode: ActionMode,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "verb", skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionExecute<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_associated_inputs() -> AssociatedInputs {
        AssociatedInputs::Auto
    }
    fn is_default_value_for_associated_inputs(value: &AssociatedInputs) -> bool {
        matches!(* value, AssociatedInputs::Auto)
    }
}
impl<TLayoutData> ActionExecute<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> ActionExecute<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ActionExecute<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionOpenUrl<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption<TLayoutData>>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionOpenUrl::<TLayoutData>::default_value_for_is_enabled",
        skip_serializing_if = "ActionOpenUrl::<TLayoutData>::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionOpenUrl::<TLayoutData>::default_value_for_mode",
        skip_serializing_if = "ActionOpenUrl::<TLayoutData>::is_default_value_for_mode"
    )]
    pub mode: ActionMode,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionOpenUrl<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> ActionOpenUrl<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ActionOpenUrl<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionShowCard<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<AdaptiveCard<TLayoutData>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption<TLayoutData>>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionShowCard::<TLayoutData>::default_value_for_is_enabled",
        skip_serializing_if = "ActionShowCard::<TLayoutData>::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionShowCard::<TLayoutData>::default_value_for_mode",
        skip_serializing_if = "ActionShowCard::<TLayoutData>::is_default_value_for_mode"
    )]
    pub mode: ActionMode,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionShowCard<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> ActionShowCard<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ActionShowCard<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionSubmit<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(
        rename = "associatedInputs",
        default = "ActionSubmit::<TLayoutData>::default_value_for_associated_inputs",
        skip_serializing_if = "ActionSubmit::<TLayoutData>::is_default_value_for_associated_inputs"
    )]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption<TLayoutData>>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionSubmit::<TLayoutData>::default_value_for_is_enabled",
        skip_serializing_if = "ActionSubmit::<TLayoutData>::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionSubmit::<TLayoutData>::default_value_for_mode",
        skip_serializing_if = "ActionSubmit::<TLayoutData>::is_default_value_for_mode"
    )]
    pub mode: ActionMode,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionSubmit<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_associated_inputs() -> AssociatedInputs {
        AssociatedInputs::Auto
    }
    fn is_default_value_for_associated_inputs(value: &AssociatedInputs) -> bool {
        matches!(* value, AssociatedInputs::Auto)
    }
}
impl<TLayoutData> ActionSubmit<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> ActionSubmit<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ActionSubmit<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Serialize, Clone)]
pub enum TargetElementOrString {
    TargetElement(TargetElement),
    String(String),
}
impl From<TargetElement> for TargetElementOrString {
    fn from(value: TargetElement) -> Self {
        TargetElementOrString::TargetElement(value)
    }
}
impl From<String> for TargetElementOrString {
    fn from(value: String) -> Self {
        TargetElementOrString::String(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionToggleVisibility<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption<TLayoutData>>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionToggleVisibility::<TLayoutData>::default_value_for_is_enabled",
        skip_serializing_if = "ActionToggleVisibility::<TLayoutData>::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionToggleVisibility::<TLayoutData>::default_value_for_mode",
        skip_serializing_if = "ActionToggleVisibility::<TLayoutData>::is_default_value_for_mode"
    )]
    pub mode: ActionMode,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "targetElements")]
    pub target_elements: Vec<TargetElementOrString>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionToggleVisibility<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> ActionToggleVisibility<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData>
for ActionToggleVisibility<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Action<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute<TLayoutData>>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl<TLayoutData>>),
    #[serde(rename = "Action.ShowCard")]
    ShowCard(Box<ActionShowCard<TLayoutData>>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit<TLayoutData>>),
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility<TLayoutData>>),
}
impl<TLayoutData> Action<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            Action::Execute(inner) => inner,
            Action::OpenUrl(inner) => inner,
            Action::ShowCard(inner) => inner,
            Action::Submit(inner) => inner,
            Action::ToggleVisibility(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Action<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Serialize, Clone)]
pub enum ElementOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    Element(Box<Element<TLayoutData>>),
    FallbackOption(FallbackOption),
    _Phantom(std::marker::PhantomData<TLayoutData>),
}
impl<TLayoutData> From<Element<TLayoutData>> for ElementOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: Element<TLayoutData>) -> Self {
        ElementOrFallbackOption::<TLayoutData>::Element(Box::new(value))
    }
}
impl<TLayoutData> From<FallbackOption> for ElementOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: FallbackOption) -> Self {
        ElementOrFallbackOption::<TLayoutData>::FallbackOption(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionSet<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "actions")]
    pub actions: Vec<Action<TLayoutData>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "ActionSet::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "ActionSet::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ActionSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ActionSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for ActionSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Serialize, Clone)]
pub enum BackgroundImageOrString {
    BackgroundImage(BackgroundImage),
    String(String),
}
impl From<BackgroundImage> for BackgroundImageOrString {
    fn from(value: BackgroundImage) -> Self {
        BackgroundImageOrString::BackgroundImage(value)
    }
}
impl From<String> for BackgroundImageOrString {
    fn from(value: String) -> Self {
        BackgroundImageOrString::String(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveCard<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action<TLayoutData>>>,
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<Element<TLayoutData>>>,
    #[serde(rename = "fallbackText", skip_serializing_if = "Option::is_none")]
    pub fallback_text: Option<String>,
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<Refresh<TLayoutData>>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "speak", skip_serializing_if = "Option::is_none")]
    pub speak: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        rename = "verticalContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for AdaptiveCard<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AuthCardButton {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Authentication {
    #[serde(rename = "buttons", skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<AuthCardButton>>,
    #[serde(rename = "connectionName", skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "tokenExchangeResource", skip_serializing_if = "Option::is_none")]
    pub token_exchange_resource: Option<TokenExchangeResource>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BackgroundImage {
    #[serde(rename = "fillMode", skip_serializing_if = "Option::is_none")]
    pub fill_mode: Option<ImageFillMode>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "verticalAlignment", skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<VerticalAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CaptionSource {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Serialize, Clone)]
pub enum ColumnOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    Column(Box<Column<TLayoutData>>),
    FallbackOption(FallbackOption),
    _Phantom(std::marker::PhantomData<TLayoutData>),
}
impl<TLayoutData> From<Column<TLayoutData>> for ColumnOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: Column<TLayoutData>) -> Self {
        ColumnOrFallbackOption::<TLayoutData>::Column(Box::new(value))
    }
}
impl<TLayoutData> From<FallbackOption> for ColumnOrFallbackOption<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: FallbackOption) -> Self {
        ColumnOrFallbackOption::<TLayoutData>::FallbackOption(value)
    }
}
#[derive(serde::Serialize, Clone)]
pub enum StringOrNumber {
    String(String),
    Number(f64),
}
impl From<String> for StringOrNumber {
    fn from(value: String) -> Self {
        StringOrNumber::String(value)
    }
}
impl From<f64> for StringOrNumber {
    fn from(value: f64) -> Self {
        StringOrNumber::Number(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Column<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ColumnOrFallbackOption<TLayoutData>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Column::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "Column::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Element<TLayoutData>>>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(
        rename = "verticalContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<StringOrNumber>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> Column<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Column<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColumnSet<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column<TLayoutData>>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "ColumnSet::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "ColumnSet::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ColumnSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ColumnSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for ColumnSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Container<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Container::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "Container::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "items")]
    pub items: Vec<Element<TLayoutData>>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(
        rename = "verticalContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> Container<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Container<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for Container<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DataQuery {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "dataset")]
    pub dataset: String,
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Element<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet<TLayoutData>>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet<TLayoutData>>),
    #[serde(rename = "Container")]
    Container(Box<Container<TLayoutData>>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet<TLayoutData>>),
    #[serde(rename = "Image")]
    Image(Box<Image<TLayoutData>>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet<TLayoutData>>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet<TLayoutData>>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate<TLayoutData>>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber<TLayoutData>>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText<TLayoutData>>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime<TLayoutData>>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle<TLayoutData>>),
    #[serde(rename = "Input")]
    Input(Box<Input<TLayoutData>>),
    #[serde(rename = "Media")]
    Media(Box<Media<TLayoutData>>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock<TLayoutData>>),
    #[serde(rename = "Table")]
    Table(Box<Table<TLayoutData>>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock<TLayoutData>>),
}
impl<TLayoutData> Element<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            Element::ActionSet(inner) => inner,
            Element::ColumnSet(inner) => inner,
            Element::Container(inner) => inner,
            Element::FactSet(inner) => inner,
            Element::Image(inner) => inner,
            Element::ImageSet(inner) => inner,
            Element::InputChoiceSet(inner) => inner,
            Element::InputDate(inner) => inner,
            Element::InputNumber(inner) => inner,
            Element::InputText(inner) => inner,
            Element::InputTime(inner) => inner,
            Element::InputToggle(inner) => inner,
            Element::Input(inner) => inner.as_has_layout_data(),
            Element::Media(inner) => inner,
            Element::RichTextBlock(inner) => inner,
            Element::Table(inner) => inner,
            Element::TextBlock(inner) => inner,
        }
    }
}
impl<TLayoutData> Element<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_element(&self) -> &dyn crate::LayoutableElement {
        match self {
            Element::ActionSet(inner) => inner,
            Element::ColumnSet(inner) => inner,
            Element::Container(inner) => inner,
            Element::FactSet(inner) => inner,
            Element::Image(inner) => inner,
            Element::ImageSet(inner) => inner,
            Element::InputChoiceSet(inner) => inner,
            Element::InputDate(inner) => inner,
            Element::InputNumber(inner) => inner,
            Element::InputText(inner) => inner,
            Element::InputTime(inner) => inner,
            Element::InputToggle(inner) => inner,
            Element::Input(inner) => inner.as_element(),
            Element::Media(inner) => inner,
            Element::RichTextBlock(inner) => inner,
            Element::Table(inner) => inner,
            Element::TextBlock(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Element<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FactSet<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "facts")]
    pub facts: Vec<Fact>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "FactSet::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "FactSet::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> FactSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for FactSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for FactSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum ISelectAction<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute<TLayoutData>>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl<TLayoutData>>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit<TLayoutData>>),
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility<TLayoutData>>),
}
impl<TLayoutData> ISelectAction<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            ISelectAction::Execute(inner) => inner,
            ISelectAction::OpenUrl(inner) => inner,
            ISelectAction::Submit(inner) => inner,
            ISelectAction::ToggleVisibility(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ISelectAction<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Serialize, Clone)]
pub enum StringOrBlockElementHeight {
    String(String),
    BlockElementHeight(BlockElementHeight),
}
impl From<String> for StringOrBlockElementHeight {
    fn from(value: String) -> Self {
        StringOrBlockElementHeight::String(value)
    }
}
impl From<BlockElementHeight> for StringOrBlockElementHeight {
    fn from(value: BlockElementHeight) -> Self {
        StringOrBlockElementHeight::BlockElementHeight(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Image<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(
        rename = "height",
        default = "Image::<TLayoutData>::default_value_for_height",
        skip_serializing_if = "Image::<TLayoutData>::is_default_value_for_height"
    )]
    pub height: StringOrBlockElementHeight,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Image::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "Image::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> Image<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_height() -> StringOrBlockElementHeight {
        StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto)
    }
    fn is_default_value_for_height(value: &StringOrBlockElementHeight) -> bool {
        matches!(
            * value,
            StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto)
        )
    }
}
impl<TLayoutData> Image<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Image<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for Image<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height.clone()
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ImageSet<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "imageSize",
        default = "ImageSet::<TLayoutData>::default_value_for_image_size",
        skip_serializing_if = "ImageSet::<TLayoutData>::is_default_value_for_image_size"
    )]
    pub image_size: ImageSize,
    #[serde(rename = "images")]
    pub images: Vec<Image<TLayoutData>>,
    #[serde(
        rename = "isVisible",
        default = "ImageSet::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "ImageSet::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageSetStyle>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> ImageSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_image_size() -> ImageSize {
        ImageSize::Medium
    }
    fn is_default_value_for_image_size(value: &ImageSize) -> bool {
        matches!(* value, ImageSize::Medium)
    }
}
impl<TLayoutData> ImageSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ImageSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for ImageSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Inline<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "TextRun")]
    TextRun(Box<TextRun<TLayoutData>>),
}
impl<TLayoutData> Inline<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            Inline::TextRun(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Inline<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputChoice {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputChoiceSet<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<InputChoice>>,
    #[serde(rename = "choices.data", skip_serializing_if = "Option::is_none")]
    pub choices_data: Option<DataQuery>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(
        rename = "isMultiSelect",
        default = "InputChoiceSet::<TLayoutData>::default_value_for_is_multi_select",
        skip_serializing_if = "InputChoiceSet::<TLayoutData>::is_default_value_for_is_multi_select"
    )]
    pub is_multi_select: bool,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputChoiceSet::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputChoiceSet::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ChoiceInputStyle>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputChoiceSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_multi_select() -> bool {
        false
    }
    fn is_default_value_for_is_multi_select(value: &bool) -> bool {
        *value == false
    }
}
impl<TLayoutData> InputChoiceSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputChoiceSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputChoiceSet<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputDate<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputDate::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputDate::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputDate<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputDate<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputDate<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputNumber<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputNumber::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputNumber::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputNumber<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputNumber<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputNumber<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputText<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inlineAction", skip_serializing_if = "Option::is_none")]
    pub inline_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(
        rename = "isMultiline",
        default = "InputText::<TLayoutData>::default_value_for_is_multiline",
        skip_serializing_if = "InputText::<TLayoutData>::is_default_value_for_is_multiline"
    )]
    pub is_multiline: bool,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputText::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputText::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<f64>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "regex", skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<TextInputStyle>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputText<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_multiline() -> bool {
        false
    }
    fn is_default_value_for_is_multiline(value: &bool) -> bool {
        *value == false
    }
}
impl<TLayoutData> InputText<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputText<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputText<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputTime<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputTime::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputTime::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(rename = "placeholder", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputTime<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputTime<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputTime<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputToggle::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "InputToggle::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition", skip_serializing_if = "Option::is_none")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth", skip_serializing_if = "Option::is_none")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(
        rename = "value",
        default = "InputToggle::<TLayoutData>::default_value_for_value",
        skip_serializing_if = "InputToggle::<TLayoutData>::is_default_value_for_value"
    )]
    pub value: String,
    #[serde(
        rename = "valueOff",
        default = "InputToggle::<TLayoutData>::default_value_for_value_off",
        skip_serializing_if = "InputToggle::<TLayoutData>::is_default_value_for_value_off"
    )]
    pub value_off: String,
    #[serde(
        rename = "valueOn",
        default = "InputToggle::<TLayoutData>::default_value_for_value_on",
        skip_serializing_if = "InputToggle::<TLayoutData>::is_default_value_for_value_on"
    )]
    pub value_on: String,
    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_value() -> String {
        String::from("false")
    }
    fn is_default_value_for_value(value: &String) -> bool {
        *value == "false"
    }
}
impl<TLayoutData> InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_value_off() -> String {
        String::from("false")
    }
    fn is_default_value_for_value_off(value: &String) -> bool {
        *value == "false"
    }
}
impl<TLayoutData> InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_value_on() -> String {
        String::from("true")
    }
    fn is_default_value_for_value_on(value: &String) -> bool {
        *value == "true"
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for InputToggle<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Input<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "Input.ChoiceSet")]
    ChoiceSet(Box<InputChoiceSet<TLayoutData>>),
    #[serde(rename = "Input.Date")]
    Date(Box<InputDate<TLayoutData>>),
    #[serde(rename = "Input.Number")]
    Number(Box<InputNumber<TLayoutData>>),
    #[serde(rename = "Input.Text")]
    Text(Box<InputText<TLayoutData>>),
    #[serde(rename = "Input.Time")]
    Time(Box<InputTime<TLayoutData>>),
    #[serde(rename = "Input.Toggle")]
    Toggle(Box<InputToggle<TLayoutData>>),
}
impl<TLayoutData> Input<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            Input::ChoiceSet(inner) => inner,
            Input::Date(inner) => inner,
            Input::Number(inner) => inner,
            Input::Text(inner) => inner,
            Input::Time(inner) => inner,
            Input::Toggle(inner) => inner,
        }
    }
}
impl<TLayoutData> Input<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_element(&self) -> &dyn crate::LayoutableElement {
        match self {
            Input::ChoiceSet(inner) => inner,
            Input::Date(inner) => inner,
            Input::Number(inner) => inner,
            Input::Text(inner) => inner,
            Input::Time(inner) => inner,
            Input::Toggle(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Input<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Item<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "Action.Execute")]
    ActionExecute(Box<ActionExecute<TLayoutData>>),
    #[serde(rename = "Action.OpenUrl")]
    ActionOpenUrl(Box<ActionOpenUrl<TLayoutData>>),
    #[serde(rename = "Action.ShowCard")]
    ActionShowCard(Box<ActionShowCard<TLayoutData>>),
    #[serde(rename = "Action.Submit")]
    ActionSubmit(Box<ActionSubmit<TLayoutData>>),
    #[serde(rename = "Action.ToggleVisibility")]
    ActionToggleVisibility(Box<ActionToggleVisibility<TLayoutData>>),
    #[serde(rename = "Action")]
    Action(Box<Action<TLayoutData>>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet<TLayoutData>>),
    #[serde(rename = "Column")]
    Column(Box<Column<TLayoutData>>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet<TLayoutData>>),
    #[serde(rename = "Container")]
    Container(Box<Container<TLayoutData>>),
    #[serde(rename = "Element")]
    Element(Box<Element<TLayoutData>>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet<TLayoutData>>),
    #[serde(rename = "Image")]
    Image(Box<Image<TLayoutData>>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet<TLayoutData>>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet<TLayoutData>>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate<TLayoutData>>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber<TLayoutData>>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText<TLayoutData>>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime<TLayoutData>>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle<TLayoutData>>),
    #[serde(rename = "Input")]
    Input(Box<Input<TLayoutData>>),
    #[serde(rename = "Media")]
    Media(Box<Media<TLayoutData>>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock<TLayoutData>>),
    #[serde(rename = "Table")]
    Table(Box<Table<TLayoutData>>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock<TLayoutData>>),
    #[serde(rename = "ToggleableItem")]
    ToggleableItem(Box<ToggleableItem<TLayoutData>>),
}
impl<TLayoutData> Item<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            Item::ActionExecute(inner) => inner,
            Item::ActionOpenUrl(inner) => inner,
            Item::ActionShowCard(inner) => inner,
            Item::ActionSubmit(inner) => inner,
            Item::ActionToggleVisibility(inner) => inner,
            Item::Action(inner) => inner.as_has_layout_data(),
            Item::ActionSet(inner) => inner,
            Item::Column(inner) => inner,
            Item::ColumnSet(inner) => inner,
            Item::Container(inner) => inner,
            Item::Element(inner) => inner.as_has_layout_data(),
            Item::FactSet(inner) => inner,
            Item::Image(inner) => inner,
            Item::ImageSet(inner) => inner,
            Item::InputChoiceSet(inner) => inner,
            Item::InputDate(inner) => inner,
            Item::InputNumber(inner) => inner,
            Item::InputText(inner) => inner,
            Item::InputTime(inner) => inner,
            Item::InputToggle(inner) => inner,
            Item::Input(inner) => inner.as_has_layout_data(),
            Item::Media(inner) => inner,
            Item::RichTextBlock(inner) => inner,
            Item::Table(inner) => inner,
            Item::TextBlock(inner) => inner,
            Item::ToggleableItem(inner) => inner.as_has_layout_data(),
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Item<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Media<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "captionSources", skip_serializing_if = "Option::is_none")]
    pub caption_sources: Option<Vec<CaptionSource>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Media::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "Media::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "poster", skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "sources")]
    pub sources: Vec<MediaSource>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> Media<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Media<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for Media<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MediaSource {
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "webUrl", skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Refresh<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionExecute<TLayoutData>>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Refresh<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Serialize, Clone)]
pub enum InlineOrString<TLayoutData>
where
    TLayoutData: Default,
{
    Inline(Inline<TLayoutData>),
    String(String),
    _Phantom(std::marker::PhantomData<TLayoutData>),
}
impl<TLayoutData> From<Inline<TLayoutData>> for InlineOrString<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: Inline<TLayoutData>) -> Self {
        InlineOrString::<TLayoutData>::Inline(value)
    }
}
impl<TLayoutData> From<String> for InlineOrString<TLayoutData>
where
    TLayoutData: Default,
{
    fn from(value: String) -> Self {
        InlineOrString::<TLayoutData>::String(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RichTextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inlines")]
    pub inlines: Vec<InlineOrString<TLayoutData>>,
    #[serde(
        rename = "isVisible",
        default = "RichTextBlock::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "RichTextBlock::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> RichTextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for RichTextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for RichTextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Table<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableColumnDefinition>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(
        rename = "firstRowAsHeader",
        default = "Table::<TLayoutData>::default_value_for_first_row_as_header",
        skip_serializing_if = "Table::<TLayoutData>::is_default_value_for_first_row_as_header"
    )]
    pub first_row_as_header: bool,
    #[serde(
        rename = "gridStyle",
        default = "Table::<TLayoutData>::default_value_for_grid_style",
        skip_serializing_if = "Table::<TLayoutData>::is_default_value_for_grid_style"
    )]
    pub grid_style: ContainerStyle,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(
        rename = "horizontalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Table::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "Table::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow<TLayoutData>>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(
        rename = "showGridLines",
        default = "Table::<TLayoutData>::default_value_for_show_grid_lines",
        skip_serializing_if = "Table::<TLayoutData>::is_default_value_for_show_grid_lines"
    )]
    pub show_grid_lines: bool,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(
        rename = "verticalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_first_row_as_header() -> bool {
        true
    }
    fn is_default_value_for_first_row_as_header(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_grid_style() -> ContainerStyle {
        ContainerStyle::Default
    }
    fn is_default_value_for_grid_style(value: &ContainerStyle) -> bool {
        matches!(* value, ContainerStyle::Default)
    }
}
impl<TLayoutData> Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_show_grid_lines() -> bool {
        true
    }
    fn is_default_value_for_show_grid_lines(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for Table<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "items")]
    pub items: Vec<Element<TLayoutData>>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(
        rename = "verticalContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for TableCell<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TableColumnDefinition {
    #[serde(
        rename = "horizontalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(
        rename = "verticalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(
        rename = "width",
        default = "TableColumnDefinition::default_value_for_width",
        skip_serializing_if = "TableColumnDefinition::is_default_value_for_width"
    )]
    pub width: StringOrNumber,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TableColumnDefinition {
    fn default_value_for_width() -> StringOrNumber {
        StringOrNumber::Number(1f64)
    }
    fn is_default_value_for_width(value: &StringOrNumber) -> bool {
        matches!(* value, StringOrNumber::Number(1f64))
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "cells", skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<TableCell<TLayoutData>>>,
    #[serde(
        rename = "horizontalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(
        rename = "verticalCellContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for TableRow<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TargetElement {
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "isVisible", skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Colors>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption<TLayoutData>>,
    #[serde(rename = "fontType", skip_serializing_if = "Option::is_none")]
    pub font_type: Option<FontType>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isSubtle",
        default = "TextBlock::<TLayoutData>::default_value_for_is_subtle",
        skip_serializing_if = "TextBlock::<TLayoutData>::is_default_value_for_is_subtle"
    )]
    pub is_subtle: bool,
    #[serde(
        rename = "isVisible",
        default = "TextBlock::<TLayoutData>::default_value_for_is_visible",
        skip_serializing_if = "TextBlock::<TLayoutData>::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "maxLines", skip_serializing_if = "Option::is_none")]
    pub max_lines: Option<f64>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<FontSize>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(
        rename = "style",
        default = "TextBlock::<TLayoutData>::default_value_for_style",
        skip_serializing_if = "TextBlock::<TLayoutData>::is_default_value_for_style"
    )]
    pub style: TextBlockStyle,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<FontWeight>,
    #[serde(
        rename = "wrap",
        default = "TextBlock::<TLayoutData>::default_value_for_wrap",
        skip_serializing_if = "TextBlock::<TLayoutData>::is_default_value_for_wrap"
    )]
    pub wrap: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_subtle() -> bool {
        false
    }
    fn is_default_value_for_is_subtle(value: &bool) -> bool {
        *value == false
    }
}
impl<TLayoutData> TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl<TLayoutData> TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_style() -> TextBlockStyle {
        TextBlockStyle::Default
    }
    fn is_default_value_for_style(value: &TextBlockStyle) -> bool {
        matches!(* value, TextBlockStyle::Default)
    }
}
impl<TLayoutData> TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_wrap() -> bool {
        false
    }
    fn is_default_value_for_wrap(value: &bool) -> bool {
        *value == false
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
impl<TLayoutData> crate::LayoutableElement for TextBlock<TLayoutData>
where
    TLayoutData: Default,
{
    fn get_height(&self) -> StringOrBlockElementHeight {
        self.height
            .map(StringOrBlockElementHeight::BlockElementHeight)
            .unwrap_or(
                StringOrBlockElementHeight::BlockElementHeight(BlockElementHeight::Auto),
            )
    }
    fn get_separator(&self) -> bool {
        self.separator.unwrap_or(false)
    }
    fn get_spacing(&self) -> Spacing {
        self.spacing.unwrap_or(Spacing::Default)
    }
    fn get_is_visible(&self) -> bool {
        self.is_visible
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextRun<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Colors>,
    #[serde(rename = "fontType", skip_serializing_if = "Option::is_none")]
    pub font_type: Option<FontType>,
    #[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    #[serde(
        rename = "isSubtle",
        default = "TextRun::<TLayoutData>::default_value_for_is_subtle",
        skip_serializing_if = "TextRun::<TLayoutData>::is_default_value_for_is_subtle"
    )]
    pub is_subtle: bool,
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction<TLayoutData>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<FontSize>,
    #[serde(rename = "strikethrough", skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "underline", skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<FontWeight>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<TLayoutData>,
}
impl<TLayoutData> TextRun<TLayoutData>
where
    TLayoutData: Default,
{
    fn default_value_for_is_subtle() -> bool {
        false
    }
    fn is_default_value_for_is_subtle(value: &bool) -> bool {
        *value == false
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for TextRun<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        &self.layout_data
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum ToggleableItem<TLayoutData>
where
    TLayoutData: Default,
{
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet<TLayoutData>>),
    #[serde(rename = "Column")]
    Column(Box<Column<TLayoutData>>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet<TLayoutData>>),
    #[serde(rename = "Container")]
    Container(Box<Container<TLayoutData>>),
    #[serde(rename = "Element")]
    Element(Box<Element<TLayoutData>>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet<TLayoutData>>),
    #[serde(rename = "Image")]
    Image(Box<Image<TLayoutData>>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet<TLayoutData>>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet<TLayoutData>>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate<TLayoutData>>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber<TLayoutData>>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText<TLayoutData>>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime<TLayoutData>>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle<TLayoutData>>),
    #[serde(rename = "Input")]
    Input(Box<Input<TLayoutData>>),
    #[serde(rename = "Media")]
    Media(Box<Media<TLayoutData>>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock<TLayoutData>>),
    #[serde(rename = "Table")]
    Table(Box<Table<TLayoutData>>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock<TLayoutData>>),
}
impl<TLayoutData> ToggleableItem<TLayoutData>
where
    TLayoutData: Default,
{
    pub fn as_has_layout_data(&self) -> &dyn crate::HasLayoutData<TLayoutData> {
        match self {
            ToggleableItem::ActionSet(inner) => inner,
            ToggleableItem::Column(inner) => inner,
            ToggleableItem::ColumnSet(inner) => inner,
            ToggleableItem::Container(inner) => inner,
            ToggleableItem::Element(inner) => inner.as_has_layout_data(),
            ToggleableItem::FactSet(inner) => inner,
            ToggleableItem::Image(inner) => inner,
            ToggleableItem::ImageSet(inner) => inner,
            ToggleableItem::InputChoiceSet(inner) => inner,
            ToggleableItem::InputDate(inner) => inner,
            ToggleableItem::InputNumber(inner) => inner,
            ToggleableItem::InputText(inner) => inner,
            ToggleableItem::InputTime(inner) => inner,
            ToggleableItem::InputToggle(inner) => inner,
            ToggleableItem::Input(inner) => inner.as_has_layout_data(),
            ToggleableItem::Media(inner) => inner,
            ToggleableItem::RichTextBlock(inner) => inner,
            ToggleableItem::Table(inner) => inner,
            ToggleableItem::TextBlock(inner) => inner,
        }
    }
}
impl<TLayoutData> crate::HasLayoutData<TLayoutData> for ToggleableItem<TLayoutData>
where
    TLayoutData: Default,
{
    fn layout_data(&self) -> &core::cell::RefCell<TLayoutData> {
        self.as_has_layout_data().layout_data()
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TokenExchangeResource {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[macro_export]
macro_rules! impl_as_trait {
    ($trait_name:path, $as_trait_name:ident, $method_name:ident, $layout_data:ident) => {
        pub (crate) trait $as_trait_name { fn $method_name (& self) -> & dyn $trait_name;
        } impl $as_trait_name for adaptive_cards::Action < $layout_data > { fn
        $method_name (& self) -> & dyn $trait_name { match self {
        adaptive_cards::Action::Execute(inner) => inner,
        adaptive_cards::Action::OpenUrl(inner) => inner,
        adaptive_cards::Action::ShowCard(inner) => inner,
        adaptive_cards::Action::Submit(inner) => inner,
        adaptive_cards::Action::ToggleVisibility(inner) => inner, } } } impl
        $as_trait_name for adaptive_cards::Element < $layout_data > { fn $method_name (&
        self) -> & dyn $trait_name { match self {
        adaptive_cards::Element::ActionSet(inner) => inner,
        adaptive_cards::Element::ColumnSet(inner) => inner,
        adaptive_cards::Element::Container(inner) => inner,
        adaptive_cards::Element::FactSet(inner) => inner,
        adaptive_cards::Element::Image(inner) => inner,
        adaptive_cards::Element::ImageSet(inner) => inner,
        adaptive_cards::Element::InputChoiceSet(inner) => inner,
        adaptive_cards::Element::InputDate(inner) => inner,
        adaptive_cards::Element::InputNumber(inner) => inner,
        adaptive_cards::Element::InputText(inner) => inner,
        adaptive_cards::Element::InputTime(inner) => inner,
        adaptive_cards::Element::InputToggle(inner) => inner,
        adaptive_cards::Element::Input(inner) => inner. $method_name (),
        adaptive_cards::Element::Media(inner) => inner,
        adaptive_cards::Element::RichTextBlock(inner) => inner,
        adaptive_cards::Element::Table(inner) => inner,
        adaptive_cards::Element::TextBlock(inner) => inner, } } } impl $as_trait_name for
        adaptive_cards::ISelectAction < $layout_data > { fn $method_name (& self) -> &
        dyn $trait_name { match self { adaptive_cards::ISelectAction::Execute(inner) =>
        inner, adaptive_cards::ISelectAction::OpenUrl(inner) => inner,
        adaptive_cards::ISelectAction::Submit(inner) => inner,
        adaptive_cards::ISelectAction::ToggleVisibility(inner) => inner, } } } impl
        $as_trait_name for adaptive_cards::Inline < $layout_data > { fn $method_name (&
        self) -> & dyn $trait_name { match self { adaptive_cards::Inline::TextRun(inner)
        => inner, } } } impl $as_trait_name for adaptive_cards::Input < $layout_data > {
        fn $method_name (& self) -> & dyn $trait_name { match self {
        adaptive_cards::Input::ChoiceSet(inner) => inner,
        adaptive_cards::Input::Date(inner) => inner, adaptive_cards::Input::Number(inner)
        => inner, adaptive_cards::Input::Text(inner) => inner,
        adaptive_cards::Input::Time(inner) => inner, adaptive_cards::Input::Toggle(inner)
        => inner, } } } impl $as_trait_name for adaptive_cards::Item < $layout_data > {
        fn $method_name (& self) -> & dyn $trait_name { match self {
        adaptive_cards::Item::ActionExecute(inner) => inner,
        adaptive_cards::Item::ActionOpenUrl(inner) => inner,
        adaptive_cards::Item::ActionShowCard(inner) => inner,
        adaptive_cards::Item::ActionSubmit(inner) => inner,
        adaptive_cards::Item::ActionToggleVisibility(inner) => inner,
        adaptive_cards::Item::Action(inner) => inner. $method_name (),
        adaptive_cards::Item::ActionSet(inner) => inner,
        adaptive_cards::Item::Column(inner) => inner,
        adaptive_cards::Item::ColumnSet(inner) => inner,
        adaptive_cards::Item::Container(inner) => inner,
        adaptive_cards::Item::Element(inner) => inner. $method_name (),
        adaptive_cards::Item::FactSet(inner) => inner, adaptive_cards::Item::Image(inner)
        => inner, adaptive_cards::Item::ImageSet(inner) => inner,
        adaptive_cards::Item::InputChoiceSet(inner) => inner,
        adaptive_cards::Item::InputDate(inner) => inner,
        adaptive_cards::Item::InputNumber(inner) => inner,
        adaptive_cards::Item::InputText(inner) => inner,
        adaptive_cards::Item::InputTime(inner) => inner,
        adaptive_cards::Item::InputToggle(inner) => inner,
        adaptive_cards::Item::Input(inner) => inner. $method_name (),
        adaptive_cards::Item::Media(inner) => inner,
        adaptive_cards::Item::RichTextBlock(inner) => inner,
        adaptive_cards::Item::Table(inner) => inner,
        adaptive_cards::Item::TextBlock(inner) => inner,
        adaptive_cards::Item::ToggleableItem(inner) => inner. $method_name (), } } } impl
        $as_trait_name for adaptive_cards::ToggleableItem < $layout_data > { fn
        $method_name (& self) -> & dyn $trait_name { match self {
        adaptive_cards::ToggleableItem::ActionSet(inner) => inner,
        adaptive_cards::ToggleableItem::Column(inner) => inner,
        adaptive_cards::ToggleableItem::ColumnSet(inner) => inner,
        adaptive_cards::ToggleableItem::Container(inner) => inner,
        adaptive_cards::ToggleableItem::Element(inner) => inner. $method_name (),
        adaptive_cards::ToggleableItem::FactSet(inner) => inner,
        adaptive_cards::ToggleableItem::Image(inner) => inner,
        adaptive_cards::ToggleableItem::ImageSet(inner) => inner,
        adaptive_cards::ToggleableItem::InputChoiceSet(inner) => inner,
        adaptive_cards::ToggleableItem::InputDate(inner) => inner,
        adaptive_cards::ToggleableItem::InputNumber(inner) => inner,
        adaptive_cards::ToggleableItem::InputText(inner) => inner,
        adaptive_cards::ToggleableItem::InputTime(inner) => inner,
        adaptive_cards::ToggleableItem::InputToggle(inner) => inner,
        adaptive_cards::ToggleableItem::Input(inner) => inner. $method_name (),
        adaptive_cards::ToggleableItem::Media(inner) => inner,
        adaptive_cards::ToggleableItem::RichTextBlock(inner) => inner,
        adaptive_cards::ToggleableItem::Table(inner) => inner,
        adaptive_cards::ToggleableItem::TextBlock(inner) => inner, } } }
    };
}
