use std::sync::Arc;

use axum::async_trait;
use anyhow::{Ok, Result};
use diesel::{dsl::insert_into, prelude::*};

use crate::{domain::{entities::students::{RegisterStudentEntity, StudentEntity}, repositories::students::StudentRepository}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::students}};

pub struct StudentPostgres {
    database_pool: Arc<PgPoolSquad>
}

impl StudentPostgres {

    pub fn new (database_pool: Arc<PgPoolSquad>) -> Self {
        Self { database_pool }
    }

}

#[async_trait]
impl StudentRepository for StudentPostgres {
    
    async fn register (&self, register_student_entity: RegisterStudentEntity) -> Result<StudentEntity> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let result = insert_into(students::table)
            .values(&register_student_entity)
            .returning(StudentEntity::as_select())
            .get_result::<StudentEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_first_name (&self, first_name: &str) -> Result<Vec<StudentEntity>> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let results = students::table
            .filter(students::first_name.like(format!("%{}%",first_name)))
            .select(StudentEntity::as_select())
            .get_results::<StudentEntity>(&mut connection)?;

        Ok(results)
    }

    async fn get_all (&self) -> Result<Vec<StudentEntity>> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let results = students::table
            .select(StudentEntity::as_select())
            .order_by(students::created_at.desc())
            .load::<StudentEntity>(&mut connection)?;

        Ok(results)
    }

    async fn find_by_id (&self, id: &i32) -> Result<StudentEntity> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let result = students::table
            .filter(students::id.eq(id))
            .select(StudentEntity::as_select())
            .get_result::<StudentEntity>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_email (&self, email: &str) -> Result<StudentEntity> {
        let mut connection = Arc::clone(&self.database_pool).get()?;

        let result = students::table
            .filter(students::email.eq(email))
            .select(StudentEntity::as_select())
            .get_result::<StudentEntity>(&mut connection)?;

        Ok(result)
    }

}