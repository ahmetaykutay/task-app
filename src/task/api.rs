use crate::task::{repository, InsertableTask, Task, TaskError};
use actix_web::{delete, get, post, put, web, Responder, Result};
use mongodb::bson::{doc, Document as BsonDocument};
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
async fn delete_task(
    path: web::Path<String>,
    state: web::Data<crate::AppState>,
) -> Result<impl Responder, TaskError> {
    let id = path.into_inner();
    let db = state.db.lock().unwrap().clone();
    match repository::delete(&db, id).await {
        Ok(_) => Ok(web::Json(1)),
        Err(e) => Err(e),
    }
}

fn are_keys_valid(updates: &serde_json::Map<String, serde_json::Value>) -> bool {
    let valid_keys = ["name".to_string()];
    for (key, _) in updates {
        if !valid_keys.contains(&key) {
            return false;
        }
    }
    true
}

#[put("/{id}")]
async fn update_task(
    path: web::Path<String>,
    state: web::Data<crate::AppState>,
    body: String,
) -> Result<impl Responder, TaskError> {
    let id = path.into_inner();
    let db = state.db.lock().unwrap().clone();

    let updates: serde_json::Value = serde_json::from_str(&body).unwrap();
    if !are_keys_valid(&updates.as_object().unwrap()) {
        return Err(TaskError {
            message: "keys not valid".to_string(),
        });
    }

    let updates: BsonDocument = serde_json::from_value(updates).unwrap();
    match repository::update(&db, id, updates).await {
        Ok(_) => Ok(web::Json(1)),
        Err(e) => Err(e),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tasks)
        .service(create_task)
        .service(get_task)
        .service(delete_task)
        .service(update_task);
}
