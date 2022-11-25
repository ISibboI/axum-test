use std::env;
use std::path::PathBuf;
use axum_sessions::{async_session, SessionLayer};
use biscuit_auth::{KeyPair, PrivateKey};
use dotenvy::dotenv;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, TerminalMode, TermLogger};
use clap::Parser;
use sqlx::Postgres;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::sqlx_session_store::SqlxSessionStore;

mod api;
mod sqlx_session_store;

#[derive(clap::Parser)]
struct Cli {
    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

fn initialise_logging(level: LevelFilter) {
    TermLogger::init(level, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();
}

async fn load_root_key_pair() -> KeyPair {
    let env_var = "PRIVATE_KEY_FILE";

    if let Some(path) = env::var_os(env_var) {
        let path = PathBuf::from(path);
        if path.is_file() {
            info!("Loading private key from {path:?}");
            let mut private_key_file = File::open(&path).await.unwrap();
            let mut private_key_bytes = Vec::new();
            private_key_file.read_to_end(&mut private_key_bytes).await.unwrap();
            let private_key = PrivateKey::from_bytes(&private_key_bytes).unwrap();
            KeyPair::from(private_key)
        } else {
            info!("Creating new private key at {path:?}");
            let key_pair = KeyPair::new();
            let mut private_key_file = File::create(&path).await.unwrap();
            private_key_file.write_all(&key_pair.private().to_bytes()).await.unwrap();
            key_pair
        }
    } else {
        panic!("Environment variable {env_var} not set");
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    initialise_logging(cli.log_level);
    dotenv().unwrap();

    let root_key_pair = load_root_key_pair().await;
    let sql_connection_pool = sqlx::Pool::<Postgres>::connect("postgres://axum_test:axum_test@localhost/axum_test").await.unwrap();

    let session_secret = b"HARDCODEDERRORBADCHANGENOTSECUREBADSTOPNOTDONOTHARDCODESECRETSSTOP";

    api::serve(sql_connection_pool, session_secret);
}
