use dotenv::dotenv;
use store::PostgresStore;

mod models;
mod server;
mod store;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = server::Config::new_from_env();
    let store = PostgresStore {};

    let _server = server::start(config, store).await;

    Ok(())
}
