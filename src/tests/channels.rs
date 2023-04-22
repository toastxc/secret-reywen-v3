#[cfg(test)]
mod tests {
    use crate::{
        methods::channels::{
            channel_delete,
            channel_edit::{self, DataEditChannel},
            channel_fetch, group_add_member,
            group_create::{self, DataCreateGroup},
            group_remove_member, invite_create, members_fetch,
            message_ack::{self, message_ack},
            message_bulk_delete::{self, DataBulkDelete},
        },
        tests::common::{tester_bot, tester_user, CHANNEL, GROUP, SERVER},
    };

    #[tokio::test]
    async fn test_delete_channel() {
        let http = tester_bot().await;

        if let Err(error) =
            channel_delete::channel_delete(&http, "01GXDMSSJTXB14EA7J4R77B778").await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_channel_edit() {
        let http = tester_bot().await;

        let data = DataEditChannel::new()
            .set_name("benis")
            .set_description("wenis");

        if let Err(error) = channel_edit::channel_edit(&http, CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_channel_fetch() {
        let http = tester_bot().await;

        if let Err(error) = channel_fetch::channel_fetch(&http, CHANNEL).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn group_add_member() {
        let http = tester_user().await;

        if let Err(error) =
            group_add_member::group_add_member(&http, GROUP, "01FQ6SDAVSV5X1B4A7JXNB4FZV").await
        {
            panic!("{:#?}", error);
        }
    }

    // todo does not work
    #[tokio::test]
    async fn test_create_group() {
        let http = tester_user().await;

        let data = DataCreateGroup::new("womp")
            .add_user("01FQ6SDAVSV5X1B4A7JXNB4FZV")
            .set_nsfw(false);

        println!("{:#?}", data);
        if let Err(error) = group_create::group_create(&http, &data).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_group_remove_member() {
        let http = tester_user().await;

        if let Err(error) = group_remove_member::group_remove_member(
            &http,
            "01GYM0JBNKWRJYX56F9GYABS4R",
            "01FQ6SDAVSV5X1B4A7JXNB4FZV",
        )
        .await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_invite_create() {
        let http = tester_user().await;

        if let Err(error) = invite_create::invite_create(&http, CHANNEL).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_member_fetch() {
        let http = tester_bot().await;

        if let Err(error) = members_fetch::members_fetch(&http, CHANNEL).await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_ack() {
        let http = tester_user().await;

        if let Err(error) =
            message_ack::message_ack(&http, CHANNEL, "01GYMBW4XV6TF3199RVFXWWVQ7").await
        {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_bulk_delete() {
        let http = tester_bot().await;

        let data = DataBulkDelete::new().add_message("01GYMCHDB9Q1ETS4KP9NG1WW32");
        if let Err(error) = message_bulk_delete::message_bulk_delete(&http, CHANNEL, &data).await {
            panic!("{:#?}", error);
        }
    }
}

//message_ack
/*



    TEMPALTE


 #[tokio::test]
    async fn test_template() {
        let http = tester().await;

        if let Err(error) =
            template::template(&http, CHANNEL).await
        {
            panic!("{:#?}", error);
        }
    }


*/
