pub mod error;

use crate::domain::todo::error::TodoError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Интерфейс репозитория (хранилища) задач
#[async_trait]
pub trait TodoRepository {
    /// Создать задачу
    async fn create(&self, todo: Todo) -> Result<(), TodoError>;
    /// Получить все задачи
    async fn get_all(&self) -> Result<Vec<Todo>, TodoError>;
    /// Получить задачу по id
    async fn get_by_id(&self, id: Uuid) -> Result<Todo, TodoError>;
    /// Обновить задачу
    async fn save(&self, todo: Todo) -> Result<(), TodoError>;
    /// Удалить задачу
    async fn delete(&self, id: Uuid) -> Result<(), TodoError>;
}

/// Задача
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Todo {
    /// ID
    pub id: Uuid,
    /// Имя
    pub name: String,
    /// Дата создания
    pub created: DateTime<Utc>,
    /// Выполнено
    pub done: bool,
}

impl Todo {
    pub fn new(name: String) -> Result<Self, TodoError> {
        // Валидация полей
        if name.is_empty() {
            return Err(TodoError::NameIsEmpty);
        };

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            created: Utc::now(),
            done: false,
        })
    }

    /// Пометить задачу как завершенную
    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::todo::error::TodoError;
    use crate::domain::todo::Todo;

    #[test]
    fn with_empty_name() {
        match Todo::new("".into()) {
            Err(TodoError::NameIsEmpty) => {}
            _ => {
                panic!("expected NameIsEmpty error")
            }
        };
    }

    #[test]
    fn done() {
        let mut todo = Todo::new("some".into()).unwrap();
        todo.mark_done();
        assert!(todo.done);
    }
}
