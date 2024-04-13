#[path = "db/db.rs"]
mod db;

mod config;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use fusion_util as util;

pub fn establish_connection() -> PgConnection {
    let config = config::read_config();

    // TODO: Abstract out.
    let mut db_url = String::new();
    db_url.push_str("postgres://");
    db_url.push_str(&config.username);
    db_url.push(':');
    db_url.push_str(&config.password);
    db_url.push('@');
    db_url.push_str(&config.host);
    db_url.push(':');
    db_url.push_str(&config.port.to_string().as_str());
    db_url.push('/');

    let mut default_url = db_url.clone();
    default_url.push_str("postgres");
    
    db_url.push_str(&config.name);
    // TODO: Abstract out.

    let db_test = PgConnection::establish(&db_url);

    match db_test {
        Ok(db) => {
            return db;
        },
        Err(e) => {
            let mut conn = PgConnection::establish(&default_url).expect("Could not connect to default db.");
            db::create_db_if_needed(&config.name, &mut conn);

            let db_test_2 = PgConnection::establish(&db_url);

            return db_test_2.expect("Something went wrong connecting to the newly created database!");
        }
    }
}
