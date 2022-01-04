use std::sync::RwLock;

use async_trait::async_trait;
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::model::user::User;

use super::repository::{Repository, RepositoryError, RepositoryResult};

pub struct MemoryRepository {
    users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    #[instrument(skip(self))]
    async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User> {
        let users = self.users.read()?;

        users
            .iter()
            .find(|u| &u.id.unwrap() == user_id)
            .cloned()
            .ok_or_else(|| RepositoryError::InvalidId)
    }

    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> RepositoryResult<User> {
        if self.get_user(&user.id.unwrap()).await.is_ok() {
            return Err(RepositoryError::AlreadyExists);
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.push(new_user.clone());
        Ok(new_user)
    }

    #[instrument(skip(self))]
    async fn update_user(&self, user: &User) -> RepositoryResult<User> {
        if let Ok(old_user) = self.get_user(&user.id.unwrap()).await {
            let mut updated_user = user.to_owned();
            updated_user.created_at = old_user.created_at;
            updated_user.updated_at = Some(Utc::now());
            let mut users = self.users.write()?;
            users.retain(|x| x.id != user.id);
            users.push(updated_user.clone());
            Ok(updated_user)
        } else {
            Err(RepositoryError::DoesNotExist)
        }
    }

    #[instrument(skip(self))]
    async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid> {
        let mut users = self.users.write()?;
        users.retain(|x| &x.id.unwrap() != user_id);
        Ok(user_id.to_owned())
    }
}
