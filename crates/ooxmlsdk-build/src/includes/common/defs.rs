// This is filled in at build time
pub const SUPPORTED_NAMESPACES: &[&str] = &[];

#[allow(dead_code)]
pub fn standard_prefix(_uri: &str) -> Option<&'static str> {
  None
}
