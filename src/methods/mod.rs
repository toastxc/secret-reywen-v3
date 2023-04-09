mod servers {
    pub mod ban_create;
    pub mod ban_list;
    pub mod ban_remove;
    pub mod channel_create;
    pub mod invites_fetch;
    pub mod member_edit;
    pub mod member_fetch;
    pub mod member_fetch_all;
    pub mod member_remove;
    pub mod permissions_set;
    pub mod permissions_set_default;
    pub mod roles_create;
    pub mod roles_delete;
    pub mod roles_edit;
    pub mod server_ack;
    pub mod server_create;
    pub mod server_delete;
    pub mod server_edit;
    pub mod server_fetch;
}
pub use servers::*;

pub mod driver;

pub mod channels {
    pub mod channel_ack;
    pub mod channel_delete;
    pub mod channel_edit;
    pub mod channel_fetch;
    pub mod group_add_member;
    pub mod group_create;
    pub mod group_remove_member;
    pub mod invite_create;
    pub mod members_fetch;
    pub mod message_bulk_delete;
    pub mod message_clear_reactions;
    pub mod message_delete;
    pub mod message_edit;
    pub mod message_fetch;
    pub mod message_query;
    pub mod message_query_stale;
    pub mod message_react;
    pub mod message_search;
    pub mod message_send;
    pub mod message_unreact;
    pub mod permissions_set;
    pub mod permissions_set_default;
    pub mod voice_join;
}
