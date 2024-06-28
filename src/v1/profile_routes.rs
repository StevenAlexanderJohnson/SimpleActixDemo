use crate::database::Database;
use crate::database::DatabaseTrait;
use crate::models::profile::Profile;

use actix_web::{guard, web, HttpResponse, Responder, Scope};

async fn get_profile<T>(db: web::Data<T>, user_id: web::Path<String>) -> impl Responder
where
    T: DatabaseTrait + 'static,
{
    let profile = match db.get_profile(&user_id).await {
        Ok(profile) => match profile {
            Some(profile) => profile,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };

    HttpResponse::Ok().json(profile)
}

async fn post_profile<T>(db: web::Data<T>, req_body: String) -> impl Responder
where
    T: DatabaseTrait + 'static,
{
    let profile = serde_json::from_str::<Profile>(&req_body).unwrap();

    match db.post_profile(&profile).await {
        Ok(_) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

async fn patch_profile<T>(db: web::Data<T>, req_body: String) -> impl Responder
where
    T: DatabaseTrait + 'static,
{
    let profile = serde_json::from_str::<Profile>(&req_body).unwrap();

    match db.patch_profile(&profile).await {
        Ok(_) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Create a new scope for the /profile routes
pub fn profile_routes() -> Scope {
    web::scope("/profile")
        .route("/{user_id}", web::get().to(get_profile::<Database>))
        .route(
            "",
            web::post()
                .guard(guard::Header("content-type", "application/json"))
                .to(post_profile::<Database>),
        )
        .route(
            "",
            web::patch()
                .guard(guard::Header("content-type", "application/json"))
                .to(patch_profile::<Database>),
        )
}
