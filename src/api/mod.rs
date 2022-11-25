use std::net::SocketAddr;
use axum::{Router};
use axum::routing::post;
use axum_sessions::SessionLayer;
use sqlx::{Database, Pool};
use crate::api::login::login_user;
use crate::sqlx_session_store::SqlxSessionStore;

mod login;

pub async fn serve<DB: Database>(sql_connection_pool: Pool<DB>, session_secret: &[u8]) {
    let session_store = SqlxSessionStore::new(sql_connection_pool.clone());
    let session_layer = SessionLayer::new(session_store, session_secret);

    let app = Router::new().route("/login", post(login_user)).layer(session_layer);
    let served_address = SocketAddr::from(([0,0,0,0], 3018));
    axum::Server::bind(&served_address).serve(app.into_make_service()).await.unwrap();
}