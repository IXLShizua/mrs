use crate::types::identifier::Identifier;
use crate::types::text_component::TextComponent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "contents")]
pub enum HoverEvent {
    ShowText(TextComponent),
    ShowItem(HoverEventItemStack),
    ShowEntity(HoverEventEntity),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverEventItemStack {
    pub id: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverEventEntity {
    #[serde(rename = "type")]
    pub _type: Identifier,

    pub id: Uuid,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
