#![allow(dead_code)]
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum VerticalAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
}
#[derive(serde::Deserialize)]
pub enum ActionStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "positive", alias = "Positive")]
    Positive,
    #[serde(rename = "destructive", alias = "Destructive")]
    Destructive,
}
#[derive(serde::Deserialize)]
pub enum VerticalContentAlignment {
    #[serde(rename = "top", alias = "Top")]
    Top,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "bottom", alias = "Bottom")]
    Bottom,
}
#[derive(serde::Deserialize)]
pub enum FontType {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "monospace", alias = "Monospace")]
    Monospace,
}
#[derive(serde::Deserialize)]
pub enum HorizontalAlignment {
    #[serde(rename = "left", alias = "Left")]
    Left,
    #[serde(rename = "center", alias = "Center")]
    Center,
    #[serde(rename = "right", alias = "Right")]
    Right,
}
#[derive(serde::Deserialize)]
pub enum FallbackOption {
    #[serde(rename = "drop", alias = "Drop")]
    Drop,
}
#[derive(serde::Deserialize)]
pub enum AssociatedInputs {
    #[serde(rename = "Auto", alias = "auto")]
    Auto,
    #[serde(rename = "None", alias = "none")]
    None,
}
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum ChoiceInputStyle {
    #[serde(rename = "compact", alias = "Compact")]
    Compact,
    #[serde(rename = "expanded", alias = "Expanded")]
    Expanded,
    #[serde(rename = "filtered", alias = "Filtered")]
    Filtered,
}
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum TextBlockStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "heading", alias = "Heading")]
    Heading,
}
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum ImageSetStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "stacked", alias = "Stacked")]
    Stacked,
    #[serde(rename = "grid", alias = "Grid")]
    Grid,
}
#[derive(serde::Deserialize)]
pub enum InputStyle {
    #[serde(rename = "revealOnHover", alias = "RevealOnHover")]
    RevealOnHover,
    #[serde(rename = "default", alias = "Default")]
    Default,
}
#[derive(serde::Deserialize)]
pub enum ImageStyle {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "person", alias = "Person")]
    Person,
}
#[derive(serde::Deserialize)]
pub enum InputLabelPosition {
    #[serde(rename = "inline", alias = "Inline")]
    Inline,
    #[serde(rename = "above", alias = "Above")]
    Above,
}
#[derive(serde::Deserialize)]
pub enum FontWeight {
    #[serde(rename = "default", alias = "Default")]
    Default,
    #[serde(rename = "lighter", alias = "Lighter")]
    Lighter,
    #[serde(rename = "bolder", alias = "Bolder")]
    Bolder,
}
#[derive(serde::Deserialize)]
pub enum ActionMode {
    #[serde(rename = "primary", alias = "Primary")]
    Primary,
    #[serde(rename = "secondary", alias = "Secondary")]
    Secondary,
}
#[derive(serde::Deserialize)]
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
#[derive(serde::Deserialize)]
pub enum BlockElementHeight {
    #[serde(rename = "auto", alias = "Auto")]
    Auto,
    #[serde(rename = "stretch", alias = "Stretch")]
    Stretch,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum ToggleableItem {
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
}
use super::utils::deserialize_element_or_fallback_option_optional;
#[derive(serde::Deserialize)]
pub enum ElementOrFallbackOption {
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "FallbackOption")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSet {
    #[serde(rename = "images")]
    pub images: Image,
    #[serde(rename = "style")]
    pub style: Option<ImageSetStyle>,
    #[serde(rename = "imageSize")]
    pub image_size: Option<ImageSize>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_background_image_or_string_optional;
#[derive(serde::Deserialize)]
pub enum BackgroundImageOrString {
    #[serde(rename = "BackgroundImage")]
    BackgroundImage(BackgroundImage),
    #[serde(rename = "String")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Container {
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "backgroundImage")]
    #[serde(
        default,
        deserialize_with = "deserialize_background_image_or_string_optional"
    )]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "items")]
    pub items: Element,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
}
use super::utils::deserialize_string_or_number_optional;
#[derive(serde::Deserialize)]
pub enum StringOrNumber {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "number")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputToggle {
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "valueOff")]
    pub value_off: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "valueOn")]
    pub value_on: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataQuery {
    #[serde(rename = "count")]
    pub count: Option<f64>,
    #[serde(rename = "dataset")]
    pub dataset: String,
    #[serde(rename = "skip")]
    pub skip: Option<f64>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_action_or_fallback_option_optional;
#[derive(serde::Deserialize)]
pub enum ActionOrFallbackOption {
    #[serde(rename = "Action")]
    Action(Box<Action>),
    #[serde(rename = "FallbackOption")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionShowCard {
    #[serde(rename = "card")]
    pub card: Option<AdaptiveCard>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_action_or_fallback_option_optional"
    )]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Refresh {
    #[serde(rename = "expires")]
    pub expires: Option<String>,
    #[serde(rename = "userIds")]
    pub user_ids: Option<String>,
    #[serde(rename = "action")]
    pub action: Option<ActionExecute>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Media {
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[serde(rename = "captionSources")]
    pub caption_sources: Option<CaptionSource>,
    #[serde(rename = "sources")]
    pub sources: MediaSource,
    #[serde(rename = "poster")]
    pub poster: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthCardButton {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "title")]
    pub title: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionSet {
    #[serde(rename = "actions")]
    pub actions: Action,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MediaSource {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_column_or_fallback_option_optional;
#[derive(serde::Deserialize)]
pub enum ColumnOrFallbackOption {
    #[serde(rename = "Column")]
    Column(Box<Column>),
    #[serde(rename = "FallbackOption")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Column {
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "items")]
    pub items: Option<Element>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "width")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub width: Option<StringOrNumber>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_column_or_fallback_option_optional"
    )]
    pub fallback: Option<ColumnOrFallbackOption>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "backgroundImage")]
    #[serde(
        default,
        deserialize_with = "deserialize_background_image_or_string_optional"
    )]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnSet {
    #[serde(rename = "columns")]
    pub columns: Option<Column>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptionSource {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    #[serde(rename = "rows")]
    pub rows: Option<TableRow>,
    #[serde(rename = "firstRowAsHeader")]
    pub first_row_as_header: Option<bool>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "showGridLines")]
    pub show_grid_lines: Option<bool>,
    #[serde(rename = "columns")]
    pub columns: Option<TableColumnDefinition>,
    #[serde(rename = "gridStyle")]
    pub grid_style: Option<ContainerStyle>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputText {
    #[serde(rename = "inlineAction")]
    pub inline_action: Option<ISelectAction>,
    #[serde(rename = "regex")]
    pub regex: Option<String>,
    #[serde(rename = "isMultiline")]
    pub is_multiline: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<f64>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<TextInputStyle>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "ToggleableItem")]
    ToggleableItem(Box<ToggleableItem>),
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
    #[serde(rename = "Action")]
    Action(Box<Action>),
    #[serde(rename = "Action.ToggleVisibility")]
    ActionToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.OpenUrl")]
    ActionOpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Execute")]
    ActionExecute(Box<ActionExecute>),
    #[serde(rename = "Action.ShowCard")]
    ActionShowCard(Box<ActionShowCard>),
    #[serde(rename = "Action.Submit")]
    ActionSubmit(Box<ActionSubmit>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableRow {
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "cells")]
    pub cells: Option<TableCell>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Inline {
    #[serde(rename = "TextRun")]
    TextRun(Box<TextRun>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableCell {
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "items")]
    pub items: Element,
    #[serde(rename = "backgroundImage")]
    #[serde(
        default,
        deserialize_with = "deserialize_background_image_or_string_optional"
    )]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_string_or_object_optional;
#[derive(serde::Deserialize)]
pub enum StringOrObject {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "object")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionSubmit {
    #[serde(rename = "data")]
    #[serde(default, deserialize_with = "deserialize_string_or_object_optional")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: Option<AssociatedInputs>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_action_or_fallback_option_optional"
    )]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextBlock {
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<bool>,
    #[serde(rename = "weight")]
    pub weight: Option<FontWeight>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "maxLines")]
    pub max_lines: Option<f64>,
    #[serde(rename = "size")]
    pub size: Option<FontSize>,
    #[serde(rename = "style")]
    pub style: Option<TextBlockStyle>,
    #[serde(rename = "fontType")]
    pub font_type: Option<FontType>,
    #[serde(rename = "color")]
    pub color: Option<Colors>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputTime {
    #[serde(rename = "max")]
    pub max: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "min")]
    pub min: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextRun {
    #[serde(rename = "weight")]
    pub weight: Option<FontWeight>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "strikethrough")]
    pub strikethrough: Option<bool>,
    #[serde(rename = "highlight")]
    pub highlight: Option<bool>,
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<bool>,
    #[serde(rename = "underline")]
    pub underline: Option<bool>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "color")]
    pub color: Option<Colors>,
    #[serde(rename = "fontType")]
    pub font_type: Option<FontType>,
    #[serde(rename = "italic")]
    pub italic: Option<bool>,
    #[serde(rename = "size")]
    pub size: Option<FontSize>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundImage {
    #[serde(rename = "verticalAlignment")]
    pub vertical_alignment: Option<VerticalAlignment>,
    #[serde(rename = "fillMode")]
    pub fill_mode: Option<ImageFillMode>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputDate {
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "min")]
    pub min: Option<String>,
    #[serde(rename = "max")]
    pub max: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_target_element_or_string;
#[derive(serde::Deserialize)]
pub enum TargetElementOrString {
    #[serde(rename = "TargetElement")]
    TargetElement(TargetElement),
    #[serde(rename = "String")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionToggleVisibility {
    #[serde(rename = "targetElements")]
    #[serde(deserialize_with = "deserialize_target_element_or_string")]
    pub target_elements: Vec<TargetElementOrString>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_action_or_fallback_option_optional"
    )]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactSet {
    #[serde(rename = "facts")]
    pub facts: Fact,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputChoice {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputNumber {
    #[serde(rename = "min")]
    pub min: Option<f64>,
    #[serde(rename = "max")]
    pub max: Option<f64>,
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum ISelectAction {
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
    #[serde(rename = "Action.ShowCard")]
    ShowCard(Box<ActionShowCard>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionOpenUrl {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_action_or_fallback_option_optional"
    )]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableColumnDefinition {
    #[serde(rename = "width")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub width: Option<StringOrNumber>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_string_or_block_element_height_optional;
#[derive(serde::Deserialize)]
pub enum StringOrBlockElementHeight {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "BlockElementHeight")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "size")]
    pub size: Option<ImageSize>,
    #[serde(rename = "height")]
    #[serde(
        default,
        deserialize_with = "deserialize_string_or_block_element_height_optional"
    )]
    pub height: Option<StringOrBlockElementHeight>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ImageStyle>,
    #[serde(rename = "width")]
    pub width: Option<String>,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Input {
    #[serde(rename = "Input.Date")]
    Date(Box<InputDate>),
    #[serde(rename = "Input.Time")]
    Time(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    Number(Box<InputNumber>),
    #[serde(rename = "Input.Toggle")]
    Toggle(Box<InputToggle>),
    #[serde(rename = "Input.ChoiceSet")]
    ChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Text")]
    Text(Box<InputText>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenExchangeResource {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
use super::utils::deserialize_inline_or_string;
#[derive(serde::Deserialize)]
pub enum InlineOrString {
    #[serde(rename = "Inline")]
    Inline(Inline),
    #[serde(rename = "String")]
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RichTextBlock {
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "inlines")]
    #[serde(deserialize_with = "deserialize_inline_or_string")]
    pub inlines: Vec<InlineOrString>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "webUrl")]
    pub web_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionExecute {
    #[serde(rename = "verb")]
    pub verb: Option<String>,
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: Option<AssociatedInputs>,
    #[serde(rename = "data")]
    #[serde(default, deserialize_with = "deserialize_string_or_object_optional")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_action_or_fallback_option_optional"
    )]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputChoiceSet {
    #[serde(rename = "choices.data")]
    pub choices_data: Option<DataQuery>,
    #[serde(rename = "isMultiSelect")]
    pub is_multi_select: Option<bool>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ChoiceInputStyle>,
    #[serde(rename = "choices")]
    pub choices: Option<InputChoice>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "labelWidth")]
    #[serde(default, deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(
        default,
        deserialize_with = "deserialize_element_or_fallback_option_optional"
    )]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveCard {
    #[serde(rename = "actions")]
    pub actions: Option<Action>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "fallbackText")]
    pub fallback_text: Option<String>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "refresh")]
    pub refresh: Option<Refresh>,
    #[serde(rename = "authentication")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "lang")]
    pub lang: Option<String>,
    #[serde(rename = "speak")]
    pub speak: Option<String>,
    #[serde(rename = "backgroundImage")]
    #[serde(
        default,
        deserialize_with = "deserialize_background_image_or_string_optional"
    )]
    pub background_image: Option<BackgroundImageOrString>,
    #[serde(rename = "body")]
    pub body: Option<Element>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authentication {
    #[serde(rename = "tokenExchangeResource")]
    pub token_exchange_resource: Option<TokenExchangeResource>,
    #[serde(rename = "buttons")]
    pub buttons: Option<AuthCardButton>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetElement {
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
