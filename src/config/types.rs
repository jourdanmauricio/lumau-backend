use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

/// Helps with changing the database engine without much edits.
pub type DatabaseConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;
