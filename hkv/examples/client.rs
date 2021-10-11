use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use hkv::{CommandReq, CommandResp};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:16379";
    info!("listen on {}", addr);
    let stream = TcpStream::connect(addr).await?;
    let mut client =
        AsyncProstStream::<_, CommandResp, CommandReq, _>::from(stream).for_async();

    let cmd = CommandReq::new_hset("table1", "hello", "world".to_string().into());
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Get response from HKV: {:?}", data);
    }

    Ok(())
}