//json
use serde::{Deserialize, Serialize};
// internal websocket lib
use super::result::error::{WSError, WatchdogError};
// websocket
use tokio_tungstenite::tungstenite::Message;
// async

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Websocket {
    pub token: String,
    pub format: String,
    #[serde(rename = "websocket_domain")]
    pub domain: String,
    watchdog: u32,
    watchdog_limit: u32,
}

impl Websocket {
    pub fn from_token(token: &str) -> Self {
        Websocket {
            token: String::from(token),
            format: String::from("json"),
            domain: String::from("ws.revolt.chat"),
            watchdog_limit: 1,
            ..Default::default()
        }
    }

    pub fn increment_watchdog(&mut self) -> Self {
        self.watchdog += 1;
        self.to_owned()
    }

    pub fn watchdog_compliance(&self) -> Result<(), WSError> {
        if self.watchdog >= self.watchdog_limit {
            Err(WSError::Watchdog(WatchdogError::ExceededComplianceLimit))
        } else {
            Ok(())
        }
    }
    pub fn data_is<T: serde::de::DeserializeOwned>(input: &Result<Message, WSError>) -> Option<T> {
        match serde_json::from_slice::<T>(
            &(match input {
                Ok(data) => data.to_owned().into_data(),
                Err(_) => return None,
            }),
    ) {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }
    
}
