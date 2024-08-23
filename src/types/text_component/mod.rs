mod click_event;
mod color;
mod hover_event;
mod style;

use serde::{Deserialize, Serialize};

pub use click_event::*;
pub use color::*;
pub use hover_event::*;
pub use style::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent {
    #[serde(rename = "type")]
    pub _type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<TextComponent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    #[serde(flatten)]
    pub style: Style,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,

    // TODO fix hover event
    // pub hover_event: Option<HoverEvent>,
}
