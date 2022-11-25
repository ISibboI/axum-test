use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use axum_sessions::async_session::{Session, SessionStore};
use sqlx::{Database, Pool};

#[derive(Debug)]
pub struct SqlxSessionStore<DB: Database> {
    sqlx_connection_pool: Pool<DB>,
}

impl<DB: Database + Debug> SessionStore for SqlxSessionStore<DB> {
    fn load_session<'this, 'async_trait>(
        &'this self,
        cookie_value: String
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result<Option<Session>>> + Send + 'async_trait>>
        where
            'this: 'async_trait,
            Self: 'async_trait {
        todo!()
    }
    fn store_session<'this, 'async_trait>(
        &'this self,
        session: Session
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result<Option<String>>> + Send + 'async_trait>>
        where
            'this: 'async_trait,
            Self: 'async_trait {
        todo!()
    }
    fn destroy_session<'this, 'async_trait>(
        &'this self,
        session: Session
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result> + Send + 'async_trait>>
        where
            'this: 'async_trait,
            Self: 'async_trait {
        todo!()
    }
    fn clear_store<'this, 'async_trait>(
        &'this self
    ) -> Pin<Box<dyn Future<Output = axum_sessions::async_session::Result> + Send + 'async_trait>>
        where
            'this: 'async_trait,
            Self: 'async_trait {
        todo!()
    }
}

impl<DB: Database> SqlxSessionStore<DB> {
    pub fn new(sqlx_connection_pool: Pool<DB>) -> Self {
        Self {sqlx_connection_pool}
    }
}

impl<DB: Database> Clone for SqlxSessionStore<DB> {
    fn clone(&self) -> Self {
        Self {
            sqlx_connection_pool: self.sqlx_connection_pool.clone(),
        }
    }
}