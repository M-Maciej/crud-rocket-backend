#[macro_use] extern crate rocket;

pub mod routes;

use routes::*;
//use rocket::State;
use rocket::tokio::sync::RwLock;
use rocket::futures::lock::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

type ID = u32;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Item {
    id: ID,
    name: String,
}

#[launch]
fn rocket() -> _ {
    let database: Arc<RwLock<HashMap<ID, Item>>>= Arc::new(RwLock::new(HashMap::new()));
    rocket::build()
        .manage(database)
        .mount("/", public_routes())
}
