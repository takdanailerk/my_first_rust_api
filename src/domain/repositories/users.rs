use anyhow::Result;
use axum::async_trait;

use crate::domain::entities::users::{RegisterUserEntity, UserEntity};

#[async_trait]
pub trait UserRepository {
    
    async fn register (&self, register_user_model: RegisterUserEntity) -> Result<i32>;
    async fn find_by_username (&self, username: String) -> Result<UserEntity>;

}