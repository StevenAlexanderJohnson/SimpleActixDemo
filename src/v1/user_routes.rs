use crate::models::user::User;
use actix_web::{guard, web, HttpResponse, Responder, Scope};

async fn get_user() -> impl Responder {
    let user = User {
        name: "John".to_string(),
        email: "test@email.com".to_string(),
    };

    HttpResponse::Ok().json(user)
}

async fn post_user(req_body: String) -> impl Responder {
    let user_body = serde_json::from_str::<User>(&req_body).unwrap();

    HttpResponse::Ok().json(user_body)
}

// Create a new scope for the /user routes
pub fn user_routes() -> Scope {
    web::scope("/user")
        .route("", web::get().to(get_user))
        .route(
            "",
            web::post()
                .guard(guard::Header("content-type", "application/json"))
                .to(post_user),
        )
}
