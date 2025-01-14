use image::RgbaImage;

// We have no knowledge of WASM or WIT files here, so define our own struct.
pub struct ResolvedFont {
    pub family: String,
    pub data: Vec<u8>,
}

pub struct ComponentError {
    pub message: String,
    pub inner: Vec<String>,
}

pub trait HostContext {
    fn try_resolve_font(&self, font_stack: &str) -> Option<ResolvedFont>;
    fn load_image_from_url(
        &self,
        url: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<RgbaImage, ComponentError>;
    fn log_warn(&self, message: &str);
    fn log_debug(&self, message: &str);
}
