use diesel::PgConnection;

#[database("my_db")]
pub struct MyDatabase(PgConnection);
