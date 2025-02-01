use rocket::tokio::sync::RwLock;
use rocket::State;
use std::collections::HashMap;
use std::sync::Arc;
use crate::Item;
use rocket::serde::json::Json;

// Define the shared state type
type Db = Arc<RwLock<HashMap<ID, Item>>>;
type ID = u32;

// Example "Hello, world!" route
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

// Read item by name
#[get("/read/<name>")]
async fn read(name: &str, db: &State<Db>) -> Option<Json<Item>> {
    let database =db.read().await;
    database.values()
    .find(|item| item.name==name)
    .map(|item| Json(new))

}


// Create a new item
#[post("/create/<name>")]
async fn create(name: &str, db: &State<Db>) -> Json<Item> {
    let mut database = db.write().await;  // Use write lock for write access
    let id = (database.len() as u32) + 1;
    let new_item = Item { id, name: name.to_string() };
    database.insert(id, new_item.clone());
    Json(new_item)
}

// Delete item by name
#[delete("/delete/<name>")]
async fn delete(name: &str, db: &State<Db>) -> Option<Json<Item>> {
    let mut database = db.write().await;  // Use write lock for write access
    if let Some((id, item)) = database.iter().find(|(_, item)| item.name == name).map(|(&id, item)| (id, item.clone())) {
        database.remove(&id);
        Some(Json(item))
    } else {
        None
    }
}

// Update item by name (just rename for simplicity)
#[patch("/update/<old_name>/<new_name>")]
async fn update(old_name: &str, new_name: &str, db: &State<Db>) -> Option<Json<Item>> {
    let mut database = db.write().await;  // Use write lock for write access
    if let Some((_, item)) = database.iter_mut().find(|(_, item)| item.name == old_name) {
        item.name = new_name.to_string();
        Some(Json(item.clone()))
    } else {
        None
    }
}

// Mount routes
pub fn public_routes() -> Vec<rocket::Route> {
    routes![hello, read, create, delete, update]
}
