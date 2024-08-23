use crate::network::NetworkListener;
use crate::types::player::Player;
use std::io;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct Server {
    pub max: i32,
    pub players: Vec<Player>,

    network: NetworkListener,
}

impl Server {
    pub async fn new() -> io::Result<Server> {
        let listener = TcpListener::bind("0.0.0.0:25565").await?;
        let network = NetworkListener::new(listener);
        
        todo!()
        // tokio::spawn(async {
        //     network.accept_loop();
        // });
        // 
        // Ok(Server {
        //     max: 100,
        //     players: vec![],
        //     network,
        // })
    }
}
