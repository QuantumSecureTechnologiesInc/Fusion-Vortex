use crate::command::CommandHandler;
use crate::resp::{RespParser, Value};
use crate::store::Store;
use bytes::{BufMut, BytesMut};
use fusion_net::TcpListener;
use fusion_runtime_core::Runtime;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

pub struct RedisServer {
    addr: SocketAddr,
    runtime: Arc<Runtime>,
    store: Arc<Store>,
}

impl RedisServer {
    pub fn new(addr: String, runtime: Arc<Runtime>) -> Result<Self, anyhow::Error> {
        let addr = SocketAddr::from_str(&addr)?;
        Ok(Self {
            addr,
            runtime,
            store: Arc::new(Store::new()),
        })
    }

    pub async fn run(&self) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(self.addr).await?;
        info!("Fusion Redis Server listening on {}", self.addr);

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("Accepted connection from {}", addr);
                    let store = self.store.clone();

                    self.runtime.spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, store).await {
                            error!("Error handling connection from {}: {:?}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }

    async fn handle_connection(
        mut socket: fusion_net::TcpStream,
        store: Arc<Store>,
    ) -> Result<(), anyhow::Error> {
        let handler = CommandHandler::new(store);
        let mut buffer = BytesMut::with_capacity(4096);
        let mut tmp_buf = [0u8; 4096];

        loop {
            let n = socket.read(&mut tmp_buf).await?;
            if n == 0 {
                if buffer.is_empty() {
                    return Ok(());
                } else {
                    return Ok(());
                }
            }

            buffer.put_slice(&tmp_buf[..n]);

            loop {
                match RespParser::decode(&mut buffer) {
                    Ok(Some(value)) => {
                        let response = handler.handle(value);
                        let response_bytes = response.serialize();
                        socket.write(&response_bytes).await?;
                    }
                    Ok(None) => break,
                    Err(e) => {
                        let response = Value::Error(format!("ERR Protocol error: {}", e));
                        socket.write(&response.serialize()).await?;
                        return Ok(());
                    }
                }
            }
        }
    }
}
