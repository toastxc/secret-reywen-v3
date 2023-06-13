use crate::{
    api::driver::DeltaError,
    client::process::Client,
    derive_a1, derive_a2, derive_a3, derive_a4,
    methods::{self, data::*},
    structures::{
        channels::{
            channel::Channel,
            channel_invite::Invite,
            message::{BulkMessageResponse2, Message},
        },
        permissions::newcalc::PermissionData,
        users::user::User,
    },
};

// CHANNEL
impl Client {
    derive_a1!(
        (channel_delete, methods::channel::delete, &str, ()),
        (channel_fetch, methods::channel::fetch, &str, Channel),
        (
            group_create,
            methods::group::create,
            &DataCreateGroup,
            Channel
        ),
        (
            invite_create,
            methods::channel::invite::create,
            &str,
            Invite
        ),
        (
            group_member_fetch,
            methods::group::members::fetch,
            &str,
            Vec<User>
        ),
    );
    derive_a2!(
        (
            channel_edit,
            methods::channel::edit,
            &str,
            &DataEditChannel,
            Channel
        ),
        (
            group_member_add,
            methods::group::members::add,
            &str,
            &str,
            ()
        ),
        (
            group_member_remove,
            methods::group::members::remove,
            &str,
            &str,
            ()
        ),
        (message_ack, methods::message::ack, &str, &str, ()),
        (
            message_delete_bulk,
            methods::message::bulk_delete,
            &str,
            &DataBulkDelete,
            ()
        ),
        (
            message_reactions_clear,
            methods::message::clear_reactions,
            &str,
            &str,
            ()
        ),
        (message_delete, methods::message::delete, &str, &str, ()),
        (message_fetch, methods::message::fetch, &str, &str, Message),
        (
            message_query,
            methods::message::query,
            &str,
            &DataQueryMessages,
            BulkMessageResponse2
        ),
        (
            message_search,
            methods::message::search,
            &str,
            &DataMessageSearch,
            BulkMessageResponse2
        ),
        (
            message_send,
            methods::message::send,
            &str,
            &DataMessageSend,
            Message
        ),
    );
    derive_a3!(
        (
            message_edit,
            methods::message::edit,
            &str,
            &str,
            &DataEditMessage,
            Message
        ),
        (message_react, methods::message::react, &str, &str, &str, ()),
        (
            channel_permsision_set,
            methods::permissions::channel::set,
            &str,
            &str,
            &PermissionData,
            Channel
        ),
        (
            channel_permsision_set_default,
            methods::permissions::channel::set_default,
            &str,
            &PermissionData,
            bool,
            Channel
        )
    );

    derive_a4!((
        message_unreact,
        methods::message::unreact,
        &str,
        &str,
        &str,
        &DataUnreact,
        ()
    ),);
}
