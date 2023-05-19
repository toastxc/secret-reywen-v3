#[cfg(test)]
mod tests {

    use crate::{
        methods::{
            data::*,
            permissions,
            server::{self, member},
        },
        structures::permissions::{calculator::Permissions, definitions::Permission},
        tests::common::{tester_bot, ROLE, SERVER, USER},
    };

    #[tokio::test]
    async fn test_member_edit() {
        let http = tester_bot().await;

        let data = DataMemberEdit::default();

        if let Err(curl) = member::edit(&http, SERVER, USER, data).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_remove() {
        let http = tester_bot().await;

        if let Err(curl) = member::remove(&http, SERVER, USER).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_member_fetch() {
        let http = tester_bot().await;

        if let Err(curl) = member::fetch::one(&http, SERVER, USER).await {
            panic!("{:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_member_fetch_all() {
        let http = tester_bot().await;

        if let Err(curl) = member::fetch::all(&http, SERVER).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_mkrm() {
        let http = tester_bot().await;

        // ban user
        let banreason = DataBanReason {
            reason: Some(String::from("bot test")),
        };
        if let Err(curl) = server::ban::create(&http, SERVER, USER, banreason).await {
            panic!("ban user {:#?}", curl);
        }

        if let Err(curl) = server::ban::remove(&http, SERVER, USER).await {
            panic!("remove banned user {:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_ban_list() {
        let http = tester_bot().await;

        if let Err(curl) = server::ban::list(&http, SERVER).await {
            panic!("list banned users {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let http = tester_bot().await;

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = server::channel::create(&http, SERVER, create_chan).await {
            panic!("{:#?}", curl);
        }
    }

    #[tokio::test]
    async fn test_permission_set_default() {
        let http = tester_bot().await;
        let data = Permissions::default()
            .add_allow(Permission::ViewChannel)
            .add_allow(Permission::KickMembers)
            .export();

        if let Err(error) = permissions::server::set_default(&http, SERVER, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_fetch_invites() {
        let http = tester_bot().await;

        if let Err(error) = server::invites::fetch(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }
    #[tokio::test]
    async fn test_permission_set() {
        let http = tester_bot().await;

        let perms = Permissions::default()
            .add_allow(Permission::ViewChannel)
            .add_allow(Permission::KickMembers)
            .export();

        if let Err(error) = permissions::server::set(&http, SERVER, ROLE, perms).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_create() {
        let http = tester_bot().await;

        let data = DataRoleCreate::new("dummyrole");

        if let Err(error) = server::role::create(&http, SERVER, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_role_delete() {
        let http = tester_bot().await;

        if let Err(error) = server::role::delete(&http, SERVER, "01GXG24BV8QMSFWXKFRHZV30AY").await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_roles_edit() {
        let http = tester_bot().await;

        let data = DataEditRole::new();

        if let Err(error) = server::role::edit(&http, SERVER, ROLE, data).await {
            panic!("{:#?}", error);
        }
    }

    // todo unresolved issue
    #[tokio::test]
    async fn test_server_create() {
        let http = tester_bot().await;

        let data = DataCreateServer::new("DummyServer");

        if let Err(error) = server::create(&http, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_edit() {
        let http = tester_bot().await;
        let data = DataEditServer::new();

        if let Err(error) = server::edit(&http, SERVER, data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_fetch() {
        let http = tester_bot().await;

        if let Err(error) = server::fetch(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }

    // todo untested
    #[tokio::test]
    async fn test_server_ack() {
        let http = tester_bot().await;

        if let Err(error) = server::ack(&http, SERVER).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_server_delete() {
        let http = tester_bot().await;

        if let Err(error) = server::delete(&http, SERVER).await {
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
            template(&http, SERVER).await
        {
            panic!("{:#?}", error);
        }
    }





*/
