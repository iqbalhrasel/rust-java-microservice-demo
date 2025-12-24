use std::fmt::format;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, prelude::FromRow};

use crate::errors::Error;

#[derive(Debug, FromRow, Serialize)]
struct Student {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    email: String,
    school_id: i32,
}

#[derive(Debug, Serialize)]
pub struct StudentDto {
    pub id: i32,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,

    pub email: String,

    #[serde(rename = "schoolId")]
    pub school_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchoolDto {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentSchoolDto {
    pub id: u64,

    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,

    pub email: String,
    pub school: SchoolDto,
}

#[derive(Debug, Deserialize)]
pub struct AddStudentDRequest {
    #[serde(rename = "firstName")]
    first_name: String,

    #[serde(rename = "lastName")]
    last_name: String,

    email: String,

    #[serde(rename = "schoolId")]
    school_id: i32,
}

#[derive(Clone)]
pub struct StudentService {
    db_pool: MySqlPool,
}

impl StudentService {
    pub fn new(db_pool: MySqlPool) -> Self {
        return Self { db_pool };
    }
}

//crud
impl StudentService {
    pub async fn create_student(&self, request: AddStudentDRequest) -> Result<StudentDto, Error> {
        let query_result = sqlx::query(
            r#"INSERT INTO students (first_name, last_name, email, school_id) VALUES (?,?,?,?)"#,
        )
        .bind(&request.first_name)
        .bind(&request.last_name)
        .bind(&request.email)
        .bind(&request.school_id)
        .execute(&self.db_pool)
        .await
        .map_err(|_| Error::InternalServerError)?;

        let dto = StudentDto {
            id: query_result.last_insert_id() as i32,
            first_name: request.first_name,
            last_name: request.last_name,
            email: request.email,
            school_id: request.school_id,
        };

        return Ok(dto);
    }

    pub async fn get_students(&self, school_id: i32) -> Result<Vec<StudentDto>, Error> {
        let students: Vec<Student> = sqlx::query_as(r#"SELECT * FROM students WHERE school_id=?"#)
            .bind(school_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| {
                println!("err: {:?}", e);
                return Error::InternalServerError;
            })?;

        let dtos = students
            .iter()
            .map(|s| StudentDto {
                id: s.id,
                first_name: s.first_name.to_string(),
                last_name: s.last_name.to_owned().unwrap_or("".to_string()),
                email: s.email.to_string(),
                school_id: s.school_id,
            })
            .collect();

        return Ok(dtos);
    }

    pub async fn get_student_with_school(
        &self,
        student_id: u64,
    ) -> Result<StudentSchoolDto, Error> {
        let student: Option<Student> = sqlx::query_as(r#"SELECT * FROM students WHERE id=?"#)
            .bind(student_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| {
                println!("err: {:?}", e);
                return Error::InternalServerError;
            })?;

        let student = match student {
            None => {
                return Err(Error::StudentNotFoundError);
            }
            Some(s) => s,
        };

        // let client = Client::new().get(format!(
        //     "http://localhost:8080/schools/{}",
        //     student.school_id,
        // ));
        // let school_dto: SchoolDto = client.send().await.unwrap().json().await.unwrap();

        let res = reqwest::get(format!(
            "http://localhost:8080/schools/{}",
            student.school_id,
        ))
        .await
        .map_err(|_| Error::InternalServerError)?;

        let school_dto = res
            .json::<SchoolDto>()
            .await
            .map_err(|_| Error::SchoolNotFoundError)?;

        let dto = StudentSchoolDto {
            id: student.id as u64,
            first_name: student.first_name,
            last_name: student.last_name.unwrap_or("".to_string()),
            email: student.email,
            school: school_dto,
        };

        return Ok(dto);
    }
}
