pub const SERVER: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
pub const USER: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
#[cfg(test)]
mod tests {

    use crate::{
        methods::{
            ban_create::DataBanReason, channel_create::DataChannelCreate,
            member_edit::DataMemberEdit, permissions_set::DataPermissionSet,
            permissions_set_default::DataPermissionSetDefault, roles_create::DataRoleCreate,
            roles_edit::DataEditRole, server_create::DataCreateServer, server_edit::DataEditServer,
            *,
        },
        tests::common::tester,
    };

    use super::*;

    #[tokio::test]
    async fn test_member_edit() {
        let http = tester().await;

        let data = DataMemberEdit::default();

        if let Err(curl) = member_edit::member_edit(&http, SERVER, USER, data).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_remove() {
        let http = tester().await;

        if let Err(curl) = member_remove::member_remove(&http, SERVER, USER).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_fetch() {
        let http = tester().await;

        if let Err(curl) = member_fetch::member_fetch(&http, SERVER, USER).await {
            panic!("{:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_member_fetch_all() {
        let http = tester().await;

        if let Err(curl) = member_fetch_all::member_fetch_all(&http, SERVER).await {
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
        if let Err(curl) = ban_create::ban_create(&http, SERVER, USER, banreason).await {
            panic!("ban user {:#?}", curl);
        }

        if let Err(curl) = ban_remove::ban_remove(&http, SERVER, USER).await {
            panic!("remove banned user {:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_list() {
        let http = tester().await;

        if let Err(curl) = ban_list::ban_list(&http, SERVER).await {
            panic!("list banned users {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let http = tester().await;

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = channel_create::channel_create(&http, SERVER, create_chan).await {
            panic!("{:#?}", curl);
        }
    }

    // todo this does not work
    #[tokio::test]
    async fn test_permission_set_default() {
        let http = tester().await;
        let data = DataPermissionSetDefault::new(0);
        if let Err(error) =
            permissions_set_default::permissions_set_default(&http, SERVER, data).await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_fetch_invites() {
        let http = tester().await;

        if let Err(error) = invites_fetch::invites_fetch(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }

    // todo this doesnt work
    #[tokio::test]
    async fn test_permission_set() {
        let http = tester().await;

        let data = DataPermissionSet::default();

        if let Err(error) = permissions_set::permissions_set(&http, SERVER, ROLE, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_create() {
        let http = tester().await;

        let data = DataRoleCreate::new("dummyrole");

        if let Err(error) = roles_create::roles_create(&http, SERVER, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_delete() {
        let http = tester().await;

        if let Err(error) =
            roles_delete::roles_delete(&http, SERVER, "01GXG24BV8QMSFWXKFRHZV30AY").await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_roles_edit() {
        let http = tester().await;

        let data = DataEditRole::new();

        if let Err(error) = roles_edit::roles_edit(&http, SERVER, ROLE, data).await {
            panic!("{:#?}", error);
        }
    }

    // todo unresolved issue
    #[tokio::test]
    async fn test_server_create() {
        let http = tester().await;

        let data = DataCreateServer::new("DummyServer");

        if let Err(error) = server_create::server_create(&http, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_edit() {
        let http = tester().await;
        let data = DataEditServer::new();

        if let Err(error) = server_edit::server_edit(&http, SERVER, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_fetch() {
        let http = tester().await;

        if let Err(error) = server_fetch::server_fetch(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }

    // todo untested
    #[tokio::test]
    async fn test_server_ack() {
        let http = tester().await;

        if let Err(error) = server_ack::server_ack(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_delete() {
        let http = tester().await;

        if let Err(error) = server_delete::server_delete(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }
}

// DO NOT REMOVE - TEMPLATE
//
//
//
//
//
//
//
// TEMPLATE
/*




 #[tokio::test]
    async fn test_template() {
        let http = tester().await;

        if let Err(error) =
            template::template(&http, SERVER).await
        {
            panic!("{:#?}", error);
        }
    }





*/
