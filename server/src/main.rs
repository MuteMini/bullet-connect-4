use env_logger;
use log::{ info, warn, error };
use std::{
    net::{ SocketAddr,  TcpListener, TcpStream },
};
use anyhow::Result;

use smol::{ Async, Executor, stream::StreamExt};
use tungstenite::Message;

use bullet_common::{ ClientMessage };

async fn accept_connection( 
    raw_stream: Async<TcpStream>,
    addr: SocketAddr,
) -> Result<()> {
    info!("[{}]: Incoming connection", addr);

    // Try to make an WS stream given the async TcpStream.
    let mut ws_stream = async_tungstenite::accept_async(raw_stream).await?;
    info!("[{}]: Established websocket stream", addr);

    // Get the binary message from the ws_stream and decode it into a client message object.
    while let Some(Ok( Message::Binary(t) )) = ws_stream.next().await {
        let msg = bincode::deserialize::<ClientMessage>(&t)?;

        match msg {
            ClientMessage::CreateRoom{ player_name } => {

                info!("[{}]: Recieved a CreateRoom message.", addr);
                return Ok(())
            },
            msg => {
                error!("[{}]: Invalid message recieved. {:?}", addr, msg);
                break;
            }
        }
    }

    info!("[{}]: Connection dropped", addr);
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let ex = Executor::new();

    // Starting web socket server.
    smol::block_on( async {

        // Makes a listener on the local PC.
        // THIS SHOULD BE CHANGED LATER DEPENDING ON HOW I DECIDE TO HOST THIS
        let listener = Async::<TcpListener>::bind(([127,0,0,1], 7878) ).expect("Listener could not be made");

        // When the listener accepts a new stream with the client's address,
        while let Ok((stream, addr)) = listener.accept().await {

            ex.spawn( async move {
                if let Err(e) = accept_connection( stream, addr ).await {
                    warn!("Failed to accept a connection from {}: {}", addr, e);
                }
            })
            .detach();
        }
    });

    Ok(())
}