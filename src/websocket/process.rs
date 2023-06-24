use std::pin::Pin;

use futures_util::{Stream, StreamExt};
use tokio_tungstenite::{connect_async, WebSocketStream};

use super::{data::WebSocketEvent, Websocket};

impl Websocket {
    pub async fn stream(input: Connection) -> Pin<Box<impl Stream<Item = WebSocketEvent>>> {
        Box::pin({
            (input).filter_map(|result| async {
                match result {
                    Ok(message) => serde_json::from_slice(&message.into_data()).ok(),
                    Err(e) => {
                        println!("{e}");
                        None
                    }
                }
            })
        })
    }

    pub async fn generate(self) -> Connection {
        let url = format!(
            "wss://{}/?version=1format={}&token={}",
            self.domain, self.format, self.token
        );

        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        ws_stream
    }

    pub async fn start(self) -> Pin<Box<impl Stream<Item = WebSocketEvent>>> {
        Websocket::stream(self.generate().await).await
    }
}

type Connection = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
