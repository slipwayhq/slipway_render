// We have no knowledge of WASM or WIT files here, so define our own struct.
pub struct ResolvedFont {
    pub family: String,
    pub data: Vec<u8>,
}

pub trait HostContext {
    fn try_resolve_font(&self, font_stack: &str) -> Option<ResolvedFont>;
}
