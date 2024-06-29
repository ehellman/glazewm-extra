use anyhow::Result;
use fastwebsockets::*;
use http_body_util::Empty;
use hyper::{
    body::Bytes,
    header::{CONNECTION, UPGRADE},
    upgrade::Upgraded,
    Request,
};
use hyper_util::rt::tokio::TokioIo;

use super::spawn_executor::SpawnExecutor;

use tokio::net::TcpStream;

pub struct Stream {
    collector: FragmentCollector<TokioIo<Upgraded>>,
}

impl Stream {
    pub async fn new() -> Result<Self> {
        let stream = TcpStream::connect("localhost:6123").await?;
        let req = Request::builder()
            .method("GET")
            .uri("http://localhost:6123/")
            .header("Host", "localhost:6123")
            .header(UPGRADE, "websocket")
            .header(CONNECTION, "upgrade")
            .header(
                "Sec-WebSocket-Key",
                fastwebsockets::handshake::generate_key(),
            )
            .header("Sec-WebSocket-Version", "13")
            .body(Empty::<Bytes>::new())?;

        let (ws, _) = handshake::client(&SpawnExecutor, req, stream).await?;

        Ok(Self {
            collector: FragmentCollector::new(ws),
        })
    }

    pub async fn write(&mut self, content: &str) -> Result<()> {
        self.collector
            .write_frame(Frame::text(Payload::Borrowed(content.as_bytes())))
            .await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<String> {
        let frame = self.collector.read_frame().await?;
        let payload = String::from_utf8(frame.payload.to_vec()).expect("Invalid UTF-8 data");
        Ok(payload)
    }
}