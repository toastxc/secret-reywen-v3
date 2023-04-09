use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Permission {
    // Generic permissions
    ManageChannel = 1 << 0,
    ManageServer = 1 << 1,
    ManagePermissions = 1 << 2,
    ManageRole = 1 << 3,
    ManageCustomisation = 1 << 4,

    // Member permissions
    KickMembers = 1 << 6,
    BanMembers = 1 << 7,
    TimeoutMembers = 1 << 8,
    AssignRoles = 1 << 9,
    ChangeNickname = 1 << 10,
    ManageNicknames = 1 << 11,
    ChangeAvatar = 1 << 12,
    RemoveAvatars = 1 << 13,

    // Channel permissions
    ViewChannel = 1 << 20,
    ReadMessageHistory = 1 << 21,
    SendMessage = 1 << 22,
    ManageMessages = 1 << 23,
    ManageWebhooks = 1 << 24,
    InviteOthers = 1 << 25,
    SendEmbeds = 1 << 26,
    UploadFiles = 1 << 27,
    Masquerade = 1 << 28,
    React = 1 << 29,

    // Voice permissions
    Connect = 1 << 30,
    Speak = 1 << 31,
    Video = 1 << 32,
    MuteMembers = 1 << 33,
    DeafenMembers = 1 << 34,
    MoveMembers = 1 << 35,

    // Misc. permissions
    #[default]
    GrantAllSafe = 0x000f_ffff_ffff_ffff,
}
