use biscuit_auth::{KeyPair, PrivateKey};
use clap::Parser;
use dotenvy::dotenv;
use log::{info, LevelFilter};
use migration::{Migrator, MigratorTrait};
use rand::RngCore;
use sea_orm::DatabaseConnection;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::env;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod api;
mod sql_session_store;

#[derive(clap::Parser)]
struct Cli {
    #[clap(long, default_value = "Info")]
    log_level: LevelFilter,
}

fn initialise_logging(level: LevelFilter) {
    TermLogger::init(
        level,
        Default::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

async fn load_root_key_pair() -> KeyPair {
    let env_var = "PRIVATE_KEY_FILE";

    if let Some(path) = env::var_os(env_var) {
        let path = PathBuf::from(path);
        if path.is_file() {
            info!("Loading private key from {path:?}");
            let mut private_key_file = File::open(&path).await.unwrap();
            let mut private_key_bytes = Vec::new();
            private_key_file
                .read_to_end(&mut private_key_bytes)
                .await
                .unwrap();
            let private_key = PrivateKey::from_bytes(&private_key_bytes).unwrap();
            KeyPair::from(private_key)
        } else {
            info!("Creating new private key at {path:?}");
            let key_pair = KeyPair::new();
            let mut private_key_file = File::create(&path).await.unwrap();
            private_key_file
                .write_all(&key_pair.private().to_bytes())
                .await
                .unwrap();
            key_pair
        }
    } else {
        panic!("Environment variable {env_var} not set");
    }
}

async fn load_session_secret() -> Vec<u8> {
    let env_var = "SESSION_SECRET_FILE";

    if let Some(path) = env::var_os(env_var) {
        let path = PathBuf::from(path);
        if path.is_file() {
            info!("Loading session secret from {path:?}");
            let mut session_secret_file = File::open(&path).await.unwrap();
            let mut session_secret_bytes = Vec::new();
            session_secret_file
                .read_to_end(&mut session_secret_bytes)
                .await
                .unwrap();
            session_secret_bytes
        } else {
            info!("Creating new session secret at {path:?}");
            let mut session_secret_bytes = vec![0; 64];
            rand::rngs::OsRng::default().fill_bytes(&mut session_secret_bytes);
            let mut session_secret_file = File::create(&path).await.unwrap();
            session_secret_file
                .write_all(&session_secret_bytes)
                .await
                .unwrap();
            session_secret_bytes
        }
    } else {
        panic!("Environment variable {env_var} not set");
    }
}

async fn migrate_database(db_conn: &DatabaseConnection) {
    info!("Running database migrations if necessary");
    Migrator::up(db_conn, None).await.unwrap();
    info!("Done running database migrations");
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    initialise_logging(cli.log_level);
    dotenv().unwrap();

    let root_key_pair = load_root_key_pair().await;
    let session_secret = load_session_secret().await;

    let db_conn = sea_orm::Database::connect("postgres://axum_test:axum_test@localhost/axum_test")
        .await
        .unwrap();
    migrate_database(&db_conn).await;

    api::serve(&db_conn, &session_secret).await;
}
