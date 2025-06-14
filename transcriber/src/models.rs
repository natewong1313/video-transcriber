use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub url: String,
    pub status: String,
    #[serde(deserialize_with = "null_to_default")]
    pub transcript: String,
}
// hack since when we recieve a row notification transcript is null and that breaks stuff
fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let key = Option::<T>::deserialize(de)?;
    Ok(key.unwrap_or_default())
}
impl Task {
    pub fn to_json_str(&self) -> String {
        json!({
            "id": self.id,
            "url": self.url,
            "status": self.status,
            "transcript": self.transcript
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
