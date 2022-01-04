use async_trait::async_trait;
use chrono::Utc;
use sqlx::Postgres;
use tracing::instrument;
use uuid::Uuid;

use crate::{model::user::User, persistence::repository::RepositoryError};

use super::repository::{Repository, RepositoryResult};

pub struct PostgresRepository {
    pool: sqlx::PgPool,
}

impl PostgresRepository {
    pub async fn from_env() -> sqlx::Result<Self> {
        let conn_str =
            std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

        let pool = sqlx::Pool::<Postgres>::connect(&conn_str).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    #[instrument(skip(self))]
    async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            "SELECT id, name, birth_date, custom_data, created_at, updated_at FROM users WHERE id = $1"
        )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::InvalidId
        })
    }

    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (name, birth_date, custom_data)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(&user.name)
        .bind(&user.birth_date)
        .bind(&user.custom_data)
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::AlreadyExists
        })
    }

    async fn update_user(&self, user: &User) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET custom_data = $1, updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
        )
        .bind(&user.custom_data)
        .bind(Utc::now())
        .bind(&user.id.unwrap())
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::DoesNotExist
        })
    }

    async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid> {
        let result = sqlx::query_as::<_, User>(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        result.map(|u| u.id.unwrap()).map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::DoesNotExist
        })
    }
}
