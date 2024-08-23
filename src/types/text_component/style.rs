use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Style {
    pub obfuscated: Option<bool>,
    pub bold: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
    pub italic: Option<bool>,
}

impl Style {
    pub fn builder() -> StyleBuilder {
        StyleBuilder {
            style: Style::default(),
        }
    }
}

#[derive(Debug)]
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    pub fn set_obfuscated(mut self, obfuscated: bool) -> Self {
        self.style.obfuscated = Some(obfuscated);
        self
    }

    pub fn set_bold(mut self, bold: bool) -> Self {
        self.style.bold = Some(bold);
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        self.style.strikethrough = Some(strikethrough);
        self
    }

    pub fn set_underline(mut self, underline: bool) -> Self {
        self.style.underline = Some(underline);
        self
    }

    pub fn set_italic(mut self, italic: bool) -> Self {
        self.style.italic = Some(italic);
        self
    }
}
