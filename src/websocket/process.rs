//reywen
use crate::structures::channels::message;
// internal websocket lib
use super::{
    data::{data_is, Websocket},
    result::{error::WSError, traits::ErrorConvert},
    WSRead, WSWrite,
};
// websocket
use tokio_tungstenite::{connect_async, tungstenite::Message};
// async
use futures_util::{lock::Mutex, SinkExt, StreamExt};

impl Websocket {
    pub async fn generate(&self) -> Result<WSRead, WSError> {
        let url = format!(
            "wss://{}/?version=1format={}&token={}",
            self.domain, self.format, self.token
        );

        let ws = connect_async(url).await.res()?;
        let (stream, _response) = ws;

        let (write, read) = stream.split();

        tokio::spawn(Websocket::ping_server(Mutex::new(write)));

        Ok(read)
    }

    async fn ping_server(write: WSWrite) {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

            write
                .lock()
                .await
                .send(Message::Text(String::from(
                    "{\n    \"type\": \"Ping\",\n    \"data\": 0\n}",
                )))
                .await
                .ok();
        }
    }

    pub async fn ws_handler(ws: Websocket) -> Result<(), WSError> {
        loop {
            ws.generate()
                .await?
                .for_each(|data| async {
                    if let Err(error) = tokio::spawn(Websocket::message_handler(data.res())).await {
                        println!("WARNING: {:#?}", error);
                        ws.to_owned().increment_watchdog();
                    };
                })
                .await;
            ws.watchdog_compliance()?;
        }
    }

    pub async fn message_handler(data: Result<Message, WSError>) -> Result<(), WSError> {
        if let Err(error) = data {
            return Err(error);
        } else if let Some(message) = data_is::<message::Message>(&data) {
            println!("{:#?}", message.content)
        };
        Ok(())
    }
}
