use std::sync::Arc;

use axum::async_trait;
use anyhow::Result;

use crate::{domain::{entities::users::{RegisterUserEntity, UserEntity}, repositories::users::UserRepository, value_objects::user_model::RegisterUserModel}, infrastructure::postgres::postgres_connection::PgPoolSquad};


pub struct UserPostgres {
    database_pool: Arc<PgPoolSquad>
}

impl UserPostgres {

    pub fn new (database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }

}

#[async_trait]
impl UserRepository for UserPostgres {

    async fn register (&self, register_user_model: RegisterUserEntity) -> Result<i32> {
        unimplemented!()
    }

    async fn find_by_username (&self, username: String) -> Result<UserEntity> {
        unimplemented!()
    }

}