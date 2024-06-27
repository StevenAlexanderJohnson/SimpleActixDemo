use std::str::FromStr;

use crate::models::profile::Profile;

use actix_web::{guard, web, HttpResponse, Responder, Scope};
use mongodb::bson::{doc, oid::ObjectId};

async fn get_profile(db: web::Data<mongodb::Client>, user_id: web::Path<String>) -> impl Responder {
    let profile = match db
        .database("testing")
        .collection::<Profile>("profile")
        .find_one(
            mongodb::bson::doc! {
                "_id": ObjectId::from_str(&user_id).unwrap()
            },
            None,
        )
        .await
    {
        Ok(profile) => match profile {
            Some(profile) => profile,
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
    HttpResponse::Ok().json(profile)
}

async fn post_profile(db: web::Data<mongodb::Client>, req_body: String) -> impl Responder {
    let profile = serde_json::from_str::<Profile>(&req_body).unwrap();
    match db
        .database("testing")
        .collection::<&Profile>("profile")
        .insert_one(&profile, None)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

async fn patch_profile(db: web::Data<mongodb::Client>, req_body: String) -> impl Responder {
    let profile = serde_json::from_str::<Profile>(&req_body).unwrap();
    match db
        .database("testing")
        .collection::<&Profile>("profile")
        .update_one(
            doc! {"_id": profile._id},
            {
                doc! {"$set": doc! {"name": &profile.name, "email": &profile.email, "age": &profile.age, "address": &profile.address, "phone": &profile.phone, "bio": &profile.bio}}
            },
            None,
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().json(profile),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

// Create a new scope for the /profile routes
pub fn profile_routes() -> Scope {
    web::scope("/profile")
        .route("/{user_id}", web::get().to(get_profile))
        .route(
            "",
            web::post()
                .guard(guard::Header("content-type", "application/json"))
                .to(post_profile),
        )
        .route(
            "",
            web::patch()
                .guard(guard::Header("content-type", "application/json"))
                .to(patch_profile),
        )
}
