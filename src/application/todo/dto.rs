use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub name: String,
}
