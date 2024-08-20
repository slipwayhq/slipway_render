#![allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum ActionMode {
    #[serde(rename = "primary", alias = "Primary")]
    Primary,
    #[serde(rename = "secondary", alias = "Secondary")]
    Secondary,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum ActionStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "positive", alias = "Positive")]
    Positive,
    #[serde(rename = "destructive", alias = "Destructive")]
    Destructive,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum AssociatedInputs {
    #[serde(rename = "Auto", alias = "auto")]
    Auto,
    #[serde(rename = "None", alias = "none")]
    None,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum BlockElementHeight {
    #[serde(rename = "auto", alias = "Auto")]
    Auto,
    #[serde(rename = "stretch", alias = "Stretch")]
    Stretch,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum ChoiceInputStyle {
    #[serde(rename = "compact", alias = "Compact")]
    Compact,
    #[serde(rename = "expanded", alias = "Expanded")]
    Expanded,
    #[serde(rename = "filtered", alias = "Filtered")]
    Filtered,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum FallbackOption {
    #[serde(rename = "drop", alias = "Drop")]
    Drop,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum FontType {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "monospace", alias = "Monospace")]
    Monospace,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum FontWeight {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "lighter", alias = "Lighter")]
    Lighter,
    #[serde(rename = "bolder", alias = "Bolder")]
    Bolder,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum HorizontalAlignment {
    #[serde(rename = "left", alias = "Left")]
    Left,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "right", alias = "Right")]
    Right,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum ImageSetStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "stacked", alias = "Stacked")]
    Stacked,
    #[serde(rename = "grid", alias = "Grid")]
    Grid,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum ImageStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "person", alias = "Person")]
    Person,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum InputLabelPosition {
    #[serde(rename = "inline", alias = "Inline")]
    Inline,
    #[serde(rename = "above", alias = "Above")]
    Above,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum InputStyle {
    #[serde(rename = "revealOnHover", alias = "RevealOnHover")]
    RevealOnHover,
    #[serde(rename = "default", alias = "Default")]
    Default,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum TextBlockStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "heading", alias = "Heading")]
    Heading,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum VerticalAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
}
#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum VerticalContentAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
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
pub enum ActionOrFallbackOption {
    Action(Box<Action>),
    FallbackOption(FallbackOption),
}
impl From<Action> for ActionOrFallbackOption {
    fn from(value: Action) -> Self {
        ActionOrFallbackOption::Action(Box::new(value))
    }
}
impl From<FallbackOption> for ActionOrFallbackOption {
    fn from(value: FallbackOption) -> Self {
        ActionOrFallbackOption::FallbackOption(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionExecute {
    #[serde(
        rename = "associatedInputs",
        default = "ActionExecute::default_value_for_associated_inputs",
        skip_serializing_if = "ActionExecute::is_default_value_for_associated_inputs"
    )]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionExecute::default_value_for_is_enabled",
        skip_serializing_if = "ActionExecute::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionExecute::default_value_for_mode",
        skip_serializing_if = "ActionExecute::is_default_value_for_mode"
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
}
impl ActionExecute {
    fn default_value_for_associated_inputs() -> AssociatedInputs {
        AssociatedInputs::Auto
    }
    fn is_default_value_for_associated_inputs(value: &AssociatedInputs) -> bool {
        matches!(* value, AssociatedInputs::Auto)
    }
}
impl ActionExecute {
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl ActionExecute {
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionOpenUrl {
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionOpenUrl::default_value_for_is_enabled",
        skip_serializing_if = "ActionOpenUrl::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionOpenUrl::default_value_for_mode",
        skip_serializing_if = "ActionOpenUrl::is_default_value_for_mode"
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
}
impl ActionOpenUrl {
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl ActionOpenUrl {
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionShowCard {
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<AdaptiveCard>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionShowCard::default_value_for_is_enabled",
        skip_serializing_if = "ActionShowCard::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionShowCard::default_value_for_mode",
        skip_serializing_if = "ActionShowCard::is_default_value_for_mode"
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
}
impl ActionShowCard {
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl ActionShowCard {
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionSubmit {
    #[serde(
        rename = "associatedInputs",
        default = "ActionSubmit::default_value_for_associated_inputs",
        skip_serializing_if = "ActionSubmit::is_default_value_for_associated_inputs"
    )]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionSubmit::default_value_for_is_enabled",
        skip_serializing_if = "ActionSubmit::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionSubmit::default_value_for_mode",
        skip_serializing_if = "ActionSubmit::is_default_value_for_mode"
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
}
impl ActionSubmit {
    fn default_value_for_associated_inputs() -> AssociatedInputs {
        AssociatedInputs::Auto
    }
    fn is_default_value_for_associated_inputs(value: &AssociatedInputs) -> bool {
        matches!(* value, AssociatedInputs::Auto)
    }
}
impl ActionSubmit {
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl ActionSubmit {
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
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
pub struct ActionToggleVisibility {
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isEnabled",
        default = "ActionToggleVisibility::default_value_for_is_enabled",
        skip_serializing_if = "ActionToggleVisibility::is_default_value_for_is_enabled"
    )]
    pub is_enabled: bool,
    #[serde(
        rename = "mode",
        default = "ActionToggleVisibility::default_value_for_mode",
        skip_serializing_if = "ActionToggleVisibility::is_default_value_for_mode"
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
}
impl ActionToggleVisibility {
    fn default_value_for_is_enabled() -> bool {
        true
    }
    fn is_default_value_for_is_enabled(value: &bool) -> bool {
        *value == true
    }
}
impl ActionToggleVisibility {
    fn default_value_for_mode() -> ActionMode {
        ActionMode::Primary
    }
    fn is_default_value_for_mode(value: &ActionMode) -> bool {
        matches!(* value, ActionMode::Primary)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.ShowCard")]
    ShowCard(Box<ActionShowCard>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
}
#[derive(serde::Serialize, Clone)]
pub enum ElementOrFallbackOption {
    Element(Box<Element>),
    FallbackOption(FallbackOption),
}
impl From<Element> for ElementOrFallbackOption {
    fn from(value: Element) -> Self {
        ElementOrFallbackOption::Element(Box::new(value))
    }
}
impl From<FallbackOption> for ElementOrFallbackOption {
    fn from(value: FallbackOption) -> Self {
        ElementOrFallbackOption::FallbackOption(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionSet {
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "ActionSet::default_value_for_is_visible",
        skip_serializing_if = "ActionSet::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl ActionSet {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for ActionSet {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for ActionSet {}
impl crate::element::LayoutableElement for ActionSet {
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
pub struct AdaptiveCard {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "authentication", skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<Element>>,
    #[serde(rename = "fallbackText", skip_serializing_if = "Option::is_none")]
    pub fallback_text: Option<String>,
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<Refresh>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl crate::layoutable::HasLayoutData for AdaptiveCard {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
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
pub enum ColumnOrFallbackOption {
    Column(Box<Column>),
    FallbackOption(FallbackOption),
}
impl From<Column> for ColumnOrFallbackOption {
    fn from(value: Column) -> Self {
        ColumnOrFallbackOption::Column(Box::new(value))
    }
}
impl From<FallbackOption> for ColumnOrFallbackOption {
    fn from(value: FallbackOption) -> Self {
        ColumnOrFallbackOption::FallbackOption(value)
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
pub struct Column {
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ColumnOrFallbackOption>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Column::default_value_for_is_visible",
        skip_serializing_if = "Column::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Element>>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
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
}
impl Column {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ColumnSet {
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "ColumnSet::default_value_for_is_visible",
        skip_serializing_if = "ColumnSet::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing", skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl ColumnSet {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for ColumnSet {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for ColumnSet {}
impl crate::element::LayoutableElement for ColumnSet {
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
pub struct Container {
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Container::default_value_for_is_visible",
        skip_serializing_if = "Container::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl Container {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for Container {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::element::LayoutableElement for Container {
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
pub enum Element {
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
}
impl Element {
    pub(crate) fn as_layoutable(&self) -> &dyn crate::layoutable::Layoutable {
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
            Element::Input(inner) => inner.as_layoutable(),
            Element::Media(inner) => inner,
            Element::RichTextBlock(inner) => inner,
            Element::Table(inner) => inner,
            Element::TextBlock(inner) => inner,
        }
    }
}
impl Element {
    pub(crate) fn as_element(&self) -> &dyn crate::element::LayoutableElement {
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
pub struct FactSet {
    #[serde(rename = "facts")]
    pub facts: Vec<Fact>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "FactSet::default_value_for_is_visible",
        skip_serializing_if = "FactSet::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl FactSet {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for FactSet {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for FactSet {}
impl crate::element::LayoutableElement for FactSet {
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
pub enum ISelectAction {
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
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
pub struct Image {
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "backgroundColor", skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(
        rename = "height",
        default = "Image::default_value_for_height",
        skip_serializing_if = "Image::is_default_value_for_height"
    )]
    pub height: StringOrBlockElementHeight,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Image::default_value_for_is_visible",
        skip_serializing_if = "Image::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl Image {
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
impl Image {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for Image {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for Image {}
impl crate::element::LayoutableElement for Image {
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
pub struct ImageSet {
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "imageSize",
        default = "ImageSet::default_value_for_image_size",
        skip_serializing_if = "ImageSet::is_default_value_for_image_size"
    )]
    pub image_size: ImageSize,
    #[serde(rename = "images")]
    pub images: Vec<Image>,
    #[serde(
        rename = "isVisible",
        default = "ImageSet::default_value_for_is_visible",
        skip_serializing_if = "ImageSet::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl ImageSet {
    fn default_value_for_image_size() -> ImageSize {
        ImageSize::Medium
    }
    fn is_default_value_for_image_size(value: &ImageSize) -> bool {
        matches!(* value, ImageSize::Medium)
    }
}
impl ImageSet {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for ImageSet {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for ImageSet {}
impl crate::element::LayoutableElement for ImageSet {
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
pub enum Inline {
    #[serde(rename = "TextRun")]
    TextRun(Box<TextRun>),
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
pub struct InputChoiceSet {
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<InputChoice>>,
    #[serde(rename = "choices.data", skip_serializing_if = "Option::is_none")]
    pub choices_data: Option<DataQuery>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(
        rename = "isMultiSelect",
        default = "InputChoiceSet::default_value_for_is_multi_select",
        skip_serializing_if = "InputChoiceSet::is_default_value_for_is_multi_select"
    )]
    pub is_multi_select: bool,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputChoiceSet::default_value_for_is_visible",
        skip_serializing_if = "InputChoiceSet::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputChoiceSet {
    fn default_value_for_is_multi_select() -> bool {
        false
    }
    fn is_default_value_for_is_multi_select(value: &bool) -> bool {
        *value == false
    }
}
impl InputChoiceSet {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for InputChoiceSet {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputChoiceSet {}
impl crate::element::LayoutableElement for InputChoiceSet {
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
pub struct InputDate {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
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
        default = "InputDate::default_value_for_is_visible",
        skip_serializing_if = "InputDate::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputDate {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for InputDate {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputDate {}
impl crate::element::LayoutableElement for InputDate {
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
pub struct InputNumber {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
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
        default = "InputNumber::default_value_for_is_visible",
        skip_serializing_if = "InputNumber::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputNumber {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for InputNumber {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputNumber {}
impl crate::element::LayoutableElement for InputNumber {
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
pub struct InputText {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "inlineAction", skip_serializing_if = "Option::is_none")]
    pub inline_action: Option<ISelectAction>,
    #[serde(rename = "inputStyle", skip_serializing_if = "Option::is_none")]
    pub input_style: Option<InputStyle>,
    #[serde(
        rename = "isMultiline",
        default = "InputText::default_value_for_is_multiline",
        skip_serializing_if = "InputText::is_default_value_for_is_multiline"
    )]
    pub is_multiline: bool,
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(
        rename = "isVisible",
        default = "InputText::default_value_for_is_visible",
        skip_serializing_if = "InputText::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputText {
    fn default_value_for_is_multiline() -> bool {
        false
    }
    fn is_default_value_for_is_multiline(value: &bool) -> bool {
        *value == false
    }
}
impl InputText {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for InputText {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputText {}
impl crate::element::LayoutableElement for InputText {
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
pub struct InputTime {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
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
        default = "InputTime::default_value_for_is_visible",
        skip_serializing_if = "InputTime::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputTime {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for InputTime {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputTime {}
impl crate::element::LayoutableElement for InputTime {
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
pub struct InputToggle {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
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
        default = "InputToggle::default_value_for_is_visible",
        skip_serializing_if = "InputToggle::is_default_value_for_is_visible"
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
        default = "InputToggle::default_value_for_value",
        skip_serializing_if = "InputToggle::is_default_value_for_value"
    )]
    pub value: String,
    #[serde(
        rename = "valueOff",
        default = "InputToggle::default_value_for_value_off",
        skip_serializing_if = "InputToggle::is_default_value_for_value_off"
    )]
    pub value_off: String,
    #[serde(
        rename = "valueOn",
        default = "InputToggle::default_value_for_value_on",
        skip_serializing_if = "InputToggle::is_default_value_for_value_on"
    )]
    pub value_on: String,
    #[serde(rename = "wrap", skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl InputToggle {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl InputToggle {
    fn default_value_for_value() -> String {
        String::from("false")
    }
    fn is_default_value_for_value(value: &String) -> bool {
        *value == "false"
    }
}
impl InputToggle {
    fn default_value_for_value_off() -> String {
        String::from("false")
    }
    fn is_default_value_for_value_off(value: &String) -> bool {
        *value == "false"
    }
}
impl InputToggle {
    fn default_value_for_value_on() -> String {
        String::from("true")
    }
    fn is_default_value_for_value_on(value: &String) -> bool {
        *value == "true"
    }
}
impl crate::layoutable::HasLayoutData for InputToggle {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for InputToggle {}
impl crate::element::LayoutableElement for InputToggle {
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
pub enum Input {
    #[serde(rename = "Input.ChoiceSet")]
    ChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Date")]
    Date(Box<InputDate>),
    #[serde(rename = "Input.Number")]
    Number(Box<InputNumber>),
    #[serde(rename = "Input.Text")]
    Text(Box<InputText>),
    #[serde(rename = "Input.Time")]
    Time(Box<InputTime>),
    #[serde(rename = "Input.Toggle")]
    Toggle(Box<InputToggle>),
}
impl Input {
    pub(crate) fn as_layoutable(&self) -> &dyn crate::layoutable::Layoutable {
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
impl Input {
    pub(crate) fn as_element(&self) -> &dyn crate::element::LayoutableElement {
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
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "Action.Execute")]
    ActionExecute(Box<ActionExecute>),
    #[serde(rename = "Action.OpenUrl")]
    ActionOpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.ShowCard")]
    ActionShowCard(Box<ActionShowCard>),
    #[serde(rename = "Action.Submit")]
    ActionSubmit(Box<ActionSubmit>),
    #[serde(rename = "Action.ToggleVisibility")]
    ActionToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action")]
    Action(Box<Action>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "ToggleableItem")]
    ToggleableItem(Box<ToggleableItem>),
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Media {
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "captionSources", skip_serializing_if = "Option::is_none")]
    pub caption_sources: Option<Vec<CaptionSource>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "isVisible",
        default = "Media::default_value_for_is_visible",
        skip_serializing_if = "Media::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl Media {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for Media {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for Media {}
impl crate::element::LayoutableElement for Media {
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
pub struct Refresh {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionExecute>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(serde::Serialize, Clone)]
pub enum InlineOrString {
    Inline(Inline),
    String(String),
}
impl From<Inline> for InlineOrString {
    fn from(value: Inline) -> Self {
        InlineOrString::Inline(value)
    }
}
impl From<String> for InlineOrString {
    fn from(value: String) -> Self {
        InlineOrString::String(value)
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RichTextBlock {
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "horizontalAlignment", skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inlines")]
    pub inlines: Vec<InlineOrString>,
    #[serde(
        rename = "isVisible",
        default = "RichTextBlock::default_value_for_is_visible",
        skip_serializing_if = "RichTextBlock::is_default_value_for_is_visible"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl RichTextBlock {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for RichTextBlock {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for RichTextBlock {}
impl crate::element::LayoutableElement for RichTextBlock {
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
pub struct Table {
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TableColumnDefinition>>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(
        rename = "firstRowAsHeader",
        default = "Table::default_value_for_first_row_as_header",
        skip_serializing_if = "Table::is_default_value_for_first_row_as_header"
    )]
    pub first_row_as_header: bool,
    #[serde(
        rename = "gridStyle",
        default = "Table::default_value_for_grid_style",
        skip_serializing_if = "Table::is_default_value_for_grid_style"
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
        default = "Table::default_value_for_is_visible",
        skip_serializing_if = "Table::is_default_value_for_is_visible"
    )]
    pub is_visible: bool,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<TableRow>>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    #[serde(
        rename = "showGridLines",
        default = "Table::default_value_for_show_grid_lines",
        skip_serializing_if = "Table::is_default_value_for_show_grid_lines"
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
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl Table {
    fn default_value_for_first_row_as_header() -> bool {
        true
    }
    fn is_default_value_for_first_row_as_header(value: &bool) -> bool {
        *value == true
    }
}
impl Table {
    fn default_value_for_grid_style() -> ContainerStyle {
        ContainerStyle::Default
    }
    fn is_default_value_for_grid_style(value: &ContainerStyle) -> bool {
        matches!(* value, ContainerStyle::Default)
    }
}
impl Table {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl Table {
    fn default_value_for_show_grid_lines() -> bool {
        true
    }
    fn is_default_value_for_show_grid_lines(value: &bool) -> bool {
        *value == true
    }
}
impl crate::layoutable::HasLayoutData for Table {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::layoutable::Layoutable for Table {}
impl crate::element::LayoutableElement for Table {
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
pub struct TableCell {
    #[serde(rename = "backgroundImage", skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed", skip_serializing_if = "Option::is_none")]
    pub bleed: Option<bool>,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "minHeight", skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(rename = "rtl", skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<ContainerStyle>,
    #[serde(
        rename = "verticalContentAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
pub struct TableRow {
    #[serde(rename = "cells", skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<TableCell>>,
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
pub struct TextBlock {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Colors>,
    #[serde(rename = "fallback", skip_serializing_if = "Option::is_none")]
    pub fallback: Option<ElementOrFallbackOption>,
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
        default = "TextBlock::default_value_for_is_subtle",
        skip_serializing_if = "TextBlock::is_default_value_for_is_subtle"
    )]
    pub is_subtle: bool,
    #[serde(
        rename = "isVisible",
        default = "TextBlock::default_value_for_is_visible",
        skip_serializing_if = "TextBlock::is_default_value_for_is_visible"
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
        default = "TextBlock::default_value_for_style",
        skip_serializing_if = "TextBlock::is_default_value_for_style"
    )]
    pub style: TextBlockStyle,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<FontWeight>,
    #[serde(
        rename = "wrap",
        default = "TextBlock::default_value_for_wrap",
        skip_serializing_if = "TextBlock::is_default_value_for_wrap"
    )]
    pub wrap: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = ".layout", skip_deserializing)]
    pub layout_data: core::cell::RefCell<crate::element_layout_data::ElementLayoutData>,
}
impl TextBlock {
    fn default_value_for_is_subtle() -> bool {
        false
    }
    fn is_default_value_for_is_subtle(value: &bool) -> bool {
        *value == false
    }
}
impl TextBlock {
    fn default_value_for_is_visible() -> bool {
        true
    }
    fn is_default_value_for_is_visible(value: &bool) -> bool {
        *value == true
    }
}
impl TextBlock {
    fn default_value_for_style() -> TextBlockStyle {
        TextBlockStyle::Default
    }
    fn is_default_value_for_style(value: &TextBlockStyle) -> bool {
        matches!(* value, TextBlockStyle::Default)
    }
}
impl TextBlock {
    fn default_value_for_wrap() -> bool {
        false
    }
    fn is_default_value_for_wrap(value: &bool) -> bool {
        *value == false
    }
}
impl crate::layoutable::HasLayoutData for TextBlock {
    fn layout_data(
        &self,
    ) -> &core::cell::RefCell<crate::element_layout_data::ElementLayoutData> {
        &self.layout_data
    }
}
impl crate::element::LayoutableElement for TextBlock {
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
pub struct TextRun {
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Colors>,
    #[serde(rename = "fontType", skip_serializing_if = "Option::is_none")]
    pub font_type: Option<FontType>,
    #[serde(rename = "highlight", skip_serializing_if = "Option::is_none")]
    pub highlight: Option<bool>,
    #[serde(
        rename = "isSubtle",
        default = "TextRun::default_value_for_is_subtle",
        skip_serializing_if = "TextRun::is_default_value_for_is_subtle"
    )]
    pub is_subtle: bool,
    #[serde(rename = "italic", skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(rename = "selectAction", skip_serializing_if = "Option::is_none")]
    pub select_action: Option<ISelectAction>,
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
}
impl TextRun {
    fn default_value_for_is_subtle() -> bool {
        false
    }
    fn is_default_value_for_is_subtle(value: &bool) -> bool {
        *value == false
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(tag = "type")]
pub enum ToggleableItem {
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
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
