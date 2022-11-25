use crate::api::login::login_user;
use crate::sql_session_store::SqlSessionStore;
use axum::routing::post;
use axum::Router;
use axum_sessions::SessionLayer;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;

mod login;

pub async fn serve(db_conn: &DatabaseConnection, session_secret: &[u8]) {
    let session_store = SqlSessionStore::new(db_conn.clone());
    let session_layer = SessionLayer::new(session_store, session_secret);

    let app = Router::new()
        .route("/login", post(login_user))
        .layer(session_layer)
        .with_state(db_conn.clone());
    let served_address = SocketAddr::from(([0, 0, 0, 0], 3018));
    axum::Server::bind(&served_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
