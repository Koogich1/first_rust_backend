use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod db;
mod models;
mod schema;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let pool = db::establish_connection_pool();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());

    println!("Starting server at http://{}:{}/", host, port);
    println!("Available routes:");
    println!("  POST   /api/users");
    println!("  GET    /api/user/{{id}}");
    println!("  GET    /api/users/all");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::new(
                "%a %r %s %b %Dms"
            ))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(handlers::create_user::create_user_handler))
                    .route("/user/{id}", web::get().to
                    (handlers::get_user_by_id::get_user_by_id_handler))
                    .route("/user/{id}", web::patch().to(handlers::update_user::update_user_handler))
                    .route("/user/{id}", web::delete().to(handlers::delete_user::delete_user_handler))
                    .route("/users/all", web::get().to(handlers::get_all_users::get_all_users_handler))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}