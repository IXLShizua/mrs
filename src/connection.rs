use crate::raw::{RawPacket, TryIntoRawPacket};
use crate::types::connection_state::ConnectionState;
use crate::types::var_int;
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ConnectionHandle {
    outgoing_sender: mpsc::Sender<RawPacket>,
    incoming_receiver: mpsc::Receiver<RawPacket>,
    connection: Connection,
}

impl ConnectionHandle {
    pub fn new(stream: TcpStream) -> ConnectionHandle {
        let (outgoing_tx, outgoing_rx) = mpsc::channel(128);
        let (incoming_tx, incoming_rx) = mpsc::channel(128);
        let connection = Connection::new(stream, incoming_tx, outgoing_rx);

        ConnectionHandle {
            outgoing_sender: outgoing_tx,
            incoming_receiver: incoming_rx,
            connection,
        }
    }

    async fn send_packet<T: TryIntoRawPacket>(&mut self, packet: T) -> io::Result<()> {
        let raw = packet.try_into().unwrap();

        self.outgoing_sender
            .send(raw)
            .await
            .map_err(|_| io::ErrorKind::NotConnected.into())
    }
}

#[derive(Debug)]
pub struct Connection {
    state: ConnectionState,
    incoming_sender: mpsc::Sender<RawPacket>,
    outgoing_receiver: mpsc::Receiver<RawPacket>,
    read: BufReader<ReadHalf<TcpStream>>,
    write: WriteHalf<TcpStream>,
}

impl Connection {
    pub fn new(
        stream: TcpStream,
        tx: mpsc::Sender<RawPacket>,
        rx: mpsc::Receiver<RawPacket>,
    ) -> Connection {
        let (read, write) = tokio::io::split(stream);

        Connection {
            state: ConnectionState::Handshaking,
            incoming_sender: tx,
            outgoing_receiver: rx,
            read: BufReader::new(read),
            write,
        }
    }

    pub(crate) async fn run(&mut self) {
        loop {
            tokio::select! {
                Ok(incoming) = recv_packet(&mut self.read) => {
                    let _ = self.incoming_sender.send(incoming).await;
                }
                Some(outgoing) = self.outgoing_receiver.recv() => {
                    if let Err(err) = send_raw_packet(&mut self.write, outgoing).await {
                        println!("Error with sending packet: {}", err);
                    }
                }
            }
        }
    }

    // async fn send_keep_alive(&mut self) -> io::Result<()> {
    //     let keep_alive_packet = KeepAlivePacket { id: Instant::now(). };
    //     self.sender.send()
    // }
}

async fn recv_packet(mut read: &mut BufReader<ReadHalf<TcpStream>>) -> io::Result<RawPacket> {
    let length = var_int::async_decode(&mut read).await.unwrap();
    let id = var_int::async_decode(&mut read).await.unwrap();
    let id_len = var_int::length(id);

    let mut buf = vec![0u8; length as usize - id_len];
    read.read_exact(&mut buf).await?;

    let raw = RawPacket::new_with_data(id, &buf);

    Ok(raw)
}

async fn send_raw_packet(write: &mut WriteHalf<TcpStream>, packet: RawPacket) -> io::Result<()> {
    let bytes = Vec::<u8>::try_from(packet).map_err(|_| io::ErrorKind::InvalidData)?;

    write.write_all(&bytes).await
}
