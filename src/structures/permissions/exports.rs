// this is every representation for role data for use with Delta

use serde::{Deserialize, Serialize};

use super::{calculator::PermissionDataConversion, definitions::Override};

// DataSetServerDefaultPermission
/// # Permission Value
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerDefaultPermissions {
    /// Default member permission value
    pub permissions: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerPermissions {
    /// Allow / deny values for the role in this server.
    permissions: Override,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelDefaultPermissions {
    Value {
        /// Permission values to set for members in a `Group`
        permissions: u64,
    },
    Field {
        /// Allow / deny values to set for members in this `TextChannel` or `VoiceChannel`
        permissions: Override,
    },
}

/// # Permission Value
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelPermissions {
    /// Allow / deny values to set for this role
    permissions: Override,
}

impl PermissionDataConversion {
    pub fn server_default_permissions(&self) -> ServerDefaultPermissions {
        ServerDefaultPermissions {
            permissions: self.group.clone(),
        }
    }

    pub fn server_permissions(&self) -> ServerPermissions {
        ServerPermissions {
            permissions: self.channel.clone(),
        }
    }

    pub fn channel_default_permissions_group(&self) -> ChannelDefaultPermissions {
        ChannelDefaultPermissions::Value {
            permissions: self.group.clone(),
        }
    }
    pub fn channel_default_permissions(&self) -> ChannelDefaultPermissions {
        ChannelDefaultPermissions::Field {
            permissions: self.channel.clone(),
        }
    }

    pub fn channel_permissions(&self) -> ChannelPermissions {
        ChannelPermissions {
            permissions: self.channel.clone(),
        }
    }
}
