use std::error::Error;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    Ok(())
}

// async fn handle_initial_connection(conn: &mut Connection) -> io::Result<()> {
//     let handshake = conn
//         .recv_packet()
//         .await?
//         .encode::<HandshakePacket>()
//         .unwrap();
//     println!("{:?}", handshake);
//
//     if handshake.next_state == 1 {
//         let status_request = conn
//             .recv_packet()
//             .await?
//             .encode::<StatusRequestPacket>()
//             .unwrap();
//         conn.send_packet(StatusResponsePacket::new()).await?;
//
//         let ping_request = conn
//             .recv_packet()
//             .await?
//             .encode::<PingRequestPacket>()
//             .unwrap();
//         conn.send_packet(PingResponsePacket).await?;
//     } else if handshake.next_state == 2 {
//         let login_start = conn
//             .recv_packet()
//             .await?
//             .encode::<LoginStartPacket>()
//             .unwrap();
//         println!("{:?}", login_start);
//
//         let login_success = LoginSuccessPacket {
//             username: login_start.name,
//             uuid: login_start.player_uuid,
//             properties: vec![],
//             strict_error_handling: false,
//         };
//         conn.send_packet(login_success).await?;
//
//         let login_ack = conn
//             .recv_packet()
//             .await?
//             .encode::<LoginAckPacket>()
//             .unwrap();
//         println!("{:?}", login_ack);
//
//         // TODO
//         // let client_info = conn
//         //     .recv_packet()
//         //     .await?
//         //     .encode::<ClientInformationPacket>()
//         //     .unwrap();
//         // println!("{:?}", client_info);
//
//         let server_known_packs = KnownPacksPacket {
//             known_packs: Vec::new(),
//         };
//         conn.send_packet(server_known_packs).await?;
//
//         let client_known_packs = conn
//             .recv_packet()
//             .await?
//             .encode::<KnownPacksPacket>()
//             .unwrap();
//         println!("{:?}", client_known_packs);
//     }
//
//     Ok(())
// }
