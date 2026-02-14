use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::User;

mod hashmap_user_store;
pub use hashmap_user_store::HashMapUserStore;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub trait UserStoreTrait: Send + Sync {
    fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    fn get_user(&self, email: &str) -> Result<&User, UserStoreError>;
    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError>;
}

pub type UserStore = Arc<RwLock<Box<dyn UserStoreTrait + Send + Sync>>>;

pub fn make_user_store(store: impl UserStoreTrait + 'static) -> UserStore {
    Arc::new(RwLock::new(Box::new(store)))
}
