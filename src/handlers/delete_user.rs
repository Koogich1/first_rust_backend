use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{users_model::User};
use log;

pub fn delete_user_handler(
	pool: web::Data<PgPool>,
	path: web::Path<i32>,
) -> impl Responder {
	let user_id = path.into_inner();
	log::info!("Deleting user ID: {}", user_id);

	let pool = pool.clone();

	let result = web::block(move || {
		let mut conn = pool.get().expect("Failed to get DB connection");
		User::delete(&mut conn, user_id)
	});

	match result {
		Ok(Ok(user)) => HttpResponse::Ok().json(user),
    
		Ok(Err(diesel::result::Error::NotFound)) => {
      HttpResponse::NotFound().body(format!("User with id {} not found", user_id))
    }
	}
}