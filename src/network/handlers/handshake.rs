use crate::network::connection::{Connection, State};
use crate::network::handlers::PacketHandleError;
use crate::packets::handshake::{HandshakeNextState, HandshakePacket, StatusRequestPacket};

pub async fn handle_handshake(
    conn: &mut Connection,
    packet: HandshakePacket,
) -> Result<Option<State>, PacketHandleError> {
    match packet.next_state {
        HandshakeNextState::Status => {
            let status_packet = conn
                .recv_packet()
                .await
                .ok_or(PacketHandleError::PacketNotSend)?
                .encode::<StatusRequestPacket>()?;

            handle_status(conn, status_packet).await;

            Ok(None)
        }
        HandshakeNextState::Login => todo!(),
        HandshakeNextState::Transfer => todo!(),
    }
}

pub async fn handle_status(
    conn: &mut Connection,
    packet: StatusRequestPacket,
) -> Result<(), PacketHandleError> {
    Ok(())
}
