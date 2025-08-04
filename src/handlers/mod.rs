use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::{Task, NewTask};
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use crate::DbPool;

// -- GET /tasks --
#[get("/tasks")]
async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get DB connection");
        tasks.load::<Task>(&mut conn)
    }).await;

    match result {
        Ok(Ok(task_list)) => HttpResponse::Ok().json(task_list),
        Ok(Err(_)) | Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- POST /tasks --
#[post("/tasks")]
async fn create_task(
    pool: web::Data<DbPool>,
    new_task: web::Json<NewTask>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    let inserted = web::block(move || {
        diesel::insert_into(tasks)
            .values(new_task.into_inner())
            .execute(&mut conn)
    })
    .await;

    match inserted {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- PUT /tasks/{id} --
#[put("/tasks/{id}")]
async fn update_task(
    pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
    updated_data: web::Json<NewTask>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    let updated = web::block(move || {
        diesel::update(tasks.find(task_id.into_inner()))
            .set(title.eq(updated_data.title.clone()))
            .execute(&mut conn)
    })
    .await;

    match updated {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- DELETE /tasks/{id} --
#[delete("/tasks/{id}")]
async fn delete_task(
    pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    let deleted = web::block(move || {
        diesel::delete(tasks.find(task_id.into_inner())).execute(&mut conn)
    })
    .await;

    match deleted {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- 🔗 init_routes: Register All Routes Here --
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks)
       .service(create_task)
       .service(update_task)
       .service(delete_task);
}