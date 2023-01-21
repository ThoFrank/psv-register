use diesel::prelude::*;
use diesel::Connection;

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Couldn't connect to database!")
}
