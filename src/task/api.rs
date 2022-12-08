use crate::task::{repository, Task};
use actix_web::{error, get, post, web, Responder, Result};
use derive_more::{Display, Error};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn get_all_tasks(state: web::Data<crate::AppState>) -> Result<web::Json<Vec<Task>>> {
    let db = state.db.lock().unwrap().clone();
    let collection = db.collection::<Task>("tasks");
    drop(db);
    let find_options = FindOptions::builder().limit(10).build();

    let mut cursor = collection
        .find(None, find_options)
        .await
        .expect("error here");
    let mut tasks: Vec<Task> = Vec::new();
    // Iterate over the results of the cursor.
    while let Some(task) = cursor.try_next().await.expect("another error here") {
        tasks.push(task);
    }
    Ok(web::Json(tasks))
}

// #[get("/{id}")]
// async fn get_task(path: web::Path<String>, state: web::Data<crate::AppState>) -> Result<web::Json<Task>, TaskError>{
//     let id = path.into_inner();
//     let db = state.db.lock().unwrap();
//     let tasks = db.collection("tasks");
//     drop(db);
//     let cursor: Option<Task> = tasks.find_one(doc! { "_id": id }, None).await.unwrap();
//     match cursor {
//         Some(v) => Ok(web::Json(v)),
//         None => Err(TaskError{ message: "not found"}),
//     }
// }

#[derive(Deserialize)]
struct NewTask {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct CreateTaskResult {
    id: String,
}
#[post("/create")]
async fn create_task(
    body: web::Json<NewTask>,
    state: web::Data<crate::AppState>,
) -> Result<impl Responder> {
    let body = body.into_inner();
    let new_task = doc! {
        "name": body.name,
    };
    let db = state.db.lock().unwrap();
    let tasks = db.collection("tasks");
    drop(db);
    let insert_result = tasks.insert_one(new_task, None).await.unwrap();
    Ok(web::Json(CreateTaskResult {
        id: insert_result.inserted_id.to_string(),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tasks).service(create_task);
    // .service(get_task);
}
