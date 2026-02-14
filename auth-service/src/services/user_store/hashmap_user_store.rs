use std::collections::HashMap;

use crate::domain::User;
use crate::services::user_store::{UserStoreTrait, UserStoreError};

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
}

impl UserStoreTrait for HashMapUserStore {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("email", "password", true);
        assert!(user_store.add_user(user).is_ok());
    }

    #[test]
    fn test_add_user_already_exists() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("email", "password", true);
        assert!(user_store.add_user(user.clone()).is_ok());
        assert_eq!(
            user_store.add_user(user),
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[test]
    fn test_get_user() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("email", "password", true);
        assert!(user_store.add_user(user.clone()).is_ok());

        let inserted_user = user_store.get_user("email").unwrap();
        assert_eq!(inserted_user, &user);
    }

    #[test]
    fn test_get_user_not_found() {
        let user_store = HashMapUserStore::default();
        assert_eq!(
            user_store.get_user("email"),
            Err(UserStoreError::UserNotFound)
        );
    }

    #[test]
    fn test_validate_user() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("email", "password", true);
        assert!(user_store.add_user(user.clone()).is_ok());

        assert!(user_store.validate_user("email", "password").is_ok());
    }

    #[test]
    fn test_validate_user_invalid_credentials() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("email", "password", true);
        assert!(user_store.add_user(user.clone()).is_ok());

        assert_eq!(
            user_store.validate_user("email", "wrong-password"),
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
