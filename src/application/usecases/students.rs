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
        let register_entity = register_student_model.to_entity();

        let student = self.student_repository.register(register_entity).await?;

        let student_model = student.to_model();

        Ok(student_model)
    }

    pub async fn find_by_first_name (
        &self,
        params: HashMap<String, String>
    ) -> Result<StudentModel> {

        let mut first_name: String;

        if let Some(first_name_param) = params.get("first_name") {
            first_name = first_name_param.to_string();
        } else {
            return Err(anyhow::anyhow!("E001"));
        }

        let student_entity = self.student_repository.find_by_first_name(first_name).await?;

        let student_model = student_entity.to_model();

        Ok(student_model)

    }

    pub async fn get_all (&self) -> Result<Vec<StudentModel>> {
        let results = self.student_repository.get_all().await?;

        let mut students = Vec::new();

        for result in results.into_iter() {
            students.push(result.to_model());
        }

        Ok(students)
    }

}