use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TextComponent {
    #[serde(rename = "type")]
    pub _type: Option<String>,

    pub extra: Option<Vec<TextComponent>>,
}
