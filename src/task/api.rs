use crate::task::{repository, InsertableTask, Task, TaskError};
use actix_web::{get, post, delete, web, Responder, Result};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn get_all_tasks(state: web::Data<crate::AppState>) -> Result<web::Json<Vec<Task>>> {
    let db = state.db.lock().unwrap().clone();
    let tasks = repository::all(&db, 10).await?;
    Ok(web::Json(tasks))
}

#[get("/{id}")]
async fn get_task(
    path: web::Path<String>,
    state: web::Data<crate::AppState>,
) -> Result<web::Json<Task>, TaskError> {
    let id = path.into_inner();
    let db = state.db.lock().unwrap().clone();
    let res = repository::find_by_id(&db, id).await?;
    match res {
        Some(v) => Ok(web::Json(v)),
        None => Err(TaskError {
            message: "task not found".to_string(),
        }),
    }
}

#[derive(Deserialize, Serialize)]
struct CreateTaskResponse {
    id: String,
}
#[post("/")]
async fn create_task(
    body: web::Json<InsertableTask>,
    state: web::Data<crate::AppState>,
) -> Result<impl Responder> {
    let body = body.into_inner();
    let db = state.db.lock().unwrap().clone();
    let id = repository::create(&db, body).await?;
    Ok(web::Json(CreateTaskResponse { id }))
}

#[delete("/{id}")]
async fn delete_task(path: web::Path<String>, state: web::Data<crate::AppState>) -> Result<impl Responder, TaskError> {
    let id = path.into_inner();
    let db = state.db.lock().unwrap().clone();
    match repository::delete(&db, id).await {
        Ok(_) => Ok(web::Json(1)),
        Err(e) => Err(e)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tasks)
        .service(create_task)
        .service(get_task)
        .service(delete_task);
}
