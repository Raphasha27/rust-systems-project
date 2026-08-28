use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool: Arc::new(pool) })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
