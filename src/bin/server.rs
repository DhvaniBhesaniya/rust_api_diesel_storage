use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::thread;
use actix_web::rt;
use reqwest;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from port 3000!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the server on port 3000 in a new thread
    let server_handle = thread::spawn(|| {
        rt::System::new().block_on(async {
            HttpServer::new(|| {
                App::new().route("/", web::get().to(hello))
            })
            .bind(("0.0.0.0", 3000))
            .expect("Can not bind to port 3000")
            .run()
            .await
            .expect("Server on 3000 failed");
        });
    });

    // Do a GET request to http://localhost:8080/tasks
    let client = reqwest::Client::new();
    match client.get("http://localhost:8080/tasks").send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "<Failed to read body>".to_string());
            println!("GET http://localhost:8080/tasks => {}\n{}", status, body);
        }
        Err(e) => {
            println!("Failed to GET from 8080: {}", e);
        }
    }

    // Wait for the server thread to finish (it won't, so join will block)
    let _ = server_handle.join();
    Ok(())
}