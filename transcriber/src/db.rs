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
        sqlx::query("UPDATE tasks SET status=$1 WHERE id=$2")
            .bind(task_status)
            .bind(&task.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ONLY USED FOR TESTING
    // pub async fn create_task(&self, task: Task) -> Result<()> {}
}
