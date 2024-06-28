mod database;
mod models;
mod v1;

use actix_web::{web, App, HttpServer};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client = match Client::with_uri_str(mongo_uri).await {
        Ok(client) => client,
        Err(e) => panic!("Failed to connect to MongoDB: {:?}", e),
    };

    println!("Listening on :8080");

    HttpServer::new(move || {
        App::new().service(
            // Scope the API routes to /api
            web::scope("/api")
                // Register the V1 routes
                .service(v1::v1_routes())
                // Bind the MongoDB client to the app data
                .app_data(web::Data::new(database::Database::new(client.clone()))),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
