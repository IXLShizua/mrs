#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}
