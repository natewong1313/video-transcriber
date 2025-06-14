use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Task, TaskStatus};

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn update_task_status(&self, task: Task, status: TaskStatus) -> Result<()> {
        let task_status = status.to_str();
        sqlx::query!(
            "UPDATE tasks SET status=$1 WHERE id=$2",
            task_status,
            &task.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_task_transcript(&self, task: Task) -> Result<()> {
        sqlx::query!(
            "UPDATE tasks SET transcript=$1 WHERE id=$2",
            &task.transcript,
            &task.id
        )
        .execute(&self.pool)
        .await?;

        let row = serde_json::to_string(&task)?;
        // yes this is safe
        // postgres doesnt let u use uuid as chan name since it has - so we convert to _
        let formatted_id = str::replace(&task.id.to_string(), "-", "_");
        let query = format!("SELECT pg_notify('task_{}_done', $1)", formatted_id);
        // cant use query! since its dynamic but we dont need compile time checking for this
        sqlx::query(&query).bind(row).execute(&self.pool).await?;

        Ok(())
    }

    // ONLY USED FOR TESTING
    pub async fn create_task(&self, url: &str) -> Result<Uuid> {
        let res = sqlx::query!(
            "INSERT INTO tasks (url, status) VALUES ($1, $2) RETURNING id",
            url,
            TaskStatus::NotStarted.to_str()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(res.id)
    }
}
