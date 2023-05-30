#[cfg(test)]
mod tests {
    use crate::websocket::data::Websocket;

    #[tokio::test]
    async fn websocket() {
        let ws = Websocket::from_token("");

        match Websocket::ws_handler(ws).await {
            Ok(_) => {
                println!("()")
            }
            Err(error) => {
                println!("{:#?}", error)
            }
        }
    }
}
