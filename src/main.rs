use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

mod model;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {}
