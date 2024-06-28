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
        Some(e) => HttpResponse::InternalServerError().json(e.to_string()),
        None => HttpResponse::Ok().json(profile),
    }
}

// Create a new scope for the /profile routes
pub fn profile_routes<T>() -> Scope
where
    T: DatabaseTrait + 'static,
{
    web::scope("/profile")
        .route("/{user_id}", web::get().to(get_profile::<T>))
        .route(
            "",
            web::post()
                .guard(guard::Header("content-type", "application/json"))
                .to(post_profile::<T>),
        )
        .route(
            "",
            web::patch()
                .guard(guard::Header("content-type", "application/json"))
                .to(patch_profile::<T>),
        )
        .default_service(web::to(|| async {
            println!("HIT DEFAULT SERVICE");
            HttpResponse::NotFound().finish()
        }))
}

#[cfg(test)]
mod tests {
    use crate::database::mock_database::MockDatabase;
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_profile_get() {
        println!("STARTING test_profile_get");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(MockDatabase { should_return_error: false, should_return_none: false }))
                .service(profile_routes::<MockDatabase>()),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/profile/667db15292bb20cf886a7f61")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_profile_patch() {
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(MockDatabase { should_return_error: false, should_return_none: false }))
                .service(profile_routes::<MockDatabase>()),
        )
        .await;
        let req = test::TestRequest::patch()
            .uri("/profile")
            .set_json(&Profile {
                _id: None,
                address: "address".to_string(),
                name: "name".to_string(),
                bio: "Bio".to_string(),
                email: "email".to_string(),
                phone: "phone".to_string(),
                age: 1,
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Response: {:?}", resp);
        println!("Response Body: {:?}", resp.response().body());
        assert!(resp.status().is_success());
    }
}
