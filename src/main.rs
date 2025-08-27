use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::sync::Arc;

use auth::db::DieselRepo; // ← auth_api = crate name from Cargo.toml
use auth::{login_handler, AppState, AuthRepo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (postgres://...)");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Failed to create DB pool");

    let repo: Arc<dyn AuthRepo> = Arc::new(DieselRepo::new(pool));
    let state = AppState { repo };

    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    log::info!("Starting server on {bind}");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .route("/login", web::post().to(login_handler))
    })
    .bind(bind)?
    .run()
    .await
}

