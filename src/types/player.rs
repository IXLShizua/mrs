use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    pub username: String,
    pub uuid: Uuid,
}
