use crate::derive_a0;
use crate::methods::driver::DeltaError;
use crate::methods::response::{BotResponse, OwnedBotsResponse};
use crate::structures::users::bot::PublicBot;
use crate::{
    client::process::Client,
    derive_a1, derive_a2,
    methods::{self, data::*},
    structures::users::bot::Bot,
};
impl Client {
    derive_a0!((
        bot_fetch_owned,
        methods::bot::fetch::owned,
        OwnedBotsResponse
    ));

    derive_a1!(
        (bot_create, methods::bot::create, &DataCreateBot, Bot),
        (bot_delete, methods::bot::delete, &str, ()),
        (bot_fetch, methods::bot::fetch::standard, &str, BotResponse),
        (
            bot_fetch_public,
            methods::bot::fetch::public,
            &str,
            PublicBot
        ),
    );
    derive_a2!(
        (bot_edit, methods::bot::edit, &str, &DataEditBot, Bot),
        (bot_invite, methods::bot::invite, &str, &str, ()),
    );
}
