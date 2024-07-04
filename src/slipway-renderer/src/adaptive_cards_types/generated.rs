#![allow(dead_code)]
#[derive(serde::Deserialize)]
pub enum ImageSize {
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
#[derive(serde::Deserialize)]
pub enum FontSize {
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
#[derive(serde::Deserialize)]
pub enum VerticalAlignment {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "bottom")]
    Bottom,
}
#[derive(serde::Deserialize)]
pub enum ActionStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "positive")]
    Positive,
    #[serde(rename = "destructive")]
    Destructive,
}
#[derive(serde::Deserialize)]
pub enum VerticalContentAlignment {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "bottom")]
    Bottom,
}
#[derive(serde::Deserialize)]
pub enum FontType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "monospace")]
    Monospace,
}
#[derive(serde::Deserialize)]
pub enum HorizontalAlignment {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right,
}
#[derive(serde::Deserialize)]
pub enum FallbackOption {
    #[serde(rename = "drop")]
    Drop,
}
#[derive(serde::Deserialize)]
pub enum AssociatedInputs {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "None")]
    None,
}
#[derive(serde::Deserialize)]
pub enum TextInputStyle {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "password")]
    Password,
}
#[derive(serde::Deserialize)]
pub enum ChoiceInputStyle {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "expanded")]
    Expanded,
    #[serde(rename = "filtered")]
    Filtered,
}
#[derive(serde::Deserialize)]
pub enum ImageFillMode {
    #[serde(rename = "cover")]
    Cover,
    #[serde(rename = "repeatHorizontally")]
    RepeatHorizontally,
    #[serde(rename = "repeatVertically")]
    RepeatVertically,
    #[serde(rename = "repeat")]
    Repeat,
}
#[derive(serde::Deserialize)]
pub enum TextBlockStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "heading")]
    Heading,
}
#[derive(serde::Deserialize)]
pub enum Spacing {
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
#[derive(serde::Deserialize)]
pub enum ContainerStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "emphasis")]
    Emphasis,
    #[serde(rename = "good")]
    Good,
    #[serde(rename = "attention")]
    Attention,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "accent")]
    Accent,
}
#[derive(serde::Deserialize)]
pub enum ImageSetStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "stacked")]
    Stacked,
    #[serde(rename = "grid")]
    Grid,
}
#[derive(serde::Deserialize)]
pub enum InputStyle {
    #[serde(rename = "revealOnHover")]
    RevealOnHover,
    #[serde(rename = "default")]
    Default,
}
#[derive(serde::Deserialize)]
pub enum ImageStyle {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "person")]
    Person,
}
#[derive(serde::Deserialize)]
pub enum InputLabelPosition {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "above")]
    Above,
}
#[derive(serde::Deserialize)]
pub enum FontWeight {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "bolder")]
    Bolder,
}
#[derive(serde::Deserialize)]
pub enum ActionMode {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "secondary")]
    Secondary,
}
#[derive(serde::Deserialize)]
pub enum Colors {
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
#[derive(serde::Deserialize)]
pub enum BlockElementHeight {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "stretch")]
    Stretch,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetElement {
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<Option<bool>>,
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
pub struct ActionOpenUrl {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_action_or_fallback_option_optional")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
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
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    #[serde(rename = "webUrl")]
    pub web_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum ISelectAction {
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionShowCard {
    #[serde(rename = "card")]
    pub card: Option<AdaptiveCard>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_action_or_fallback_option_optional")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
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
pub struct InputText {
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "regex")]
    pub regex: Option<String>,
    #[serde(rename = "inlineAction")]
    pub inline_action: Option<ISelectAction>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "isMultiline")]
    pub is_multiline: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<f64>,
    #[serde(rename = "style")]
    pub style: Option<TextInputStyle>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImageSet {
    #[serde(rename = "imageSize")]
    pub image_size: Option<ImageSize>,
    #[serde(rename = "images")]
    pub images: Vec<Image>,
    #[serde(rename = "style")]
    pub style: Option<ImageSetStyle>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    #[serde(rename = "firstRowAsHeader")]
    pub first_row_as_header: Option<bool>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "gridStyle")]
    pub grid_style: Option<Option<ContainerStyle>>,
    #[serde(rename = "rows")]
    pub rows: Option<Vec<TableRow>>,
    #[serde(rename = "columns")]
    pub columns: Option<Vec<TableColumnDefinition>>,
    #[serde(rename = "showGridLines")]
    pub show_grid_lines: Option<bool>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<Option<VerticalAlignment>>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Authentication {
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,
    #[serde(rename = "tokenExchangeResource")]
    pub token_exchange_resource: Option<TokenExchangeResource>,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "buttons")]
    pub buttons: Option<Vec<AuthCardButton>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableColumnDefinition {
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "width")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub width: Option<StringOrNumber>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<Option<VerticalAlignment>>,
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
    #[serde(rename = "width")]
    pub width: Option<String>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "size")]
    pub size: Option<ImageSize>,
    #[serde(rename = "style")]
    pub style: Option<ImageStyle>,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "height")]
    #[serde(deserialize_with = "deserialize_string_or_block_element_height_optional")]
    pub height: Option<StringOrBlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
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
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum ToggleableItem {
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionSet {
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthCardButton {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "image")]
    pub image: Option<String>,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableCell {
    #[serde(rename = "style")]
    pub style: Option<Option<ContainerStyle>>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "backgroundImage")]
    pub background_image: Option<BackgroundImage>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<Option<VerticalContentAlignment>>,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
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
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "Action")]
    Action(Box<Action>),
    #[serde(rename = "Action.OpenUrl")]
    ActionOpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Submit")]
    ActionSubmit(Box<ActionSubmit>),
    #[serde(rename = "Action.ToggleVisibility")]
    ActionToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.Execute")]
    ActionExecute(Box<ActionExecute>),
    #[serde(rename = "Action.ShowCard")]
    ActionShowCard(Box<ActionShowCard>),
    #[serde(rename = "ToggleableItem")]
    ToggleableItem(Box<ToggleableItem>),
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
    #[serde(rename = "Column")]
    Column(Box<Column>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundImage {
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "fillMode")]
    pub fill_mode: Option<ImageFillMode>,
    #[serde(rename = "verticalAlignment")]
    pub vertical_alignment: Option<VerticalAlignment>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputChoiceSet {
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ChoiceInputStyle>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "choices")]
    pub choices: Option<Vec<InputChoice>>,
    #[serde(rename = "isMultiSelect")]
    pub is_multi_select: Option<bool>,
    #[serde(rename = "choices.data")]
    pub choices_data: Option<DataQuery>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Refresh {
    #[serde(rename = "userIds")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "action")]
    pub action: Option<ActionExecute>,
    #[serde(rename = "expires")]
    pub expires: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableRow {
    #[serde(rename = "style")]
    pub style: Option<Option<ContainerStyle>>,
    #[serde(rename = "cells")]
    pub cells: Option<Vec<TableCell>>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<Option<VerticalAlignment>>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputToggle {
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "valueOn")]
    pub value_on: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "valueOff")]
    pub value_off: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
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
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "Action.OpenUrl")]
    OpenUrl(Box<ActionOpenUrl>),
    #[serde(rename = "Action.Submit")]
    Submit(Box<ActionSubmit>),
    #[serde(rename = "Action.ToggleVisibility")]
    ToggleVisibility(Box<ActionToggleVisibility>),
    #[serde(rename = "Action.Execute")]
    Execute(Box<ActionExecute>),
    #[serde(rename = "Action.ShowCard")]
    ShowCard(Box<ActionShowCard>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextRun {
    #[serde(rename = "weight")]
    pub weight: Option<Option<FontWeight>>,
    #[serde(rename = "size")]
    pub size: Option<Option<FontSize>>,
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<Option<bool>>,
    #[serde(rename = "strikethrough")]
    pub strikethrough: Option<bool>,
    #[serde(rename = "italic")]
    pub italic: Option<bool>,
    #[serde(rename = "fontType")]
    pub font_type: Option<Option<FontType>>,
    #[serde(rename = "highlight")]
    pub highlight: Option<bool>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "color")]
    pub color: Option<Option<Colors>>,
    #[serde(rename = "underline")]
    pub underline: Option<bool>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
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
pub struct ActionExecute {
    #[serde(rename = "data")]
    #[serde(deserialize_with = "deserialize_string_or_object_optional")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "verb")]
    pub verb: Option<String>,
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: Option<AssociatedInputs>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_action_or_fallback_option_optional")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactSet {
    #[serde(rename = "facts")]
    pub facts: Vec<Fact>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputDate {
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "min")]
    pub min: Option<String>,
    #[serde(rename = "max")]
    pub max: Option<String>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionToggleVisibility {
    #[serde(rename = "targetElements")]
    pub target_elements: Vec<TargetElement>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_action_or_fallback_option_optional")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionSubmit {
    #[serde(rename = "data")]
    #[serde(deserialize_with = "deserialize_string_or_object_optional")]
    pub data: Option<StringOrObject>,
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: Option<AssociatedInputs>,
    #[serde(rename = "mode")]
    pub mode: Option<ActionMode>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "style")]
    pub style: Option<ActionStyle>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "isEnabled")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "tooltip")]
    pub tooltip: Option<String>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_action_or_fallback_option_optional")]
    pub fallback: Option<ActionOrFallbackOption>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveCard {
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "speak")]
    pub speak: Option<String>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "fallbackText")]
    pub fallback_text: Option<String>,
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(rename = "actions")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "refresh")]
    pub refresh: Option<Refresh>,
    #[serde(rename = "backgroundImage")]
    pub background_image: Option<BackgroundImage>,
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "body")]
    pub body: Option<Vec<Element>>,
    #[serde(rename = "authentication")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "lang")]
    pub lang: Option<String>,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "Input")]
    Input(Box<Input>),
    #[serde(rename = "Input.ChoiceSet")]
    InputChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Time")]
    InputTime(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    InputNumber(Box<InputNumber>),
    #[serde(rename = "Input.Date")]
    InputDate(Box<InputDate>),
    #[serde(rename = "Input.Text")]
    InputText(Box<InputText>),
    #[serde(rename = "Input.Toggle")]
    InputToggle(Box<InputToggle>),
    #[serde(rename = "Media")]
    Media(Box<Media>),
    #[serde(rename = "Table")]
    Table(Box<Table>),
    #[serde(rename = "RichTextBlock")]
    RichTextBlock(Box<RichTextBlock>),
    #[serde(rename = "ColumnSet")]
    ColumnSet(Box<ColumnSet>),
    #[serde(rename = "Image")]
    Image(Box<Image>),
    #[serde(rename = "ImageSet")]
    ImageSet(Box<ImageSet>),
    #[serde(rename = "TextBlock")]
    TextBlock(Box<TextBlock>),
    #[serde(rename = "ActionSet")]
    ActionSet(Box<ActionSet>),
    #[serde(rename = "FactSet")]
    FactSet(Box<FactSet>),
    #[serde(rename = "Container")]
    Container(Box<Container>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnSet {
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "columns")]
    pub columns: Option<Vec<Column>>,
    #[serde(rename = "style")]
    pub style: Option<Option<ContainerStyle>>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Media {
    #[serde(rename = "poster")]
    pub poster: Option<String>,
    #[serde(rename = "sources")]
    pub sources: Vec<MediaSource>,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    #[serde(rename = "captionSources")]
    pub caption_sources: Option<Vec<CaptionSource>>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RichTextBlock {
    #[serde(rename = "inlines")]
    pub inlines: Vec<Inline>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
pub enum Input {
    #[serde(rename = "Input.ChoiceSet")]
    ChoiceSet(Box<InputChoiceSet>),
    #[serde(rename = "Input.Time")]
    Time(Box<InputTime>),
    #[serde(rename = "Input.Number")]
    Number(Box<InputNumber>),
    #[serde(rename = "Input.Date")]
    Date(Box<InputDate>),
    #[serde(rename = "Input.Text")]
    Text(Box<InputText>),
    #[serde(rename = "Input.Toggle")]
    Toggle(Box<InputToggle>),
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputTime {
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "max")]
    pub max: Option<String>,
    #[serde(rename = "min")]
    pub min: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputNumber {
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[serde(rename = "max")]
    pub max: Option<f64>,
    #[serde(rename = "min")]
    pub min: Option<f64>,
    #[serde(rename = "placeholder")]
    pub placeholder: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "labelWidth")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub label_width: Option<StringOrNumber>,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "inputStyle")]
    pub input_style: Option<InputStyle>,
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(rename = "labelPosition")]
    pub label_position: Option<InputLabelPosition>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextBlock {
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<Option<bool>>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "size")]
    pub size: Option<Option<FontSize>>,
    #[serde(rename = "color")]
    pub color: Option<Option<Colors>>,
    #[serde(rename = "fontType")]
    pub font_type: Option<Option<FontType>>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<Option<HorizontalAlignment>>,
    #[serde(rename = "weight")]
    pub weight: Option<Option<FontWeight>>,
    #[serde(rename = "wrap")]
    pub wrap: Option<bool>,
    #[serde(rename = "maxLines")]
    pub max_lines: Option<f64>,
    #[serde(rename = "style")]
    pub style: Option<Option<TextBlockStyle>>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
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
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_column_or_fallback_option_optional")]
    pub fallback: Option<ColumnOrFallbackOption>,
    #[serde(rename = "style")]
    pub style: Option<Option<ContainerStyle>>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<Option<VerticalContentAlignment>>,
    #[serde(rename = "backgroundImage")]
    pub background_image: Option<BackgroundImage>,
    #[serde(rename = "items")]
    pub items: Option<Vec<Element>>,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "width")]
    #[serde(deserialize_with = "deserialize_string_or_number_optional")]
    pub width: Option<StringOrNumber>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Container {
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
    #[serde(rename = "style")]
    pub style: Option<Option<ContainerStyle>>,
    #[serde(rename = "backgroundImage")]
    pub background_image: Option<BackgroundImage>,
    #[serde(rename = "bleed")]
    pub bleed: Option<bool>,
    #[serde(rename = "selectAction")]
    pub select_action: Option<ISelectAction>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<Option<VerticalContentAlignment>>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<String>,
    #[serde(rename = "height")]
    pub height: Option<BlockElementHeight>,
    #[serde(rename = "spacing")]
    pub spacing: Option<Spacing>,
    #[serde(rename = "separator")]
    pub separator: Option<bool>,
    #[serde(rename = "fallback")]
    #[serde(deserialize_with = "deserialize_element_or_fallback_option_optional")]
    pub fallback: Option<ElementOrFallbackOption>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "requires")]
    pub requires: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
