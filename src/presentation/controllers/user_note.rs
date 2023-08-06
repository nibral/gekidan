use std::sync::Arc;
use actix_web::web::{Data, Json, Path, Query};
use serde::{Deserialize, Serialize};
use crate::app::container::Container;
use crate::domain::note::note::Note;
use crate::domain::note::paging::{NotesPage, NotesPagingParams};
use crate::presentation::errors::api::ApiError;
use crate::presentation::extractors::admin_claim::AdminClaim;

pub async fn create_user_note(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<String>,
    post_data: Json<CreateUserNoteRequest>,
) -> Result<Json<UserNoteResponse>, ApiError> {
    let usecase = &container.user_note_usecase;
    let data = &post_data.into_inner();
    let note = usecase.create(&params.into_inner(), &data.content).await?;
    Ok(Json(note.into()))
}

pub async fn list_user_notes(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<String>,
    queries: Query<UserNoteListQuery>,
) -> Result<Json<UserNoteListResponse>, ApiError> {
    let usecase = &container.user_note_usecase;
    let notes = usecase.list(&params.into_inner(), &queries.into_inner().into()).await?;
    Ok(Json(UserNoteListResponse::from(notes)))
}

pub async fn get_user_note(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<(String, String)>,
) -> Result<Json<UserNoteResponse>, ApiError> {
    let usecase = &container.user_note_usecase;
    let (user_id, note_id) = params.into_inner();
    let note = usecase.get(&user_id, &note_id).await?;
    Ok(Json(note.into()))
}

pub async fn delete_user_note(
    _: AdminClaim,
    container: Data<Arc<Container>>,
    params: Path<(String, String)>,
) -> Result<String, ApiError> {
    let usecase = &container.user_note_usecase;
    let (user_id, note_id) = params.into_inner();
    usecase.delete(&user_id, &note_id).await?;
    Ok("ok".to_string())
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
pub struct UserNoteListQuery {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl From<UserNoteListQuery> for NotesPagingParams {
    fn from(value: UserNoteListQuery) -> Self {
        NotesPagingParams {
            offset: value.offset,
            limit: value.limit,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserNoteListResponse {
    pub total: u64,
    pub notes: Vec<UserNoteResponse>,
}

impl From<NotesPage> for UserNoteListResponse {
    fn from(value: NotesPage) -> Self {
        UserNoteListResponse {
            total: value.total,
            notes: value.notes.iter().map(|n| n.clone().into()).collect(),
        }
    }
}
