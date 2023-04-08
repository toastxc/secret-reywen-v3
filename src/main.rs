use reywen_revolt::methods::driver::Delta;

// enter values here for testing
async fn tester() -> Delta {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Delta::new(
        // url
        "https://api.revolt.chat/",
        // token
        "TOKEN",
        // timeout in seconds
        10,
        // is bot account
        true,
    )
}

#[tokio::main]
async fn main() {

    // successful
    //println!("{:#?}",fetch_self(http).await);

    // todo
    // anything that cant be done by a bot will not be tested

    // start of test

    // bans
}

#[cfg(test)]
mod tests {
    use reywen_revolt::methods::{ban_create::DataBanReason, channel_create::DataChannelCreate};

    use super::*;

    #[tokio::test]
    async fn ban() {
        let http = tester().await;

        // ban user
        let banreason = DataBanReason {
            reason: Some(String::from("bot test")),
        };
        if let Err(curl) = reywen_revolt::methods::servers::ban_create::ban_create(
            &http,
            "01GKWVWGHN242DVWG4BKXG2C7F",
            "01F1WKM5TK2V6KCZWR6DGBJDTZ",
            banreason,
        )
        .await
        {
            panic!("ban user {:#?}", curl);
        }

        // list banned users
        if let Err(curl) =
            reywen_revolt::methods::servers::ban_list::ban_list(&http, "01GKWVWGHN242DVWG4BKXG2C7F")
                .await
        {
            panic!("list banned users {:#?}", curl);
        }

        // unban user
        if let Err(curl) = reywen_revolt::methods::servers::ban_remove::ban_remove(
            &http,
            "01GKWVWGHN242DVWG4BKXG2C7F",
            "01F1WKM5TK2V6KCZWR6DGBJDTZ",
        )
        .await
        {
            panic!("remove banned user {:#?}", curl);
        }
    }
    #[tokio::test]
    async fn test_channel_create() {
        let http = tester().await;

        let create_chan = DataChannelCreate::new("womp");

        if let Err(curl) = reywen_revolt::methods::servers::channel_create::channel_create(
            &http,
            "01GKWVWGHN242DVWG4BKXG2C7F",
            create_chan,
        )
        .await
        {
            panic!("{:#?}", curl);
        }
    }
}
