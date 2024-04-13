#[path = "pg/pg.rs"]
pub mod pg;

use diesel::RunQueryDsl;
use diesel::PgConnection;
use pg::query_helper;


// TODO: Feature/error handling/abstraction.
pub fn create_db_if_needed(db_name: &str, conn: &mut PgConnection) {
    query_helper::create_database(&db_name).execute(conn).expect("Could not create database!");
}
