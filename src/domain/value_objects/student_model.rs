use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::domain::entities::students::RegisterStudentEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentModel {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterStudentModel {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: i32,
}

impl RegisterStudentModel {
    pub fn to_entity (&self) -> RegisterStudentEntity {
        RegisterStudentEntity {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            age: self.age,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc()
        }
    }
}