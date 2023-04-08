const server: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
const user: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";

#[cfg(test)]
mod tests {

    use crate::{
        methods::{
            ban_create::{ban_create, DataBanReason},
            ban_list::ban_list,
            ban_remove::ban_remove,
            channel_create::{channel_create, DataChannelCreate},
        },
        tests::common::tester,
    };

    use super::*;

    #[tokio::test]
    async fn test_ban_mkrm() {
        let http = tester().await;

        // ban user
        let banreason = DataBanReason {
            reason: Some(String::from("bot test")),
        };
        if let Err(curl) = ban_create(&http, server, user, banreason).await {
            panic!("ban user {:#?}", curl);
        }

        // unban user
        if let Err(curl) = ban_remove(&http, server, user).await {
            panic!("remove banned user {:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_list() {
        let http = tester().await;

        // list banned users
        if let Err(curl) = ban_list(&http, server).await {
            panic!("list banned users {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let http = tester().await;

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = channel_create(&http, server, create_chan).await {
            panic!("{:#?}", curl);
        }
    }
}
