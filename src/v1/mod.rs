pub mod profile_routes;
pub mod user_routes;

// Import the user_routes and profile_routes modules
// then put them in a scope with the /v1 prefix and return it
pub fn v1_routes() -> actix_web::Scope {
    actix_web::web::scope("/v1")
        .service(user_routes::user_routes())
        .service(profile_routes::profile_routes())
}
