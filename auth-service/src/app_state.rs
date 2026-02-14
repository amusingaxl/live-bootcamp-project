use crate::services::user_store::UserStore;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStore,
}

impl AppState {
    pub fn new(user_store: UserStore) -> Self {
        Self { user_store }
    }
}
