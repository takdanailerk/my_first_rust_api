use std::sync::Arc;
use anyhow::Result;

use crate::domain::{repositories::users::UserRepository, value_objects::user_model::RegisterUserModel};

pub struct UserUseCase <T>
where
    T: UserRepository + Send + Sync
{
    user_repository: Arc<T>
}

impl <T> UserUseCase<T>
where
    T: UserRepository + Send + Sync
{

    pub fn new (user_repository: Arc<T>) -> Self {
        Self {
            user_repository
        }
    }

    pub async fn register (
        &self,
        mut register_user_model: RegisterUserModel
    ) -> Result<i32> {
        unimplemented!()
    }

}