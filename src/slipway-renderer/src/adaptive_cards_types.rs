pub enum ImageSize {
    auto,
    stretch,
    small,
    medium,
    large,
}
pub enum FontSize {
    default,
    small,
    medium,
    large,
    extraLarge,
}
pub enum VerticalAlignment {
    top,
    center,
    bottom,
}
pub enum ActionStyle {
    default,
    positive,
    destructive,
}
pub enum VerticalContentAlignment {
    top,
    center,
    bottom,
}
pub enum FontType {
    default,
    monospace,
}
pub enum HorizontalAlignment {
    left,
    center,
    right,
}
pub enum FallbackOption {
    drop,
}
pub enum AssociatedInputs {
    Auto,
    None,
}
pub enum TextInputStyle {
    text,
    tel,
    url,
    email,
    password,
}
pub enum ChoiceInputStyle {
    compact,
    expanded,
    filtered,
}
pub enum ImageFillMode {
    cover,
    repeatHorizontally,
    repeatVertically,
    repeat,
}
pub enum TextBlockStyle {
    default,
    heading,
}
pub enum Spacing {
    default,
    none,
    small,
    medium,
    large,
    extraLarge,
    padding,
}
pub enum ContainerStyle {
    default,
    emphasis,
    good,
    attention,
    warning,
    accent,
}
pub enum ImageSetStyle {
    default,
    stacked,
    grid,
}
pub enum InputStyle {
    revealOnHover,
    default,
}
pub enum ImageStyle {
    default,
    person,
}
pub enum InputLabelPosition {
    inline,
    above,
}
pub enum FontWeight {
    default,
    lighter,
    bolder,
}
pub enum ActionMode {
    primary,
    secondary,
}
pub enum Colors {
    default,
    dark,
    light,
    accent,
    good,
    warning,
    attention,
}
pub enum BlockElementHeight {
    auto,
    stretch,
}
pub struct Item {
    pub requires: std::collections::HashMap<String, String>,
}
pub struct TokenExchangeResource {
    pub id: String,
    pub uri: String,
    pub providerId: String,
}
pub struct Metadata {
    pub webUrl: String,
}
pub struct Authentication {
    pub buttons: Vec<AuthCardButton>,
    pub text: String,
    pub connectionName: String,
    pub tokenExchangeResource: TokenExchangeResource,
}
pub struct BackgroundImage {
    pub fillMode: ImageFillMode,
    pub horizontalAlignment: HorizontalAlignment,
    pub url: String,
    pub verticalAlignment: VerticalAlignment,
}
pub struct Refresh {
    pub expires: String,
    pub userIds: Vec<String>,
    pub action: Action_Execute,
}
pub struct ToggleableItem {
    pub isVisible: bool,
    pub id: String,
}
pub struct AuthCardButton {
    pub value: String,
    pub image: String,
    pub type_: String,
    pub title: String,
}
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
    pub style: Option<ContainerStyle>,
}
pub struct ImageSet {
    pub images: Vec<Image>,
    pub style: ImageSetStyle,
    pub imageSize: ImageSize,
}
pub struct Fact {
    pub title: String,
    pub value: String,
}
pub enum string_or_number {
    string(String),
    number(f64),
}
pub struct TableColumnDefinition {
    pub width: string_or_number,
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
}
pub struct FactSet {
    pub facts: Vec<Fact>,
}
pub enum Column_or_FallbackOption {
    Column(Box<Column>),
    FallbackOption(FallbackOption),
}
pub struct Column {
    pub rtl: Option<bool>,
    pub separator: bool,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub width: string_or_number,
    pub items: Vec<Element>,
    pub spacing: Spacing,
    pub style: Option<ContainerStyle>,
    pub bleed: bool,
    pub selectAction: ISelectAction,
    pub backgroundImage: BackgroundImage,
    pub fallback: Column_or_FallbackOption,
    pub minHeight: String,
}
pub struct CaptionSource {
    pub mimeType: String,
    pub url: String,
    pub label: String,
}
pub struct TextRun {
    pub italic: bool,
    pub strikethrough: bool,
    pub size: Option<FontSize>,
    pub isSubtle: Option<bool>,
    pub fontType: Option<FontType>,
    pub color: Option<Colors>,
    pub text: String,
    pub underline: bool,
    pub selectAction: ISelectAction,
    pub weight: Option<FontWeight>,
    pub highlight: bool,
}
pub struct Inline {}
pub struct Container {
    pub backgroundImage: BackgroundImage,
    pub items: Vec<Element>,
    pub style: Option<ContainerStyle>,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub bleed: bool,
    pub selectAction: ISelectAction,
    pub rtl: Option<bool>,
    pub minHeight: String,
}
pub struct Media {
    pub poster: String,
    pub sources: Vec<MediaSource>,
    pub captionSources: Vec<CaptionSource>,
    pub altText: String,
}
pub struct TextBlock {
    pub maxLines: f64,
    pub size: Option<FontSize>,
    pub text: String,
    pub color: Option<Colors>,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub isSubtle: Option<bool>,
    pub fontType: Option<FontType>,
    pub wrap: bool,
    pub style: Option<TextBlockStyle>,
    pub weight: Option<FontWeight>,
}
pub struct Table {
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub gridStyle: Option<ContainerStyle>,
    pub firstRowAsHeader: bool,
    pub showGridLines: bool,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
    pub columns: Vec<TableColumnDefinition>,
    pub rows: Vec<TableRow>,
}
pub struct ColumnSet {
    pub columns: Vec<Column>,
    pub selectAction: ISelectAction,
    pub bleed: bool,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub style: Option<ContainerStyle>,
    pub minHeight: String,
}
pub struct ActionSet {
    pub actions: Vec<Action>,
}
pub enum string_or_BlockElementHeight {
    string(String),
    BlockElementHeight(BlockElementHeight),
}
pub struct Image {
    pub size: ImageSize,
    pub url: String,
    pub altText: String,
    pub selectAction: ISelectAction,
    pub width: String,
    pub backgroundColor: String,
    pub height: string_or_BlockElementHeight,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub style: ImageStyle,
}
pub enum Element_or_FallbackOption {
    Element(Box<Element>),
    FallbackOption(FallbackOption),
}
pub struct Element {
    pub height: BlockElementHeight,
    pub spacing: Spacing,
    pub fallback: Element_or_FallbackOption,
    pub separator: bool,
}
pub struct RichTextBlock {
    pub inlines: Vec<Inline>,
    pub horizontalAlignment: Option<HorizontalAlignment>,
}
pub struct TableCell {
    pub bleed: bool,
    pub rtl: Option<bool>,
    pub selectAction: ISelectAction,
    pub style: Option<ContainerStyle>,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub items: Vec<Element>,
    pub minHeight: String,
    pub backgroundImage: BackgroundImage,
}
pub struct MediaSource {
    pub mimeType: String,
    pub url: String,
}
pub struct Input_Time {
    pub max: String,
    pub min: String,
    pub value: String,
    pub placeholder: String,
}
pub struct Input_Choice {
    pub title: String,
    pub value: String,
}
pub struct Input_ChoiceSet {
    pub value: String,
    pub choices_data: Data_Query,
    pub placeholder: String,
    pub wrap: bool,
    pub choices: Vec<Input_Choice>,
    pub style: ChoiceInputStyle,
    pub isMultiSelect: bool,
}
pub struct Input {
    pub id: String,
    pub inputStyle: InputStyle,
    pub label: String,
    pub errorMessage: String,
    pub isRequired: bool,
    pub labelPosition: InputLabelPosition,
    pub labelWidth: string_or_number,
}
pub struct Input_Toggle {
    pub title: String,
    pub valueOff: String,
    pub valueOn: String,
    pub wrap: bool,
    pub value: String,
}
pub struct Input_Text {
    pub inlineAction: ISelectAction,
    pub isMultiline: bool,
    pub regex: String,
    pub maxLength: f64,
    pub placeholder: String,
    pub style: TextInputStyle,
    pub value: String,
}
pub struct Input_Date {
    pub placeholder: String,
    pub min: String,
    pub value: String,
    pub max: String,
}
pub struct Data_Query {
    pub dataset: String,
    pub skip: f64,
    pub count: f64,
}
pub struct Input_Number {
    pub max: f64,
    pub value: f64,
    pub min: f64,
    pub placeholder: String,
}
pub struct TargetElement {
    pub elementId: String,
    pub isVisible: Option<bool>,
}
pub enum Action_or_FallbackOption {
    Action(Box<Action>),
    FallbackOption(FallbackOption),
}
pub struct Action {
    pub isEnabled: bool,
    pub iconUrl: String,
    pub fallback: Action_or_FallbackOption,
    pub style: ActionStyle,
    pub tooltip: String,
    pub title: String,
    pub id: String,
    pub mode: ActionMode,
}
pub enum string_or_object {
    string(String),
    object(serde_json::Value),
}
pub struct Action_Submit {
    pub data: string_or_object,
    pub associatedInputs: AssociatedInputs,
}
pub struct ISelectAction {}
pub struct Action_OpenUrl {
    pub url: String,
}
pub struct Action_Execute {
    pub verb: String,
    pub data: string_or_object,
    pub associatedInputs: AssociatedInputs,
}
pub struct Action_ShowCard {
    pub card: AdaptiveCard,
}
pub struct Action_ToggleVisibility {
    pub targetElements: Vec<TargetElement>,
}
pub struct AdaptiveCard {
    pub rtl: Option<bool>,
    pub metadata: Metadata,
    pub selectAction: ISelectAction,
    pub version: String,
    pub verticalContentAlignment: VerticalContentAlignment,
    pub backgroundImage: BackgroundImage,
    pub refresh: Refresh,
    pub speak: String,
    pub actions: Vec<Action>,
    pub lang: String,
    pub body: Vec<Element>,
    pub minHeight: String,
    pub _schema: String,
    pub fallbackText: String,
    pub authentication: Authentication,
}
