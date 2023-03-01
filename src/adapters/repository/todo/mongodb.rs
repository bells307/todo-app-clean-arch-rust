use crate::domain::todo::error::TodoError;
use crate::domain::todo::{Todo, TodoRepository};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::UpdateModifications;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

const COLLECTION_NAME: &str = "todo";

pub struct TodoMongoDBRepository {
    db: Database,
}

impl TodoMongoDBRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn collection(&self) -> Collection<TodoDocument> {
        self.db.collection(COLLECTION_NAME)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TodoDocument {
    pub _id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub done: bool,
}

impl From<Todo> for TodoDocument {
    fn from(value: Todo) -> Self {
        Self {
            _id: value.id.to_string(),
            name: value.name.clone(),
            created: value.created,
            done: value.done,
        }
    }
}

impl TryFrom<TodoDocument> for Todo {
    type Error = uuid::Error;

    fn try_from(value: TodoDocument) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value._id)?,
            name: value.name,
            created: value.created,
            done: value.done,
        })
    }
}

#[async_trait]
impl TodoRepository for TodoMongoDBRepository {
    async fn create(&self, todo: Todo) -> Result<(), TodoError> {
        self.collection()
            .insert_one(TodoDocument::from(todo), None)
            .await
            .map(|_| ())
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))
    }

    async fn get_all(&self) -> Result<Vec<Todo>, TodoError> {
        self.collection()
            .find(None, None)
            .await
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))?
            .try_collect::<Vec<TodoDocument>>()
            .await
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))?
            .into_iter()
            .map(|d| {
                d.try_into()
                    .map_err(|e| TodoError::Other(anyhow::Error::from(e)))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Todo, TodoError> {
        self.collection()
            .find_one(doc! {"_id": id.to_string()}, None)
            .await
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))?
            .ok_or(TodoError::NotFound(id.to_string()))
            .and_then(|d| {
                d.try_into()
                    .map_err(|e| TodoError::Other(anyhow::Error::from(e)))
            })
    }

    async fn save(&self, todo: Todo) -> Result<(), TodoError> {
        self.collection()
            .update_one(
                doc! {"_id": todo.id.to_string()},
                UpdateModifications::Document(doc! {"$set": {
                    "name": todo.name,
                    "created": todo.created.to_string(),
                    "done": todo.done
                }}),
                None,
            )
            .await
            .map(|_| ())
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))
    }

    async fn delete(&self, id: Uuid) -> Result<(), TodoError> {
        self.collection()
            .delete_one(doc! {"_id": todo.id.to_string()}, None)
            .await
            .map(|_| ())
            .map_err(|e| TodoError::Other(anyhow::Error::from(e)))
    }
}
