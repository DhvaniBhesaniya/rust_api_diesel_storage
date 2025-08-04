use crate::DbPool;
use crate::models::{NewTask, Task, UpdateTask};
use crate::schema::tasks::dsl::*;
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use diesel::prelude::*;

// -- GET /tasks --
#[get("/tasks")]
async fn get_tasks(pool: web::Data<DbPool>) -> impl Responder {
    let pool = pool.clone();
    let result = web::block(move || {
        let mut conn = pool.get().expect("Couldn't get DB connection");
        tasks.load::<Task>(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(task_list)) => HttpResponse::Ok().json(task_list),
        Ok(Err(_)) | Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- POST /tasks --
#[post("/tasks")]
async fn create_task(pool: web::Data<DbPool>, new_task: web::Json<NewTask>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");
    let new_title = new_task.title.clone();

    // Check if a task with the same title exists
    let exists = web::block(move || {
        tasks
            .filter(title.eq(&new_title))
            .first::<Task>(&mut conn)
            .optional()
    })
    .await
    .unwrap();

    match exists {
        Ok(Some(existing_task)) => {
            // Task with same title exists, return its id in the message
            let msg = format!("Same title is present for task id: {}.", existing_task.id);
            HttpResponse::BadRequest().json(msg)
        }
        Ok(None) => {
            // No task with same title, proceed to insert
            let mut conn = pool.get().expect("Couldn't get DB connection");
            let inserted = web::block(move || {
                diesel::insert_into(tasks)
                    .values(new_task.into_inner())
                    .execute(&mut conn)
            })
            .await;

            match inserted {
                Ok(_) => HttpResponse::Ok().json("Task created successfully."),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- PUT /tasks/{id} --
#[put("/tasks/{id}")]
async fn update_task(
    pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
    updated_data: web::Json<UpdateTask>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    // let updated = web::block(move || {
    //     diesel::update(tasks.find(task_id.into_inner()))
    //         .set(title.eq(updated_data.title.clone()))
    //         .execute(&mut conn)
    // })
    // .await;
    let task_id_clone = task_id.clone();
    let updated = web::block(move || {
        diesel::update(tasks.find(task_id.into_inner()))
            .set(&*updated_data)
            .execute(&mut conn)
    })
    .await;

    match updated {
        Ok(_) => {
            let msg = format!("Task updated successfully for id: {}.", task_id_clone);
            HttpResponse::Ok().json(msg)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// -- DELETE /tasks/{id} --
#[delete("/tasks/{id}")]
async fn delete_task(pool: web::Data<DbPool>, task_id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection");

    let task_id_clone = task_id.clone();
    // Check if the task exists
    let exists = web::block({
        let task_id = task_id.into_inner();
        move || tasks.find(task_id).first::<Task>(&mut conn).optional()
    })
    .await
    .unwrap();

    match exists {
        Ok(Some(_)) => {
            // Task exists, proceed to delete
            let mut conn = pool.get().expect("Couldn't get DB connection");
            let deleted =
                web::block(move || diesel::delete(tasks.find(task_id_clone)).execute(&mut conn))
                    .await;
            match deleted {
                Ok(_) => {
                    let msg = format!("Task deleted successfully for id: {:?}.", task_id_clone);
                    HttpResponse::Ok().json(msg)
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => {
            // Task does not exist
            HttpResponse::BadRequest().json("Invalid task id, task is not present.")
        }
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
