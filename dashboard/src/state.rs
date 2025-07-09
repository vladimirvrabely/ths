use sqlx::Error;

use crate::sqlite::DbPool;

#[derive(Clone, Debug)]
pub struct SharedState(pub DbPool);

impl SharedState {
    pub async fn new(db_path: &str) -> Result<Self, Error> {
        Ok(Self(DbPool::new(db_path).await?))
    }
}
