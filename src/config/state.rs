use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use log::info;
use std::sync::{Arc, Mutex};

use super::config::read_config;

/// Helps with changing the database engine without much edits.
pub type DatabaseConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;

pub struct State {
    // pub users: Arc<Mutex<Vec<User>>>,
    pool: Arc<Mutex<Pool>>,
}

// impl State {
//     fn get_pool_conn(&mut self) {
//         info!("Get get_pool_conn");
//         let config = read_config();
//         self.pool = Pool::builder()
//             .build(ConnectionManager::<DatabaseConnection>::new(
//                 config.database,
//             ))
//             .unwrap();

//         // self.pool = database_pool;
//     }
// }
