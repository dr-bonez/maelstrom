use crate::store::DataStore;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

mod handlers;
mod routes;

#[derive(Clone)]
pub struct Config {
    /// The port and address to run the server on
    pub server_addr: String,
    /// The hostname of the server, used to construct user's id
    pub hostname: String,
    /// Storage Engine Type
    pub store_type: String,
    /// Database URL (will distinquish between postgres, sqlite, sled)
    pub database_url: String,
}

impl Config {
    /// Returns a new SeverConfig by attempting
    /// to load from `env` vars.  Panics if
    /// any are missing.
    pub fn new_from_env() -> Self {
        Self {
            server_addr: env::var("SERVER_ADDR").expect("SERVER_ADDR env var missing."),
            hostname: env::var("HOSTNAME").expect("HOSTNAME env var missing."),
            store_type: env::var("STORE_TYPE").expect("STORE_TYPE env var missing."),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL env var missing."),
        }
    }
}

pub struct State<T: DataStore> {
    pub config: Config,
    pub store: T,
}

/// Starts the server. Takes a `ServerConfig`.
pub async fn start<T: DataStore + Sync + Send>(config: Config, store: &T) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let addr = config.server_addr.clone();

    HttpServer::new(move || {
        App::new()
            .data(State {
                config: config.clone(),
                store: store.clone(),
            })
            .wrap(Cors::new().send_wildcard().finish())
            .wrap(Logger::default())
            .configure(routes::config)
    })
    .bind(addr)?
    .run()
    .await
}
