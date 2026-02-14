use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::UserStore;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<Box<dyn UserStore>>>,
}

impl AppState {
    pub fn new<T>(store: T) -> Self
    where
        T: UserStore + 'static,
    {
        Self {
            user_store: Arc::new(RwLock::new(Box::new(store))),
        }
    }
}
