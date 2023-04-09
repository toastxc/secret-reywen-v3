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
