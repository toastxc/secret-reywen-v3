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
            message_bulk_delete::{self, DataBulkDelete}, message_clear_reactions, message_send::{message_send, self, DataMessageSend}, message_delete::{message_delete, self}, message_edit::{message_edit, DataEditMessage, self}, message_fetch::{message_fetch, self},
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

    #[tokio::test]
    async fn test_message_clear_reactions() {
        let http = tester_bot().await;

        if let Err(error) = message_clear_reactions::message_clear_reactions(&http, CHANNEL, "01GYMCHDB9WRYN8WEVG25FESVS").await {
            panic!("{:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_delete() {
        let http = tester_bot().await;

        let create_result_data = DataMessageSend::new().content("reywen_test");
        let create_result = message_send::message_send(&http, CHANNEL, &create_result_data).await;

        if let Err(error) = create_result {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let cr_data = create_result.ok().unwrap();
        if let Err(error) = message_delete::message_delete(&http, CHANNEL, cr_data._id.as_str()).await {
            panic!("delete message failed {:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_edit() {
        let http = tester_bot().await;

        let original_message_data = DataMessageSend::new().content("original content");
        let original_message = message_send::message_send(&http, CHANNEL, &original_message_data).await;

        if let Err(error) = original_message {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let original_message_success = original_message.ok().unwrap();
        let edit_message_data = DataEditMessage::new().content("edited content");
        if let Err(error) = message_edit::message_edit(&http, CHANNEL, original_message_success._id.as_str(), &edit_message_data).await {
            panic!("edit message failed {:#?}", error);
        }
    }

    #[tokio::test]
    async fn test_message_fetch() {
        let http = tester_bot().await;

        let original_message_data = DataMessageSend::new().content("fetch test");
        let original_message = message_send::message_send(&http, CHANNEL, &original_message_data).await;

        if let Err(error) = original_message {
            panic!("create message failed (required for test) {:#?}", error);
        }

        let og_succ = original_message.ok().unwrap();
        if let Err(error) = message_fetch::message_fetch(&http, CHANNEL, og_succ._id.as_str()).await {
            panic!("fetch message failed {:#?}", error);
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
