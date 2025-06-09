use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub url: String,
}

pub enum TaskStatus {
    InProgress,
}

impl TaskStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            TaskStatus::InProgress => "inProgress",
        }
    }
}
