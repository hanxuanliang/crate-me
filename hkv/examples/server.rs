use anyhow::Result;
use async_prost::AsyncProstStream;
use hkv::{CommandReq, CommandResp};
use tokio::net::TcpListener;
use futures::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:16379";
    let listen = TcpListener::bind(addr).await?;
    info!("listen on {}", addr);

    loop {
        let (stream, addr) = listen.accept().await?;
        info!("Start connect with {}", addr);

        tokio::spawn(async move {
            let mut stream = 
                AsyncProstStream::<_, CommandReq, CommandResp, _>::from(stream).for_async();
            while let Some(Ok(data)) = stream.next().await {
                info!("Get new command from client: {:?}", data);

                let mut resp = CommandResp::default();
                resp.status = 404;
                resp.message = "NotFound".to_string();
                stream.send(resp).await.unwrap();
            }

            info!("Client {:?} disconnected", addr)
        });
    }
}