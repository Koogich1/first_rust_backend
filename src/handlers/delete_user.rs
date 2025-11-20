use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{users_model::User};
use log;

pub async fn delete_user_handler(
	pool: web::Data<PgPool>,
	path: web::Path<i32>,
) -> impl Responder {
	let user_id = path.into_inner();
	log::info!("Deleting user ID: {}", user_id);

	let pool = pool.clone();

	let result = web::block(move || {
		let mut conn = pool.get().expect("Failed to get DB connection");
		User::delete(&mut conn, user_id)
	}).await;

	match result {
		Ok(Ok(deleted_count)) => {
				if deleted_count > 0 {
						HttpResponse::Ok().json(format!("User {} deleted successfully", user_id))
				} else {
						HttpResponse::NotFound().json("User not found")
				}
		},
		Ok(Err(diesel_error)) => {
				log::error!("Diesel error: {:?}", diesel_error);
				match diesel_error {
						diesel::result::Error::NotFound => HttpResponse::NotFound().json("User not found"),
						_ => HttpResponse::InternalServerError().json("Database error"),
				}
		},
		Err(blocking_error) => {
				log::error!("Blocking error: {:?}", blocking_error);
				HttpResponse::InternalServerError().json("Failed to process request")
		}
	}
}