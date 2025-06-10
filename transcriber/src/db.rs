use anyhow::Result;
use sqlx::PgPool;

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

    pub async fn update_task_transcript(&self, task: Task, transcript: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE tasks SET transcript=$1 WHERE id=$2",
            transcript,
            &task.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ONLY USED FOR TESTING
    pub async fn create_task(&self, url: &str) -> Result<i32> {
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
