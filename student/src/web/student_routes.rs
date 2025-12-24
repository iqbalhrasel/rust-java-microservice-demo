use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    errors::Error,
    model::{AddStudentDRequest, StudentService},
};

pub fn student_routes(service: StudentService) -> Router {
    return Router::new()
        .route("/students", post(create_student))
        .route("/students/school/{school_id}", get(get_students))
        .route("/students/{student_id}", get(get_student_with_school))
        .with_state(service);
}

async fn create_student(
    State(service): State<StudentService>,
    Json(request): Json<AddStudentDRequest>,
) -> Result<impl IntoResponse, Error> {
    let dto = service.create_student(request).await?;
    return Ok((StatusCode::OK, Json(dto)));
}

async fn get_students(
    State(service): State<StudentService>,
    Path(school_id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let dtos = service.get_students(school_id).await?;
    return Ok((StatusCode::OK, Json(dtos)));
}

async fn get_student_with_school(
    State(service): State<StudentService>,
    Path(student_id): Path<u64>,
) -> Result<impl IntoResponse, Error> {
    let dto = service.get_student_with_school(student_id).await?;
    return Ok((StatusCode::OK, Json(dto)));
}
