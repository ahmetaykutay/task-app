pub mod api;
pub mod repository;

use serde::{Deserialize, Serialize, ser::SerializeStruct};
use mongodb::bson;
use derive_more::{Display, Error};
use actix_web::error;


// TODO use enum to specify error types
#[derive(Debug, Display, Error)]
#[display(fmt = "error: {}", message)]
pub struct TaskError {
    message: String,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for TaskError {}
impl From<mongodb::error::Error> for TaskError {
    fn from(e: mongodb::error::Error) -> Self {
        TaskError {message: format!("{}", e.kind.to_string())}
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    pub id: bson::oid::ObjectId,
    pub name: Option<String>,
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Task", 2)?;
        state.serialize_field("id", &self.id.to_string())?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableTask {
    pub name: Option<String>,
}

