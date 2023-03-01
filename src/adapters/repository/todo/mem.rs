use crate::domain::todo::error::TodoError;
use crate::domain::todo::{Todo, TodoRepository};
use async_trait::async_trait;
use dashmap::DashMap;
use uuid::Uuid;

/// Хранилище задач в памяти
pub struct TodoMemoryRepository(DashMap<Uuid, Todo>);

impl TodoMemoryRepository {
    pub fn new() -> Self {
        Self(DashMap::new())
    }
}

#[async_trait]
impl TodoRepository for TodoMemoryRepository {
    async fn create(&self, todo: Todo) -> Result<(), TodoError> {
        self.0.insert(todo.id, todo);
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Todo>, TodoError> {
        Ok(self.0.iter().map(|entry| entry.value().clone()).collect())
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Todo, TodoError> {
        Ok(self
            .0
            .get(&id)
            .ok_or(TodoError::NotFound(id.to_string()))?
            .value()
            .clone())
    }

    async fn save(&self, todo: Todo) -> Result<(), TodoError> {
        self.0.insert(todo.id, todo);
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), TodoError> {
        self.0
            .remove(&id)
            .ok_or(TodoError::NotFound(id.to_string()))?;

        Ok(())
    }
}
