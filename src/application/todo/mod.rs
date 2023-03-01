pub mod dto;

use crate::domain::todo::error::TodoError;
use crate::domain::todo::{Todo, TodoRepository};
use thiserror::Error;
use uuid::Uuid;

/// Сервис работы с задачами
pub struct TodoService {
    /// Репозиторий задач
    repo: Box<dyn TodoRepository + Send + Sync>,
}

impl TodoService {
    pub fn new(repo: Box<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    /// Создание новой задачи
    pub async fn create(&self, dto: dto::CreateTodo) -> Result<Todo, TodoError> {
        let todo = Todo::new(dto.name)?;
        self.repo.create(todo.clone()).await?;
        Ok(todo)
    }

    /// Получить задачу
    pub async fn get(&self, id: Uuid) -> Result<Todo, TodoError> {
        self.repo.get_by_id(id).await
    }

    /// Получить все задачи
    pub async fn get_all(&self) -> Result<Vec<Todo>, TodoError> {
        self.repo.get_all().await
    }

    /// Пометить задачу как выполненную
    pub async fn mark_done(&self, id: Uuid) -> Result<(), TodoError> {
        let mut todo = self.repo.get_by_id(id).await?;
        todo.mark_done();
        self.repo.save(todo).await
    }

    /// Удалить задачу
    pub async fn delete(&self, id: Uuid) -> Result<(), TodoError> {
        self.repo.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::repository::todo::mem::TodoMemoryRepository;
    use crate::application::todo::dto::CreateTodo;
    use crate::application::todo::TodoService;
    use crate::domain::todo::error::TodoError;

    #[tokio::test]
    async fn create() {
        let repo = TodoMemoryRepository::new();
        let service = TodoService::new(Box::new(repo));

        let todo = service
            .create(CreateTodo {
                name: "some".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(todo.name(), "some".to_string());
    }

    #[tokio::test]
    async fn get() {
        let repo = TodoMemoryRepository::new();
        let service = TodoService::new(Box::new(repo));

        let created_todo = service
            .create(CreateTodo {
                name: "some".to_string(),
            })
            .await
            .unwrap();

        let got_todo = service.get(created_todo.id()).await.unwrap();
        assert_eq!(got_todo.id(), created_todo.id());
    }

    #[tokio::test]
    async fn get_all() {
        let repo = TodoMemoryRepository::new();
        let service = TodoService::new(Box::new(repo));

        let created_todo1 = service
            .create(CreateTodo {
                name: "some".to_string(),
            })
            .await
            .unwrap();

        let created_todo2 = service
            .create(CreateTodo {
                name: "another".to_string(),
            })
            .await
            .unwrap();

        let got_todos = service.get_all().await.unwrap();
        assert_eq!(got_todos[0].name(), created_todo1.name());
        assert_eq!(got_todos[1].name(), created_todo2.name());
    }

    #[tokio::test]
    async fn mark_done() {
        let repo = TodoMemoryRepository::new();
        let service = TodoService::new(Box::new(repo));

        let created_todo = service
            .create(CreateTodo {
                name: "some".to_string(),
            })
            .await
            .unwrap();

        service.mark_done(created_todo.id()).await.unwrap();

        let got_todo = service.get(created_todo.id()).await.unwrap();
        assert!(got_todo.done());
    }

    #[tokio::test]
    async fn delete() {
        let repo = TodoMemoryRepository::new();
        let service = TodoService::new(Box::new(repo));

        let created_todo = service
            .create(CreateTodo {
                name: "some".to_string(),
            })
            .await
            .unwrap();

        let got_todo = service.get(created_todo.id()).await.unwrap();
        assert_eq!(got_todo.id(), created_todo.id());

        service.delete(created_todo.id()).await.unwrap();

        match service.get(created_todo.id()).await {
            Err(TodoError::NotFound(_)) => {}
            _ => {
                panic!("expected NotFound error")
            }
        }
    }
}
