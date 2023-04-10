use serde::{Deserialize, Serialize};

use super::definitions::Permission;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissionSet {
    pub permissions: DataPermissions,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissions {
    pub allow: u32,
    pub deny: u32,
}
impl DataPermissions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReadablePermissionData {
    pub allow: Vec<Permission>,
    pub deny: Vec<Permission>,
}
impl ReadablePermissionData {
    pub fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Permissions {
    pub readable: ReadablePermissionData,
}

impl Permissions {
    // converts from readable to bitwise
    pub fn export(&self) -> DataPermissionSet {
        let mut result = DataPermissions::new();

        for x in self.readable.allow.clone() {
            result.allow += x as u32;
        }

        for x in self.readable.deny.clone() {
            result.deny += x as u32;
        }

        DataPermissionSet {
            permissions: result,
        }
    }

    pub fn add_allow(&mut self, permission: Permission) -> Self {
        self.readable.allow.push(permission);
        self.clone()
    }

    pub fn add_deny(&mut self, permission: Permission) -> Self {
        self.readable.deny.push(permission);
        self.clone()
    }
}
