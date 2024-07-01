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
pub struct Item {
    #[serde(rename = "requires")]
    pub requires: std::collections::HashMap<String, String>,
}
#[derive(serde::Deserialize)]
pub struct TokenExchangeResource {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(serde::Deserialize)]
pub struct Metadata {
    #[serde(rename = "webUrl")]
    pub web_url: String,
}
#[derive(serde::Deserialize)]
pub struct Authentication {
    #[serde(rename = "buttons")]
    pub buttons: Vec<AuthCardButton>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    #[serde(rename = "tokenExchangeResource")]
    pub token_exchange_resource: TokenExchangeResource,
}
#[derive(serde::Deserialize)]
pub struct BackgroundImage {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "fillMode")]
    pub fill_mode: ImageFillMode,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: HorizontalAlignment,
    #[serde(rename = "verticalAlignment")]
    pub vertical_alignment: VerticalAlignment,
}
#[derive(serde::Deserialize)]
pub struct Refresh {
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
    #[serde(rename = "expires")]
    pub expires: String,
    #[serde(rename = "action")]
    pub action: ActionExecute,
}
#[derive(serde::Deserialize)]
pub struct ToggleableItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
}
#[derive(serde::Deserialize)]
pub struct AuthCardButton {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "image")]
    pub image: String,
}
#[derive(serde::Deserialize)]
pub struct TableRow {
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "cells")]
    pub cells: Vec<TableCell>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
}
#[derive(serde::Deserialize)]
pub struct ImageSet {
    #[serde(rename = "images")]
    pub images: Vec<Image>,
    #[serde(rename = "imageSize")]
    pub image_size: ImageSize,
    #[serde(rename = "style")]
    pub style: ImageSetStyle,
}
#[derive(serde::Deserialize)]
pub struct Fact {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
pub enum StringOrNumber {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "number")]
    Number(f64),
}
#[derive(serde::Deserialize)]
pub struct TableColumnDefinition {
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "width")]
    pub width: StringOrNumber,
}
#[derive(serde::Deserialize)]
pub struct FactSet {
    #[serde(rename = "facts")]
    pub facts: Vec<Fact>,
}
#[derive(serde::Deserialize)]
pub enum ColumnOrFallbackOption {
    #[serde(rename = "Column")]
    Column(Box<Column>),
    #[serde(rename = "FallbackOption")]
    FallbackOption(FallbackOption),
}
#[derive(serde::Deserialize)]
pub struct Column {
    #[serde(rename = "bleed")]
    pub bleed: bool,
    #[serde(rename = "fallback")]
    pub fallback: ColumnOrFallbackOption,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "backgroundImage")]
    pub background_image: BackgroundImage,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "minHeight")]
    pub min_height: String,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "separator")]
    pub separator: bool,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "spacing")]
    pub spacing: Spacing,
    #[serde(rename = "width")]
    pub width: StringOrNumber,
}
#[derive(serde::Deserialize)]
pub struct CaptionSource {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
#[derive(serde::Deserialize)]
pub struct TextRun {
    #[serde(rename = "color")]
    pub color: Option<Colors>,
    #[serde(rename = "fontType")]
    pub font_type: Option<FontType>,
    #[serde(rename = "size")]
    pub size: Option<FontSize>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "underline")]
    pub underline: bool,
    #[serde(rename = "italic")]
    pub italic: bool,
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<bool>,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "weight")]
    pub weight: Option<FontWeight>,
    #[serde(rename = "strikethrough")]
    pub strikethrough: bool,
    #[serde(rename = "highlight")]
    pub highlight: bool,
}
#[derive(serde::Deserialize)]
pub struct Inline {}
#[derive(serde::Deserialize)]
pub struct Container {
    #[serde(rename = "minHeight")]
    pub min_height: String,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
    #[serde(rename = "bleed")]
    pub bleed: bool,
    #[serde(rename = "backgroundImage")]
    pub background_image: BackgroundImage,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
}
#[derive(serde::Deserialize)]
pub struct Media {
    #[serde(rename = "poster")]
    pub poster: String,
    #[serde(rename = "sources")]
    pub sources: Vec<MediaSource>,
    #[serde(rename = "captionSources")]
    pub caption_sources: Vec<CaptionSource>,
    #[serde(rename = "altText")]
    pub alt_text: String,
}
#[derive(serde::Deserialize)]
pub struct TextBlock {
    #[serde(rename = "style")]
    pub style: Option<TextBlockStyle>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "fontType")]
    pub font_type: Option<FontType>,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "wrap")]
    pub wrap: bool,
    #[serde(rename = "isSubtle")]
    pub is_subtle: Option<bool>,
    #[serde(rename = "color")]
    pub color: Option<Colors>,
    #[serde(rename = "maxLines")]
    pub max_lines: f64,
    #[serde(rename = "size")]
    pub size: Option<FontSize>,
    #[serde(rename = "weight")]
    pub weight: Option<FontWeight>,
}
#[derive(serde::Deserialize)]
pub struct Table {
    #[serde(rename = "verticalCellContentAlignment")]
    pub vertical_cell_content_alignment: Option<VerticalAlignment>,
    #[serde(rename = "columns")]
    pub columns: Vec<TableColumnDefinition>,
    #[serde(rename = "firstRowAsHeader")]
    pub first_row_as_header: bool,
    #[serde(rename = "horizontalCellContentAlignment")]
    pub horizontal_cell_content_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "showGridLines")]
    pub show_grid_lines: bool,
    #[serde(rename = "gridStyle")]
    pub grid_style: Option<ContainerStyle>,
    #[serde(rename = "rows")]
    pub rows: Vec<TableRow>,
}
#[derive(serde::Deserialize)]
pub struct ColumnSet {
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "minHeight")]
    pub min_height: String,
    #[serde(rename = "columns")]
    pub columns: Vec<Column>,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "bleed")]
    pub bleed: bool,
}
#[derive(serde::Deserialize)]
pub struct ActionSet {
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
}
#[derive(serde::Deserialize)]
pub enum StringOrBlockElementHeight {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "BlockElementHeight")]
    BlockElementHeight(BlockElementHeight),
}
#[derive(serde::Deserialize)]
pub struct Image {
    #[serde(rename = "size")]
    pub size: ImageSize,
    #[serde(rename = "height")]
    pub height: StringOrBlockElementHeight,
    #[serde(rename = "width")]
    pub width: String,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "style")]
    pub style: ImageStyle,
    #[serde(rename = "altText")]
    pub alt_text: String,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
}
#[derive(serde::Deserialize)]
pub enum ElementOrFallbackOption {
    #[serde(rename = "Element")]
    Element(Box<Element>),
    #[serde(rename = "FallbackOption")]
    FallbackOption(FallbackOption),
}
#[derive(serde::Deserialize)]
pub struct Element {
    #[serde(rename = "height")]
    pub height: BlockElementHeight,
    #[serde(rename = "separator")]
    pub separator: bool,
    #[serde(rename = "fallback")]
    pub fallback: ElementOrFallbackOption,
    #[serde(rename = "spacing")]
    pub spacing: Spacing,
}
#[derive(serde::Deserialize)]
pub struct RichTextBlock {
    #[serde(rename = "inlines")]
    pub inlines: Vec<Inline>,
    #[serde(rename = "horizontalAlignment")]
    pub horizontal_alignment: Option<HorizontalAlignment>,
}
#[derive(serde::Deserialize)]
pub struct TableCell {
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "rtl")]
    pub rtl: Option<Option<bool>>,
    #[serde(rename = "bleed")]
    pub bleed: bool,
    #[serde(rename = "items")]
    pub items: Vec<Element>,
    #[serde(rename = "backgroundImage")]
    pub background_image: BackgroundImage,
    #[serde(rename = "minHeight")]
    pub min_height: String,
    #[serde(rename = "style")]
    pub style: Option<ContainerStyle>,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: Option<VerticalContentAlignment>,
}
#[derive(serde::Deserialize)]
pub struct MediaSource {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(serde::Deserialize)]
pub struct InputTime {
    #[serde(rename = "max")]
    pub max: String,
    #[serde(rename = "min")]
    pub min: String,
    #[serde(rename = "placeholder")]
    pub placeholder: String,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
pub struct InputChoice {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
pub struct InputChoiceSet {
    #[serde(rename = "wrap")]
    pub wrap: bool,
    #[serde(rename = "choices")]
    pub choices: Vec<InputChoice>,
    #[serde(rename = "isMultiSelect")]
    pub is_multi_select: bool,
    #[serde(rename = "placeholder")]
    pub placeholder: String,
    #[serde(rename = "choices.data")]
    pub choices_data: DataQuery,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "style")]
    pub style: ChoiceInputStyle,
}
#[derive(serde::Deserialize)]
pub struct Input {
    #[serde(rename = "labelPosition")]
    pub label_position: InputLabelPosition,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "inputStyle")]
    pub input_style: InputStyle,
    #[serde(rename = "isRequired")]
    pub is_required: bool,
    #[serde(rename = "labelWidth")]
    pub label_width: StringOrNumber,
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(serde::Deserialize)]
pub struct InputToggle {
    #[serde(rename = "valueOn")]
    pub value_on: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "valueOff")]
    pub value_off: String,
    #[serde(rename = "wrap")]
    pub wrap: bool,
}
#[derive(serde::Deserialize)]
pub struct InputText {
    #[serde(rename = "inlineAction")]
    pub inline_action: ISelectAction,
    #[serde(rename = "regex")]
    pub regex: String,
    #[serde(rename = "maxLength")]
    pub max_length: f64,
    #[serde(rename = "isMultiline")]
    pub is_multiline: bool,
    #[serde(rename = "placeholder")]
    pub placeholder: String,
    #[serde(rename = "style")]
    pub style: TextInputStyle,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
pub struct InputDate {
    #[serde(rename = "placeholder")]
    pub placeholder: String,
    #[serde(rename = "min")]
    pub min: String,
    #[serde(rename = "max")]
    pub max: String,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(serde::Deserialize)]
pub struct DataQuery {
    #[serde(rename = "skip")]
    pub skip: f64,
    #[serde(rename = "count")]
    pub count: f64,
    #[serde(rename = "dataset")]
    pub dataset: String,
}
#[derive(serde::Deserialize)]
pub struct InputNumber {
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "max")]
    pub max: f64,
    #[serde(rename = "min")]
    pub min: f64,
    #[serde(rename = "placeholder")]
    pub placeholder: String,
}
#[derive(serde::Deserialize)]
pub struct TargetElement {
    #[serde(rename = "elementId")]
    pub element_id: String,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
}
#[derive(serde::Deserialize)]
pub enum ActionOrFallbackOption {
    #[serde(rename = "Action")]
    Action(Box<Action>),
    #[serde(rename = "FallbackOption")]
    FallbackOption(FallbackOption),
}
#[derive(serde::Deserialize)]
pub struct Action {
    #[serde(rename = "mode")]
    pub mode: ActionMode,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "fallback")]
    pub fallback: ActionOrFallbackOption,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "tooltip")]
    pub tooltip: String,
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "style")]
    pub style: ActionStyle,
}
#[derive(serde::Deserialize)]
pub enum StringOrObject {
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "object")]
    Object(serde_json::Value),
}
#[derive(serde::Deserialize)]
pub struct ActionSubmit {
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data")]
    pub data: StringOrObject,
}
#[derive(serde::Deserialize)]
pub struct ISelectAction {}
#[derive(serde::Deserialize)]
pub struct ActionOpenUrl {
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(serde::Deserialize)]
pub struct ActionExecute {
    #[serde(rename = "verb")]
    pub verb: String,
    #[serde(rename = "associatedInputs")]
    pub associated_inputs: AssociatedInputs,
    #[serde(rename = "data")]
    pub data: StringOrObject,
}
#[derive(serde::Deserialize)]
pub struct ActionShowCard {
    #[serde(rename = "card")]
    pub card: AdaptiveCard,
}
#[derive(serde::Deserialize)]
pub struct ActionToggleVisibility {
    #[serde(rename = "targetElements")]
    pub target_elements: Vec<TargetElement>,
}
#[derive(serde::Deserialize)]
pub struct AdaptiveCard {
    #[serde(rename = "authentication")]
    pub authentication: Authentication,
    #[serde(rename = "verticalContentAlignment")]
    pub vertical_content_alignment: VerticalContentAlignment,
    #[serde(rename = "fallbackText")]
    pub fallback_text: String,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "speak")]
    pub speak: String,
    #[serde(rename = "refresh")]
    pub refresh: Refresh,
    #[serde(rename = "minHeight")]
    pub min_height: String,
    #[serde(rename = "backgroundImage")]
    pub background_image: BackgroundImage,
    #[serde(rename = "body")]
    pub body: Vec<Element>,
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "rtl")]
    pub rtl: Option<bool>,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "selectAction")]
    pub select_action: ISelectAction,
    #[serde(rename = "actions")]
    pub actions: Vec<Action>,
}
