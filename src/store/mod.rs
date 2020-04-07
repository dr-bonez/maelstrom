pub mod postgres;
pub mod sqlite;

pub use postgres::PostgresStore;
pub use sqlite::SqliteStore;

pub trait DataStore: Clone {
    /// The storage engine type.
    ///
    /// e.g. `postgres`, `sqlite` `sled`
    fn engine_type() -> String;
}
