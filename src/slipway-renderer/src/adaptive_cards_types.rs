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
    pub providerId: String,
    pub uri: String,
}
pub struct Metadata {
    pub webUrl: String,
}
pub struct Authentication {
    pub text: String,
    pub tokenExchangeResource: TokenExchangeResource,
    pub connectionName: String,
    pub buttons: Vec<AuthCardButton>,
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
    pub type_: String,
    pub value: String,
    pub title: String,
    pub image: String,
}
pub struct TableRow {
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
    pub cells: Vec<TableCell>,
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
pub enum String_or_f64 {
    String(String),
    f64(f64),
}
pub struct TableColumnDefinition {
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
    pub width: String_or_f64,
}
pub struct FactSet {
    pub facts: Vec<Fact>,
}
pub enum String_or_f64 {
    String(String),
    f64(f64),
}
pub enum Column_or_FallbackOption {
    Column(Column),
    FallbackOption(FallbackOption),
}
pub struct Column {
    pub minHeight: String,
    pub separator: bool,
    pub style: Option<ContainerStyle>,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub bleed: bool,
    pub spacing: Spacing,
    pub width: String_or_f64,
    pub rtl: Option<bool>,
    pub items: Vec<Element>,
    pub fallback: Column_or_FallbackOption,
    pub selectAction: ISelectAction,
    pub backgroundImage: BackgroundImage,
}
pub struct CaptionSource {
    pub url: String,
    pub mimeType: String,
    pub label: String,
}
pub struct TextRun {
    pub underline: bool,
    pub italic: bool,
    pub highlight: bool,
    pub selectAction: ISelectAction,
    pub fontType: Option<FontType>,
    pub color: Option<Colors>,
    pub strikethrough: bool,
    pub size: Option<FontSize>,
    pub text: String,
    pub weight: Option<FontWeight>,
    pub isSubtle: Option<bool>,
}
pub struct Inline {}
pub struct Container {
    pub items: Vec<Element>,
    pub style: Option<ContainerStyle>,
    pub backgroundImage: BackgroundImage,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub bleed: bool,
    pub minHeight: String,
    pub rtl: Option<bool>,
    pub selectAction: ISelectAction,
}
pub struct Media {
    pub captionSources: Vec<CaptionSource>,
    pub poster: String,
    pub altText: String,
    pub sources: Vec<MediaSource>,
}
pub struct TextBlock {
    pub color: Option<Colors>,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub fontType: Option<FontType>,
    pub isSubtle: Option<bool>,
    pub style: Option<TextBlockStyle>,
    pub text: String,
    pub size: Option<FontSize>,
    pub weight: Option<FontWeight>,
    pub maxLines: f64,
    pub wrap: bool,
}
pub struct Table {
    pub showGridLines: bool,
    pub verticalCellContentAlignment: Option<VerticalAlignment>,
    pub columns: Vec<TableColumnDefinition>,
    pub horizontalCellContentAlignment: Option<HorizontalAlignment>,
    pub firstRowAsHeader: bool,
    pub rows: Vec<TableRow>,
    pub gridStyle: Option<ContainerStyle>,
}
pub struct ColumnSet {
    pub style: Option<ContainerStyle>,
    pub bleed: bool,
    pub columns: Vec<Column>,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub selectAction: ISelectAction,
    pub minHeight: String,
}
pub struct ActionSet {
    pub actions: Vec<Action>,
}
pub enum String_or_BlockElementHeight {
    String(String),
    BlockElementHeight(BlockElementHeight),
}
pub struct Image {
    pub selectAction: ISelectAction,
    pub height: String_or_BlockElementHeight,
    pub size: ImageSize,
    pub url: String,
    pub width: String,
    pub style: ImageStyle,
    pub altText: String,
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub backgroundColor: String,
}
pub enum Element_or_FallbackOption {
    Element(Element),
    FallbackOption(FallbackOption),
}
pub struct Element {
    pub separator: bool,
    pub height: BlockElementHeight,
    pub fallback: Element_or_FallbackOption,
    pub spacing: Spacing,
}
pub struct RichTextBlock {
    pub horizontalAlignment: Option<HorizontalAlignment>,
    pub inlines: Vec<Inline>,
}
pub struct TableCell {
    pub style: Option<ContainerStyle>,
    pub verticalContentAlignment: Option<VerticalContentAlignment>,
    pub bleed: bool,
    pub selectAction: ISelectAction,
    pub items: Vec<Element>,
    pub backgroundImage: BackgroundImage,
    pub minHeight: String,
    pub rtl: Option<bool>,
}
pub struct MediaSource {
    pub mimeType: String,
    pub url: String,
}
pub struct Input {
    pub max: String,
    pub placeholder: String,
    pub min: String,
    pub value: String,
}
pub struct Input {
    pub value: String,
    pub title: String,
}
pub struct Input {
    pub value: String,
    pub choices: Vec<Input_Choice>,
    pub placeholder: String,
    pub isMultiSelect: bool,
    pub style: ChoiceInputStyle,
    pub choices_data: Data_Query,
    pub wrap: bool,
}
pub enum String_or_f64 {
    String(String),
    f64(f64),
}
pub struct Input {
    pub labelWidth: String_or_f64,
    pub isRequired: bool,
    pub labelPosition: InputLabelPosition,
    pub id: String,
    pub errorMessage: String,
    pub label: String,
    pub inputStyle: InputStyle,
}
pub struct Input {
    pub value: String,
    pub valueOff: String,
    pub title: String,
    pub valueOn: String,
    pub wrap: bool,
}
pub struct Input {
    pub inlineAction: ISelectAction,
    pub style: TextInputStyle,
    pub maxLength: f64,
    pub regex: String,
    pub isMultiline: bool,
    pub value: String,
    pub placeholder: String,
}
pub struct Input {
    pub max: String,
    pub min: String,
    pub placeholder: String,
    pub value: String,
}
pub struct Data {
    pub skip: f64,
    pub count: f64,
    pub dataset: String,
}
pub struct Input {
    pub min: f64,
    pub max: f64,
    pub placeholder: String,
    pub value: f64,
}
pub struct TargetElement {
    pub elementId: String,
    pub isVisible: Option<bool>,
}
pub enum Action_or_FallbackOption {
    Action(Action),
    FallbackOption(FallbackOption),
}
pub struct Action {
    pub iconUrl: String,
    pub id: String,
    pub isEnabled: bool,
    pub title: String,
    pub tooltip: String,
    pub style: ActionStyle,
    pub fallback: Action_or_FallbackOption,
    pub mode: ActionMode,
}
pub enum String_or_object {
    String(String),
    object(object),
}
pub struct Action {
    pub associatedInputs: AssociatedInputs,
    pub data: String_or_object,
}
pub struct ISelectAction {}
pub struct Action {
    pub url: String,
}
pub enum String_or_object {
    String(String),
    object(object),
}
pub struct Action {
    pub associatedInputs: AssociatedInputs,
    pub data: String_or_object,
    pub verb: String,
}
pub struct Action {
    pub card: AdaptiveCard,
}
pub struct Action {
    pub targetElements: Vec<TargetElement>,
}
pub struct AdaptiveCard {
    pub minHeight: String,
    pub lang: String,
    pub authentication: Authentication,
    pub rtl: Option<bool>,
    pub backgroundImage: BackgroundImage,
    pub version: String,
    pub body: Vec<Element>,
    pub refresh: Refresh,
    pub speak: String,
    pub selectAction: ISelectAction,
    pub _schema: uri,
    pub verticalContentAlignment: VerticalContentAlignment,
    pub metadata: Metadata,
    pub actions: Vec<Action>,
    pub fallbackText: String,
}
