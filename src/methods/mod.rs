mod bots;
mod channels;
mod corelib;
pub mod driver;
mod servers;
mod users;

pub mod channel {
    pub use crate::methods::channels::{
        channel_delete::channel_delete as delete, channel_edit::channel_edit as edit,
        channel_fetch::channel_fetch as fetch,
    };
    pub mod invite {
        pub use crate::methods::channels::invite_create::invite_create as create;
    }
}
pub mod group {
    pub use crate::methods::channels::group_create::group_create as create;
    pub mod members {
        pub use crate::methods::channels::{
            group_add_member::group_add_member as add,
            group_remove_member::group_remove_member as remove,
            members_fetch::members_fetch as fetch,
        };
    }
}
pub mod message {
    pub use crate::methods::channels::{
        message_ack::message_ack as ack, message_bulk_delete::message_bulk_delete as bulk_delete,
        message_clear_reactions::message_clear_reactions as clear_reactions,
        message_delete::message_delete as delete, message_edit::message_edit as edit,
        message_fetch::message_fetch as fetch, message_query::message_query as query,
        message_react::message_react as react, message_search::message_search as search,
        message_send::message_send as send, message_unreact::message_unreact as unreact,
    };
    pub mod reaction {
        pub use crate::methods::channels::message_clear_reactions::message_clear_reactions as clear;
        pub use crate::methods::channels::message_react::message_react as react;
        pub use crate::methods::channels::message_unreact::message_unreact as unreact;
    }
}
pub mod permissions {
    pub mod channel {
        pub use crate::methods::channels::{
            permissions_set::permissions_set as set,
            permissions_set_default::permissions_set_default as set_default,
        };
    }
    pub mod server {
        pub use crate::methods::servers::{
            permissions_set::permissions_set as set,
            permissions_set_default::permissions_set_default as set_default,
        };
    }
}

pub mod bot {
    pub use crate::methods::bots::{
        bot_create::bot_create as create, bot_delete::bot_delete as delete,
        bot_edit::bot_edit as edit, bot_invite::bot_invite as invite,
    };
    pub mod fetch {
        pub use crate::methods::bots::{
            bot_fetch::bot_fetch as standard, bot_fetch_owned::bot_fetch_owned as owned,
            bot_fetch_public::bot_fetch_public as public,
        };
    }
}

pub mod server {
    pub mod ban {
        pub use crate::methods::servers::{
            ban_create::ban_create as create, ban_list::ban_list as list,
            ban_remove::ban_remove as remove,
        };
    }
    pub use crate::methods::servers::{
        server_ack::server_ack as ack, server_create::server_create as create,
        server_delete::server_delete as delete, server_edit::server_edit as edit,
        server_fetch::server_fetch as fetch,
    };
    pub mod channel {
        pub use crate::methods::servers::channel_create::channel_create as create;
    }

    pub mod invites {
        pub use crate::methods::servers::invites_fetch::invites_fetch as fetch;
    }

    pub mod role {
        pub use crate::methods::servers::{
            roles_create::roles_create as create, roles_delete::roles_delete as delete,
            roles_edit::roles_edit as edit,
        };
    }
    pub mod member {
        pub mod fetch {
            pub use crate::methods::servers::{
                member_fetch::member_fetch as one, member_fetch_all::member_fetch_all as all,
            };
        }
        pub use crate::methods::servers::{
            member_edit::member_edit as edit, member_remove::member_remove as remove,
        };
    }
}

pub mod user {

    pub mod incomplete {}
}

pub mod util {
    pub use crate::methods::corelib::*;
}

pub mod data {

    // channel

    pub use crate::methods::{
        bots::{bot_create::DataCreateBot, bot_edit::DataEditBot},
        channels::{
            channel_edit::DataEditChannel, group_create::DataCreateGroup,
            message_bulk_delete::DataBulkDelete, message_edit::DataEditMessage,
            message_query::DataQueryMessages, message_search::DataMessageSearch,
            message_send::DataMessageSend, message_unreact::DataUnreact,
        },
        servers::{
            ban_create::DataBanReason, ban_list::DataBanList, channel_create::DataChannelCreate,
            member_edit::DataMemberEdit, member_remove::AllMemberResponse,
            roles_create::DataRoleCreate, roles_edit::DataEditRole,
            server_create::DataCreateServer, server_edit::DataEditServer,
        },
    };
}

pub mod response {
    pub use crate::methods::{
        bots::{bot_fetch::BotResponse, bot_fetch_owned::OwnedBotsResponse},
        servers::{member_fetch_all::ResponseMemberAll, server_create::CreateServerResponse},
    };
}
