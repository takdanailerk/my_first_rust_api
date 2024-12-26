use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{application::usecases::students::StudentUseCase, domain::{repositories::students::StudentRepository, value_objects::{response_model::ResponseObject, student_model::RegisterStudentModel}}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::students::StudentPostgres}};


pub fn routes (database_pool: Arc<PgPoolSquad>) -> Router {

    let student_repository = StudentPostgres::new(database_pool);
    let student_use_case = StudentUseCase::new(Arc::new(student_repository));

    Router::new()
        .route("/", post(register))
        .route("/find-by-first-name", get(find_by_first_name))
        .route("/", get(get_all))
        .with_state(Arc::new(student_use_case))

}

pub async fn register <T> (
    State(student_use_case): State<Arc<StudentUseCase<T>>>,
    Json(register_student_model): Json<RegisterStudentModel>
) -> impl IntoResponse
where 
    T: StudentRepository + Send + Sync
{
    match student_use_case.register(register_student_model).await {
        Ok(student) => ResponseObject {
            status_code: StatusCode::CREATED,
            success: true,
            message: String::from("Register student successfully!"),
            result: Some(student)
        },
        Err(error) => ResponseObject {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            success: false,
            message: error.to_string(),
            result: None
        }
    }
}

pub async fn find_by_first_name <T> (
    State(student_use_case): State<Arc<StudentUseCase<T>>>,
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse
where
    T: StudentRepository + Send + Sync
{
    match student_use_case.find_by_first_name(params).await {
        Ok(student) => ResponseObject {
            status_code: StatusCode::OK,
            success: true,
            message: String::from("Get student by first name successfully!"),
            result: Some(student)
        },
        Err(error) => match error.to_string().as_str() {
            "E001" => ResponseObject {
                status_code: StatusCode::BAD_REQUEST,
                success: false,
                message: String::from("This endpoint needs first name"),
                result: None
            },
            _ => ResponseObject {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                success: false,
                message: error.to_string(),
                result: None
            }
        }
    }
}

pub async fn get_all <T> (
    State(student_use_case): State<Arc<StudentUseCase<T>>>
) -> impl IntoResponse
where
    T: StudentRepository + Send + Sync
{
    match student_use_case.get_all().await {
        Ok(students) => ResponseObject {
            status_code: StatusCode::OK,
            success: true,
            message: String::from("Get all students successfully!"),
            result: Some(students)
        },
        Err(error) => ResponseObject {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            success: false,
            message: error.to_string(),
            result: None
        }
    }
}