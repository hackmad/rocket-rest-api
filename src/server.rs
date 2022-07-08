use crate::db::*;
use crate::routes::*;
use rocket::{Build, Rocket};
use std::env;

pub fn build() -> Rocket<Build> {
    use rocket::figment::{
        util::map,
        value::{Map, Value},
    };

    let db: Map<_, Value> = map! {
        "url" => env::var("DATABASE_URL").expect("set DATABASE_URL").into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment().merge(("databases", map!["my_db" => db]));

    rocket::custom(figment)
        .attach(MyDatabase::fairing())
        .mount("/api/v1/", routes![get_all, new_user, find_user, login])
}
