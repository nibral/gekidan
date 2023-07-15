use dotenv::dotenv;
use sea_orm::{Database, DbConn};
use crate::domain::constants::SQLITE3_DB_URL;

pub async fn db_conn() -> DbConn {
    dotenv().ok();
    let database_uri = dotenv::var(SQLITE3_DB_URL)
        .expect(&*format!("{} must be set", SQLITE3_DB_URL));
    Database::connect(database_uri)
        .await
        .expect("Failed to connect database")
}
