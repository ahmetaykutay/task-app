use crate::task::{InsertableTask, Task, TaskError};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid};
use mongodb::options::FindOptions;
use mongodb::Database;

const COLLECTION: &str = "tasks";

pub async fn all(db: &Database, limit: i64) -> Result<Vec<Task>, TaskError> {
    let collection = db.collection::<Task>(COLLECTION);
    let find_options = FindOptions::builder().limit(limit).build();

    let mut cursor = collection.find(None, find_options).await?;
    let mut tasks: Vec<Task> = Vec::new();
    // Iterate over the results of the cursor.
    while let Some(task) = cursor.try_next().await.expect("another error here") {
        tasks.push(task);
    }
    Ok(tasks)
}

pub async fn create(db: &Database, new_task: InsertableTask) -> Result<String, TaskError> {
    let new_task = doc! {
        "name": new_task.name,
    };
    let tasks = db.collection(COLLECTION);
    let insert_result = tasks.insert_one(new_task, None).await?;
    Ok(insert_result.inserted_id.to_string())
}
pub async fn find_by_id(db: &Database, id: String) -> Result<Option<Task>, TaskError> {
    let tasks = db.collection(COLLECTION);
    let id = oid::ObjectId::parse_str(id).unwrap();
    let task: Option<Task> = tasks.find_one(Some(doc! { "_id": id }), None).await?;
    Ok(task)
}

pub async fn update(db: &Database, id: String, updates: mongodb::bson::Document) -> Result<(), TaskError> {
    let id = oid::ObjectId::parse_str(id).unwrap();
    let query = doc! { "_id": id };
    let update = doc! { "$set": updates };
    let tasks = db.collection::<Task>(COLLECTION);
    tasks.update_one(query, update, None).await?;
    Ok(())
}

pub async fn delete(db: &Database, id: String) -> Result<(), TaskError> {
    let tasks = db.collection::<Task>(COLLECTION);
    let id = oid::ObjectId::parse_str(id).expect("Couldn't par id");
    tasks.delete_one(doc! { "_id": id }, None).await?;
    Ok(())
}

