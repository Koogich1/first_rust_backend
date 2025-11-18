use actix_web::{web, HttpResponse, Responder};
use crate::db::PgPool;
use crate::models::{ User};

pub async fn get_all_users_handler(
    pool: web::Data<PgPool>
) -> impl Responder {
    
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get DB connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().body("Database connection error");
        }
    };

    let users_fetch_result = web::block  (
        move || {
            User::get_all(&mut conn)
        }
        
    )
    .await;

    match users_fetch_result {
        Ok(db_operation_result) => { // Result<Result<Vec<User>, diesel::result::Error>, actix_web::Error>
            match db_operation_result {
                Ok(users) => {
                    // Если пользователи успешно получены, возвращаем их в формате JSON с HTTP 200 OK
                    HttpResponse::Ok().json(users)
                },
                Err(e) => {
                    // Если произошла ошибка в Diesel (например, ошибка SQL)
                    log::error!("Failed to fetch users from database: {:?}", e);
                    HttpResponse::InternalServerError().body("Failed to retrieve users")
                }
            }
        },
        Err(e) => {
            // Если блокирующая операция "упала" или не смогла запуститься
            log::error!("Blocking operation failed for get_all_users: {:?}", e);
            HttpResponse::InternalServerError().body("Internal server error during user retrieval")
        }
    }
}