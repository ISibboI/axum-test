use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

pub async fn login_user(Json(payload): Json<LoginUser>) {

}