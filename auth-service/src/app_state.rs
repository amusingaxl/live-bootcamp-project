use std::sync::Arc;

use crate::domain::UserStore;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<Box<dyn UserStore>>,
}

impl AppState {
    pub fn new<T>(store: T) -> Self
    where
        T: UserStore + 'static,
    {
        Self {
            user_store: Arc::new(Box::new(store)),
        }
    }
}
