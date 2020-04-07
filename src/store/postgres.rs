#[derive(Clone)]
pub struct PostgresStore {}

impl super::DataStore for PostgresStore {
    fn engine_type() -> String {
        "postgres".to_string()
    }
}
