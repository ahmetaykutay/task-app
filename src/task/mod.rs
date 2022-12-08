pub mod api;
pub mod repository;

use serde::{Deserialize, Serialize};
use mongodb::bson;
use derive_more::{Display, Error};
use actix_web::{error};


#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", message)]
struct TaskError {
    message: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for TaskError {}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    // #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    pub id: bson::oid::ObjectId,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTask {
    pub name: Option<String>,
}

