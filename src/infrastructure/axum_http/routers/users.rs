use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{application::usecases::users::UserUseCase, domain::{repositories::users::UserRepository, value_objects::user_model::RegisterUserModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::users::UserPostgres}};


pub fn routes (database_pool: Arc<PgPoolSquad>) -> Router {

    let user_repository = UserPostgres::new(database_pool);
    let user_use_case = UserUseCase::new(Arc::new(user_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(user_use_case))

}

pub async fn register <T> (
    State(user_use_case): State<Arc<UserUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync
{
    unimplemented!()
}