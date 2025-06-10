use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub url: String,
    pub status: String,
}
impl Task {
    pub fn to_json_str(&self) -> String {
        json!({
            "id": self.id,
            "url": self.url,
            "status": self.status
        })
        .to_string()
    }
}

pub enum TaskStatus {
    NotStarted,
    InProgress,
    Error,
    Finished,
}

impl TaskStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            TaskStatus::NotStarted => "notStarted",
            TaskStatus::InProgress => "inProgress",
            TaskStatus::Error => "error",
            TaskStatus::Finished => "finished",
        }
    }
    pub fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}
