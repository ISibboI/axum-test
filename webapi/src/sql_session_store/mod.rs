use axum_sessions::async_session::{Session, SessionStore};
use sea_orm::DatabaseConnection;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct SqlSessionStore {
    db_conn: DatabaseConnection,
}

impl SessionStore for SqlSessionStore {
    fn load_session<'this, 'async_trait>(
        &'this self,
        _cookie_value: String,
    ) -> Pin<
        Box<
            dyn Future<Output = axum_sessions::async_session::Result<Option<Session>>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'this: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
    fn store_session<'this, 'async_trait>(
        &'this self,
        _session: Session,
    ) -> Pin<
        Box<
            dyn Future<Output = axum_sessions::async_session::Result<Option<String>>>
                + Send
                + 'async_trait,
        >,
    >
    where
        'this: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
    fn destroy_session<'this, 'async_trait>(
        &'this self,
        _session: Session,
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result> + Send + 'async_trait>>
    where
        'this: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
    fn clear_store<'this, 'async_trait>(
        &'this self,
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result> + Send + 'async_trait>>
    where
        'this: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}

impl SqlSessionStore {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        Self { db_conn }
    }
}
