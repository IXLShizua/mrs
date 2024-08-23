use std::io;
use crate::raw::{RawPacket, TryIntoRawPacket};
use crate::types::var_int;
use std::io::ErrorKind;
use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite, BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

mod state;

pub use state::*;

#[derive(Debug)]
pub struct Connection {
    addr: SocketAddr,
    state: State,
    incoming_receiver: mpsc::Receiver<RawPacket>,
    outgoing_sender: mpsc::Sender<RawPacket>,
    incoming_handle: JoinHandle<()>,
    outgoing_handle: JoinHandle<()>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<RawPacket>(128);
        let (incoming_tx, incoming_rx) = mpsc::channel::<RawPacket>(128);

        let incoming_packets_handle: JoinHandle<()> = tokio::spawn(async move {
            loop {
                match recv_packet(&mut reader).await {
                    Ok(packet) => {
                        println!("New packet: {:?}", packet);

                        if let Err(err) = incoming_tx.send(packet).await {
                            println!("Error with sending packet to channel: {}", err);
                        }
                    }
                    Err(err) => {
                        println!("Error with recv packet from client: {}", err);

                        if let ErrorKind::ConnectionAborted = err.kind() {
                            println!("Closing connection");

                            break;
                        }
                    }
                }
            }
        });
        let outgoing_packets_handle: JoinHandle<()> = tokio::spawn(async move {
            while let Some(packet) = outgoing_rx.recv().await {
                if let Err(err) = send_raw_packet(&mut writer, packet).await {
                    println!("Error with sending packet to client: {}", err);
                }
            }
        });

        todo!();
        // Connection {
        //     state: State::Handshaking,
        //     incoming_receiver: incoming_rx,
        //     outgoing_sender: outgoing_tx,
        //     incoming_handle: incoming_packets_handle,
        //     outgoing_handle: outgoing_packets_handle,
        // }
    }

    pub async fn send_packet<T: TryIntoRawPacket>(&mut self, packet: T) -> io::Result<()> {
        let packet = packet.try_into().map_err(|_| io::ErrorKind::InvalidData)?;

        self.outgoing_sender
            .send(packet)
            .await
            .map_err(|_| ErrorKind::NotConnected.into())
    }

    pub async fn recv_packet(&mut self) -> Option<RawPacket> {
        self.incoming_receiver.recv().await
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.incoming_handle.abort();
        self.outgoing_handle.abort();
    }
}

async fn recv_packet<T: AsyncRead + Unpin>(mut read: T) -> io::Result<RawPacket> {
    let length = var_int::decode(&mut read)
        .await
        .map_err(|_| io::ErrorKind::ConnectionAborted)?;
    let id = var_int::decode(&mut read)
        .await
        .map_err(|_| io::ErrorKind::ConnectionAborted)?;
    let id_len = var_int::length(id);

    let mut buf = vec![0u8; length as usize - id_len];
    read.read_exact(&mut buf).await?;

    let raw = RawPacket::new_with_data(id, &buf);

    Ok(raw)
}

async fn send_raw_packet<T: AsyncWrite + Unpin>(mut write: T, packet: RawPacket) -> io::Result<()> {
    let bytes = Vec::<u8>::try_from(packet).map_err(|_| io::ErrorKind::InvalidData)?;

    write.write_all(&bytes).await
}
