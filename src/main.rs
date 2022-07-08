#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sodiumoxide;

mod db;
mod models;
mod routes;
mod schema;
mod server;

#[launch]
fn rocket() -> _ {
    server::build()
}
