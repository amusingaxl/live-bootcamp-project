use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::{Email, Password, User};

#[derive(Default)]
pub struct HashMapUserStore {
    users: RwLock<HashMap<Email, User>>,
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&self, user: User) -> Result<(), UserStoreError> {
        if self.users.read().await.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.write().await.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        self.users
            .read()
            .await
            .get(&email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password != password {
            return Err(UserStoreError::InvalidCredentials);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let user_store = HashMapUserStore::default();
        let user = User::new(
            Email::parse("email@example").unwrap(),
            Password::parse("password").unwrap(),
            true,
        );
        assert!(user_store.add_user(user).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_user_already_exists() {
        let user_store = HashMapUserStore::default();
        let user = User::new(
            Email::parse("email@example").unwrap(),
            Password::parse("password").unwrap(),
            true,
        );
        assert!(user_store.add_user(user.clone()).await.is_ok());
        assert_eq!(
            user_store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let user_store = HashMapUserStore::default();
        let email = Email::parse("email@example").unwrap();
        let password = Password::parse("password").unwrap();
        let user = User::new(email.clone(), password.clone(), true);
        assert!(user_store.add_user(user.clone()).await.is_ok());

        let inserted_user = user_store.get_user(email).await.unwrap();
        assert_eq!(inserted_user, user);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let user_store = HashMapUserStore::default();
        let email = Email::parse("email@example").unwrap();
        assert_eq!(
            user_store.get_user(email).await,
            Err(UserStoreError::UserNotFound)
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user_store = HashMapUserStore::default();
        let email = Email::parse("email@example").unwrap();
        let password = Password::parse("password").unwrap();
        let user = User::new(email.clone(), password.clone(), true);
        assert!(user_store.add_user(user.clone()).await.is_ok());

        assert!(user_store.validate_user(email, password).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_invalid_credentials() {
        let user_store = HashMapUserStore::default();
        let email = Email::parse("email@example").unwrap();
        let password = Password::parse("password").unwrap();
        let user = User::new(email.clone(), password, true);
        assert!(user_store.add_user(user.clone()).await.is_ok());

        assert_eq!(
            user_store
                .validate_user(email, Password::parse("wrong-password").unwrap())
                .await,
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
