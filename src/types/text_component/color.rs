use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
    Hex(String),

    #[serde(rename = "black")]
    Black,

    #[serde(rename = "dark_blue")]
    DarkBlue,

    #[serde(rename = "dark_green")]
    DarkGreen,

    #[serde(rename = "dark_aqua")]
    DarkCyan,

    #[serde(rename = "dark_red")]
    DarkRed,

    #[serde(rename = "dark_purple")]
    Purple,

    #[serde(rename = "gold")]
    Gold,

    #[serde(rename = "gray")]
    Gray,

    #[serde(rename = "dark_gray")]
    DarkGray,

    #[serde(rename = "blue")]
    Blue,

    #[serde(rename = "green")]
    BrightGreen,

    #[serde(rename = "aqua")]
    Cyan,

    #[serde(rename = "red")]
    Red,

    #[serde(rename = "light_purple")]
    Pink,

    #[serde(rename = "yellow")]
    Yellow,

    #[serde(rename = "white")]
    White,
}

// impl Deserialize for Color {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         todo!()
//     }
// }