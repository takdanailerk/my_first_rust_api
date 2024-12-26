use std::{collections::HashMap, sync::Arc};
use anyhow::{Ok, Result};

use crate::domain::{repositories::students::StudentRepository, value_objects::student_model::{RegisterStudentModel, StudentModel}};


pub struct StudentUseCase <T>
where
    T: StudentRepository + Send + Sync
{
    pub student_repository: Arc<T>
}

impl <T> StudentUseCase<T>
where
    T: StudentRepository + Send + Sync
{

    pub fn new (student_repository: Arc<T>) -> Self {
        Self {
            student_repository
        }
    }

    pub async fn register (
        &self,
        register_student_model: RegisterStudentModel
    ) -> Result<StudentModel> {

        match self.student_repository.find_by_email(&register_student_model.email).await {
            std::result::Result::Ok(_) => return Err(anyhow::anyhow!("E001")),
            Err(error) => match error.to_string().as_str() {
                "Record not found" => (),
                _ => return Err(error)
            }
        }

        let register_entity = register_student_model.to_entity();

        let student = self.student_repository.register(register_entity).await?;

        let student_model = student.to_model();

        Ok(student_model)
    }

    pub async fn find_by_first_name (
        &self,
        params: &HashMap<String, String>
    ) -> Result<Vec<StudentModel>> {

        let first_name = match params.get("first_name") {
            Some(first_name_param) => first_name_param.to_string(),
            None => return Err(anyhow::anyhow!("E001")),
        };

        let student_entities = self.student_repository.find_by_first_name(first_name.as_str()).await?;

        let student_models = student_entities
            .into_iter().map(|student_entity| student_entity.to_model()).collect();

        Ok(student_models)

    }

    pub async fn get_all (&self) -> Result<Vec<StudentModel>> {
        let results = self.student_repository.get_all().await?;

        let mut students = Vec::new();

        for result in results.into_iter() {
            students.push(result.to_model());
        }

        Ok(students)
    }

    pub async fn find_by_id (&self, id: &i32) -> Result<StudentModel> {
        let result = self.student_repository.find_by_id(id).await;

        match result {
            std::result::Result::Ok(_) => (),
            Err(error) => match error.to_string().as_str() {
                "Record not found" => return Err(anyhow::anyhow!("E001")),
                _ => return Err(error)
            }
        };

        let student_model = result?.to_model();

        Ok(student_model)
    }

}