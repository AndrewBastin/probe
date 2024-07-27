use rocket_db_pools::Database;

#[derive(Database)]
#[database("db")]
pub struct DB(sqlx::SqlitePool);
