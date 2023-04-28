use serde::{Deserialize, Serialize};

use super::definitions::{Override, Permission};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    pub allow: Vec<Permission>,
    pub deny: Vec<Permission>,
}

impl Permissions {
    // converts from readable to bitwise

    pub fn add_allow(&mut self, permission: Permission) -> Self {
        self.allow.push(permission);
        self.clone()
    }

    pub fn add_deny(&mut self, permission: Permission) -> Self {
        self.deny.push(permission);
        self.clone()
    }

    pub fn default() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn export(&self) -> PermissionDataConversion {
        // define channel
        let mut channel = Override::new();

        for x in self.allow.clone() {
            channel.allow += x as u64;
        }
        for x in self.deny.clone() {
            channel.deny += x as u64;
        }

        // define group

        let mut group = 0;

        for x in self.allow.clone() {
            group += x as u64;
        }

        // deny is simply dropped from scope

        PermissionDataConversion {
            readable: self.clone(),
            group,
            channel,
        }
    }
}

// todo permissions map
//
// There are 3 ways of storing permission data
// 1 Readable data, this is a vec of enums with readable names to represent the value of each perm
// 2 DataPermission / Roled, this contains two u64s representing allow and deny permissions
// 3 NRoled, this contains a single u64 and is applicable for channels that do not have roles

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionDataConversion {
    // readable data
    pub readable: Permissions,
    // data for roleless entries
    pub group: u64,
    // data for roled entries
    pub channel: Override,
}
