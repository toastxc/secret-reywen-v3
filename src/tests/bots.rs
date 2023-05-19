#[cfg(test)]
mod tests {

    use crate::{
        methods::{bot, data::*},
        tests::common::{tester_user, BOT, SERVER},
    };

    #[tokio::test]
    async fn test_create_bot() {
        let http = tester_user().await;

        let data = DataCreateBot::new("womp");

        if let Err(error) = bot::create(&http, data).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_delete_bot() {
        let http = tester_user().await;

        let data = DataCreateBot::new("womp");
        match bot::create(&http, data).await {
            Ok(bot) => {
                if let Err(error) = bot::delete(&http, &bot.id).await {
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
        match bot::create(&http, data_bot_create).await {
            Ok(bot) => {
                if let Err(error) = bot::edit(&http, &bot.id, data_bot_edit).await {
                    panic!("{:#?}", error);
                };
            }
            Err(e) => panic!("failed pretest (creating bot)\n{:#?}", e),
        }
    }

    #[tokio::test]
    async fn test_bot_fetch() {
        let http = tester_user().await;

        if let Err(error) = bot::fetch::standard(&http, BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_owned() {
        let http = tester_user().await;

        if let Err(error) = bot::fetch::owned(&http).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_fetch_public() {
        let http = tester_user().await;

        if let Err(error) = bot::fetch::public(&http, BOT).await {
            panic!("{:#?}", error);
        };
    }

    #[tokio::test]
    async fn test_bot_invite() {
        let http = tester_user().await;

        if let Err(error) = bot::invite(&http, BOT, SERVER).await {
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
