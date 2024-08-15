#[derive(Debug)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}
