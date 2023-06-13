pub mod data;
pub mod process;
pub mod result;

use futures_util::{
    lock::Mutex,
    stream::{SplitSink, SplitStream},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub type WSRead =
    SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>;

pub type WSWrite = Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>;
