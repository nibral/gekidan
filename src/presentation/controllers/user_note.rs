use std::sync::Arc;
use actix_web::web::{Data, Json, Path};
use serde::{Deserialize, Serialize};
use crate::app::container::Container;
use crate::domain::note::note::Note;
use crate::presentation::errors::api::ApiError;

pub async fn create_user_note(
    container: Data<Arc<Container>>,
    params: Path<String>,
    post_data: Json<CreateUserNoteRequest>,
) -> Result<Json<UserNoteResponse>, ApiError> {
    let usecase = &container.user_note_usecase;
    let data = &post_data.into_inner();
    let note = usecase.create(&params.into_inner(), &data.content).await?;
    Ok(Json(note.into()))
}

pub async fn list_notes(
    container: Data<Arc<Container>>,
    params: Path<String>,
) -> Result<Json<UserNoteListResponse>, ApiError> {
    let usecase = &container.user_note_usecase;
    let notes = usecase.list(&params.into_inner()).await?;
    Ok(Json(UserNoteListResponse::from(notes)))
}

#[derive(Serialize, Deserialize)]
pub struct UserNoteResponse {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub created_at: String,
}

impl From<Note> for UserNoteResponse {
    fn from(value: Note) -> Self {
        UserNoteResponse {
            id: value.id,
            user_id: value.user_id,
            content: value.content,
            created_at: value.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserNoteRequest {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserNoteListResponse {
    pub notes: Vec<UserNoteResponse>,
}

impl From<Vec<Note>> for UserNoteListResponse {
    fn from(value: Vec<Note>) -> Self {
        UserNoteListResponse {
            notes: value.iter().map(|n| n.clone().into()).collect(),
        }
    }
}
