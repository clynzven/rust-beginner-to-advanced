use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Serialize, sqlx::FromRow)]
pub struct Note {
    id: i64,
    title: String,
    body: String,
    done: bool,
}

#[derive(Deserialize)]
pub struct NewNote {
    title: String,
    #[serde(default)]
    body: String,
}

#[derive(Deserialize)]
pub struct UpdateNote {
    title: Option<String>,
    body: Option<String>,
    done: Option<bool>,
}

pub async fn list(State(db): State<PgPool>) -> Result<Json<Vec<Note>>, AppError> {
    let notes = sqlx::query_as::<_, Note>("SELECT id, title, body, done FROM notes ORDER BY id")
        .fetch_all(&db)
        .await?;
    Ok(Json(notes))
}

pub async fn create(State(db): State<PgPool>, Json(input): Json<NewNote>)
    -> Result<(StatusCode, Json<Note>), AppError>
{
    if input.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty"));
    }
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (title, body) VALUES ($1, $2) RETURNING id, title, body, done")
        .bind(input.title.trim())
        .bind(input.body)
        .fetch_one(&db)
        .await?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get(State(db): State<PgPool>, Path(id): Path<i64>) -> Result<Json<Note>, AppError> {
    sqlx::query_as::<_, Note>("SELECT id, title, body, done FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound(id))
}

pub async fn update(State(db): State<PgPool>, Path(id): Path<i64>, Json(input): Json<UpdateNote>)
    -> Result<Json<Note>, AppError>
{
    sqlx::query_as::<_, Note>(
        "UPDATE notes SET title = COALESCE($2, title), body = COALESCE($3, body),
                          done = COALESCE($4, done)
         WHERE id = $1 RETURNING id, title, body, done")
        .bind(id)
        .bind(input.title)
        .bind(input.body)
        .bind(input.done)
        .fetch_optional(&db)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound(id))
}

pub async fn delete(State(db): State<PgPool>, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM notes WHERE id = $1").bind(id).execute(&db).await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(id));
    }
    Ok(StatusCode::NO_CONTENT)
}
