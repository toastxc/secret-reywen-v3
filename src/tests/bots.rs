#[cfg(test)]
mod tests {

    use crate::{
        methods::bots::{
            bot_create::{self, DataCreateBot},
            bot_delete,
            bot_edit::{self, DataEditBot},
            bot_fetch, bot_fetch_owned, bot_fetch_public, bot_invite,
        },
        tests::common::{tester_user, BOT, CHANNEL, GROUP, SERVER},
    };

    #[tokio::test]
    async fn test_create_bot() {
        let http = tester_user().await;

        let data = DataCreateBot::new("womp");

        if let Err(error) = bot_create::bot_create(&http, data).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_delete_bot() {
        let http = tester_user().await;

        let data = DataCreateBot::new("womp");
        match bot_create::bot_create(&http, data).await {
            Ok(bot) => {
                if let Err(error) = bot_delete::bot_delete(&http, &bot.id).await {
                    panic!("{:#?}", error);
                };
            }
            Err(e) => panic!("failed pretest (creating bot)\n{:#?}", e),
        }
    }

    #[tokio::test]
    async fn test_edit_bot() {
        let http = tester_user().await;

        let data_bot_create = DataCreateBot::new("wompywompy");
        let data_bot_edit = DataEditBot::new().set_name("cowdoyinthecity2");
        match bot_create::bot_create(&http, data_bot_create).await {
            Ok(bot) => {
                if let Err(error) = bot_edit::bot_edit(&http, &bot.id, data_bot_edit).await {
                    panic!("{:#?}", error);
                };
            }
            Err(e) => panic!("failed pretest (creating bot)\n{:#?}", e),
        }
    }

    #[tokio::test]
    async fn test_bot_fetch() {
        let http = tester_user().await;

        if let Err(error) = bot_fetch::bot_fetch(&http, BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_owned() {
        let http = tester_user().await;

        if let Err(error) = bot_fetch_owned::bot_fetch_owned(&http).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_public() {
        let http = tester_user().await;

        if let Err(error) = bot_fetch_public::bot_fetch_public(&http, BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_invite() {
        let http = tester_user().await;

        if let Err(error) = bot_invite::bot_invite(&http, BOT, SERVER).await {
            panic!("{:#?}", error);
        };
    }
}

// TEMPLATE DO NOT REMOVE

/*



    #[tokio::test]
    async fn test_example() {
        let http = tester_user().await;

        let data = DataExample::new("womp");

        if let Err(error) = example::example(&http, data).await {
            panic!("{:#?}", error);
        };
    }
*/
