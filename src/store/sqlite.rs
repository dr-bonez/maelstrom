#[derive(Clone)]
pub struct SqliteStore {}

impl super::DataStore for SqliteStore {
    fn engine_type() -> String {
        "sqlite".to_string()
    }
}
