use crate::task::{InsertableTask, Task, TaskError};
use mongodb::{bson, Database};

const COLLECTION: &str = "tasks";

// pub fn all(db: Database) -> Result<Vec<Task>, TaskError> {
// }
