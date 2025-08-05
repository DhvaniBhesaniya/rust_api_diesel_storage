pub mod handlers;
pub mod models;
pub mod schema;

use actix_cors::Cors;
use actix_web::http::header::{self, ORIGIN};
use actix_web::{App, HttpServer, web};
use diesel::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager}; // or PgConnection

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server_addr = ("0.0.0.0", 1234);

    println!(
        "🚀 Server running at http://{}:{}",
        server_addr.0, server_addr.1
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allow_any_origin() // <--- Allow all origins
                    .allowed_origin("http://127.0.0.1:5500") // <--- Allow specific origin
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::init_routes)
    })
    .bind(server_addr)?
    .run()
    .await
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(web::Data::new(pool.clone()))
    //         .configure(handlers::init_routes)
    // })
    // .bind(server_addr)?
    // .run()
    // .await
}
