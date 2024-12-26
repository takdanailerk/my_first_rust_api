use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{domain::value_objects::student_model::StudentModel, infrastructure::postgres::schema::students};

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = students)]
pub struct StudentEntity {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl StudentEntity {

    pub fn to_model (&self) -> StudentModel {
        StudentModel {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at
        }
    }

}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = students)]
pub struct RegisterStudentEntity {
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}