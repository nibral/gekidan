use async_trait::async_trait;
use crate::domain::error::CommonError;
use crate::domain::note::note::Note;

#[async_trait]
pub trait NoteRepository: Sync + Send {
    async fn add(&self, new_note: &Note) -> Result<(), CommonError>;
    async fn list(&self, user_id: &String) -> Result<Vec<Note>, CommonError>;
    async fn get(&self, user_id: &String, note_id: &String) -> Result<Note, CommonError>;
    async fn update(&self, note: &Note) -> Result<(), CommonError>;
}
