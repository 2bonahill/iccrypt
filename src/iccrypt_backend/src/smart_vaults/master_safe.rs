use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};

use crate::common::user::{User, UserID};

use super::{
    secret::{Secret, SecretID},
    user_safe::UserSafe,
};

// use anyhow::Ok;
// use anyhow::Result;

pub type UserSafes = BTreeMap<UserID, UserSafe>;

#[derive(CandidType, Deserialize, Clone)]
pub struct MasterSafe {
    date_created: Option<u64>,
    date_modified: Option<u64>,
    pub user_safes: UserSafes,
}

impl Default for MasterSafe {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterSafe {
    pub fn new() -> Self {
        Self {
            date_created: None,
            date_modified: None,
            user_safes: BTreeMap::new(),
        }
    }

    /// Returns a mutable reference to the user safe
    pub fn get_user_safe(&mut self, user: UserID) -> Option<&mut UserSafe> {
        if let Some(us) = self.user_safes.get_mut(&user) {
            Some(us)
        } else {
            None
        }
    }

    pub fn open_new_user_safe(&mut self, user_id: UserID) -> &mut UserSafe {
        // create the user
        let new_user = User::new(user_id);
        let new_user_safe = UserSafe::new(new_user.get_id());
        self.user_safes.insert(user_id, new_user_safe);
        self.get_user_safe(user_id).unwrap()
    }

    pub fn delete_user_safe(&mut self, user: UserID) {
        self.user_safes.remove(&user);
    }

    /// Inserts a secret into a user's safe.
    // TODO: propagate error if user_safe does not exist
    pub fn add_user_secret(&mut self, user_id: UserID, secret: Secret) {
        if let Some(user_safe) = self.get_user_safe(user_id) {
            // the user already has a safe
            user_safe.add_secret(secret);
        }
    }

    pub fn update_user_secret(&mut self, user_id: UserID, secret: Secret) {
        if let Some(user_safe) = self.get_user_safe(user_id) {
            user_safe.update_secret(secret);
        }
    }

    pub fn remove_user_secret(&mut self, user_id: UserID, _secret_id: SecretID) {
        if let Some(_user_safe) = self.get_user_safe(user_id) {
            // the user has a safe
            // user_safe.add_secret(secret);
            todo!();
        }
    }
}
