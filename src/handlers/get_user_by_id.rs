use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{users_model::User};

pub async fn get_user_by_id_handler(
	pool: web::Data<PgPool>,
	path: web::Path<i32>
) -> impl Responder {
	let user_id = path.into_inner();
	println!("Received request for user ID: {}", user_id);

	let mut conn = match pool.get() {
		Ok(conn) => conn,
		Err(e) => {
				log::error!("Failed to get DB connection from pool: {:?}", e);
				return HttpResponse::InternalServerError().body("Database connection error");
		}
	};

	let user_fet_result = web::block(
		move || {
			User::get_by_id(&mut conn, user_id)
		}
	)
	.await;

	match user_fet_result {
		Ok(Ok(user)) => {
			HttpResponse::Ok().json(user)
		},
		Ok(Err(diesel::result::Error::NotFound)) => {
			log::warn!("User with ID {} not found.", user_id);
			HttpResponse::NotFound().body(format!("User with id {} not found", user_id))
		},
		Ok(Err(e)) => {
			log::error!("Failed to fetch user from database: {:?}", e);
			HttpResponse::InternalServerError().body("Failed to retrieve user")
		},
		Err(e) => {
			log::error!("Error during web::block execution: {:?}", e);
			HttpResponse::InternalServerError().body("Failed to retrieve user due to an internal error")
		}
	}
}