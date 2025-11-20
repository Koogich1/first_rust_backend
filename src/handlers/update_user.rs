use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{users_model::User, users_model::UpdateUser};
use log;

pub async fn update_user_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    user_data: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user_data = user_data.into_inner();

    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        User::update(&mut conn, user_id, &user_data)
    })
    .await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(diesel::result::Error::NotFound)) => {
            HttpResponse::NotFound().body(format!("User with id {} not found", user_id))
        }
        Ok(Err(e)) => {
            log::error!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
        Err(e) => {
            log::error!("Error during web::block execution: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update user due to an internal error")
        }
    }
}