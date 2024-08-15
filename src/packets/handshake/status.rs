use crate::raw::{PacketError, PacketWriteExt, RawPacket};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponsePacket {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    pub enforces_secure_chat: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<PlayersSample>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayersSample {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}

impl StatusResponsePacket {
    pub fn new() -> StatusResponsePacket {
        StatusResponsePacket {
            version: Version {
                name: "Rewerys".to_string(),
                protocol: 767,
            },
            players: Players {
                max: 100,
                online: 0,
                sample: Vec::new(),
            },
            description: Description {
                text: "Rustlang Server - Example description".to_string(),
            },
            enforces_secure_chat: false,
        }
    }
}

impl TryFrom<StatusResponsePacket> for RawPacket {
    type Error = PacketError;

    fn try_from(value: StatusResponsePacket) -> Result<Self, Self::Error> {
        let mut raw = RawPacket::new(0x00);
        raw.write_string(&serde_json::to_string(&value).unwrap())?;

        Ok(raw)
    }
}

#[derive(Debug)]
pub struct StatusRequestPacket;

impl TryFrom<RawPacket> for StatusRequestPacket {
    type Error = PacketError;

    fn try_from(value: RawPacket) -> Result<Self, Self::Error> {
        if value.id == 0x00 {
            Ok(StatusRequestPacket)
        } else {
            Err(PacketError::WrongId)
        }
    }
}
