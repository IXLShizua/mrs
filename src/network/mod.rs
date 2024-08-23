use crate::network::connection::Connection;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub mod connection;
mod handlers;
mod handle_packet;

#[derive(Debug)]
pub struct NetworkListener {
    listener: TcpListener,
    connections: HashMap<SocketAddr, Connection>,
}

impl NetworkListener {
    pub fn new(listener: TcpListener) -> NetworkListener {
        NetworkListener {
            listener,
            connections: HashMap::new(),
        }
    }

    pub async fn accept_loop(&mut self) {
        
    }
}

#[derive(Debug)]
pub enum Message {
    NewConnection(Connection)
}

pub async fn accept_loop(listener: TcpListener) {
    let (tx, rx) = mpsc::channel(512);
    
    let accept_loop_handle = tokio::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            let conn = Connection::new(stream);
            
            if let Err(err) = tx.send(Message::NewConnection(conn)).await {
                println!("Error with send new connection: {}", err);
            }
        }
    });
} 