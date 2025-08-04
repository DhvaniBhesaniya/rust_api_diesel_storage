pub mod handlers;
pub mod models;
pub mod schema;

use actix_cors::Cors;
use actix_web::http::header;
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

    let server_addr = ("0.0.0.0", 8080);

    println!(
        "🚀 Server running at http://{}:{}",
        server_addr.0, server_addr.1
    );

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allow_any_origin()
                    //     .allow_any_method()
                    //     .allow_any_header()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
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
