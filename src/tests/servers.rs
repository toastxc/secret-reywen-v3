const server: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
const user: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";

#[cfg(test)]
mod tests {

    use crate::{
        methods::{
            ban_create::DataBanReason, channel_create::DataChannelCreate,
            member_edit::DataMemberEdit, permissions_set_default::DataPermissionSet, *,
        },
        tests::common::tester,
    };

    use super::*;

    #[tokio::test]
    async fn test_member_edit() {
        let http = tester().await;

        let data = DataMemberEdit::default();

        if let Err(curl) = member_edit::member_edit(&http, server, user, data).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_fetch() {
        let http = tester().await;

        if let Err(curl) = member_fetch::member_fetch(&http, server, user).await {
            panic!("{:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_member_fetch_all() {
        let http = tester().await;

        if let Err(curl) = member_fetch_all::member_fetch_all(&http, server).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_mkrm() {
        let http = tester().await;

        // ban user
        let banreason = DataBanReason {
            reason: Some(String::from("bot test")),
        };
        if let Err(curl) = ban_create::ban_create(&http, server, user, banreason).await {
            panic!("ban user {:#?}", curl);
        }

        if let Err(curl) = ban_remove::ban_remove(&http, server, user).await {
            panic!("remove banned user {:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_list() {
        let http = tester().await;

        // list banned users
        if let Err(curl) = ban_list::ban_list(&http, server).await {
            panic!("list banned users {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let http = tester().await;

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = channel_create::channel_create(&http, server, create_chan).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_permission_set_default() {
        let http = tester().await;
        let data = DataPermissionSet::new(0);
        if let Err(error) =
            permissions_set_default::permissions_set_default(&http, server, data).await
        {
            panic!("{:#?}", error);
        }
    }
}
