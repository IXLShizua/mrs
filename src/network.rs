use crate::connection::Connection;
use crate::raw::RawPacket;
use futures::{stream, StreamExt};
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use crate::packets::handshake::HandshakePacket;

#[derive(Debug)]
pub struct Network {
    listener: TcpListener,
    connections: HashMap<SocketAddr, Connection>,
}

impl Network {
    pub fn new(listener: TcpListener) -> Network {
        Network {
            listener,
            connections: HashMap::new(),
        }
    }

    async fn accept_loop(&mut self) {
        // while let Ok((stream, addr)) = self.listener.accept().await {
        //     let (incoming_rx, rx) = mpsc::channel(128);
        //     let (tx, outgoing_rx) = mpsc::channel(128);
        //     self.connections.insert(addr, Connection::new(stream, rx));
        // 
        //     tokio::spawn(async move {
        //         
        //     });
        // }
    }

    // async fn recv_packets(&mut self) -> Vec<io::Result<RawPacket>> {
    //     stream::iter(self.connections.values_mut())
    //         .map(|conn| conn.recv_packet())
    //         .buffer_unordered(64)
    //         .collect()
    //         .await
    // }
}

async fn handle_handshake(handshake: HandshakePacket) -> io::Result<()> {
    match handshake.next_state {
        1 => handle_handshake_status().await,
        2 => handle_handshake_login().await,
        _ => unreachable!()
    }
}

async fn handle_handshake_status() {}

async fn handle_handshake_login() {}