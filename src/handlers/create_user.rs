use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{NewUserRequest, User};

pub async fn create_user_handler(
	pool: web::Data<PgPool>,
	new_user_req: web::Json<NewUserRequest>,
) -> impl Responder {
	let new_user_data = new_user_req.into_inner();

	let mut conn = match pool.get() {
			Ok(conn) => conn,
			Err(e) => {
					log::error!("Failed to get DB connection from pool: {:?}", e);
					return HttpResponse::InternalServerError().body("Database connection error");
			}
	};

	// Операции с БД должны выполняться в блокирующем (blocking) потоке,
	// чтобы не блокировать основной асинхронный runtime.
	let user_creation_result = web::block(move || {
			User::create(&mut conn, new_user_data)
	})
	.await;

	match user_creation_result {
			Ok(user_result) => match user_result {
					Ok(user) => HttpResponse::Created().json(user),
					Err(diesel::result::Error::DatabaseError(db_error_kind, _) ) => {
							// Обработка конкретных ошибок базы данных, например, дубликатов
							if let diesel::result::DatabaseErrorKind::UniqueViolation = db_error_kind {
									HttpResponse::BadRequest().body("User with this username or email already exists")
							} else {
									log::error!("Database error during user creation: {:?}", db_error_kind);
									HttpResponse::InternalServerError().body("Failed to create user due to database error")
							}
					},
					Err(e) => {
							log::error!("Failed to create user: {:?}", e);
							HttpResponse::InternalServerError().body("Failed to create user")
					}
			},
			Err(e) => {
					log::error!("Blocking operation error: {:?}", e);
					HttpResponse::InternalServerError().body("Internal server error during user creation")
			}
	}
}