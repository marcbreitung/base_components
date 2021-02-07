#[derive(Clone, PartialEq)]
pub enum ColorScheme {
    Input,
    Readonly,
    Error,
}

pub fn create_default_color_scheme() -> ColorScheme {
    ColorScheme::Input
}
