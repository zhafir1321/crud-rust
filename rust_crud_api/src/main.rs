use actix_web::{web, App, HttpServer, middleware::Logger};
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncMysqlConnection;
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;
mod handlers;

// Type alias for the connection pool for easier use
type DbPool = Pool<AsyncMysqlConnection>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a new connection manager
    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);

    // Create a new connection pool
    let pool = Pool::builder()
        .build(config)
        .await
        .expect("Failed to create database connection pool.");

    println!("🚀 Server started successfully at http://127.0.0.1:8080");

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            // Add the connection pool to the application data
            .app_data(web::Data::new(pool.clone()))
            // Add a logger middleware
            .wrap(Logger::default())
            // Define application routes
            .service(
                web::scope("/products")
                    .route("", web::post().to(handlers::create_product))
                    .route("", web::get().to(handlers::get_products))
                    .route("/{id}", web::get().to(handlers::get_product_by_id))
                    .route("/{id}", web::put().to(handlers::update_product_by_id))
                    .route("/{id}", web::delete().to(handlers::delete_product_by_id)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
