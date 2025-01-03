use crate::domain::entities::students::{RegisterStudentEntity, StudentEntity};
use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait StudentRepository {
    
    async fn register (&self, register_student_entity: RegisterStudentEntity) -> Result<StudentEntity>;
    async fn find_by_first_name (&self, first_name: &str) -> Result<Vec<StudentEntity>>;
    async fn get_all (&self) -> Result<Vec<StudentEntity>>;
    async fn find_by_id (&self, id: &i32) -> Result<StudentEntity>;
    async fn find_by_email (&self, email: &str) -> Result<StudentEntity>;

}