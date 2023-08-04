use std::sync::Arc;
use crate::domain::error::CommonError;
use crate::domain::note::note::{Note, NoteStatus};
use crate::domain::note::note_repository::NoteRepository;

pub struct UserNoteUseCase {
    note_repository: Arc<dyn NoteRepository>,
}

impl UserNoteUseCase {
    pub fn new(
        note_repository: Arc<dyn NoteRepository>,
    ) -> Self {
        UserNoteUseCase {
            note_repository,
        }
    }

    pub async fn create(&self, user_id: &String, content: &String) -> Result<Note, CommonError> {
        let new_note = Note::new(user_id, content);
        self.note_repository
            .add(&new_note)
            .await
            .map(|_| new_note)
    }

    pub async fn list(&self, user_id: &String) -> Result<Vec<Note>, CommonError> {
        self.note_repository
            .list(user_id)
            .await
    }

    pub async fn get(&self, user_id: &String, note_id: &String) -> Result<Note, CommonError> {
        self.note_repository
            .get(user_id, note_id)
            .await
    }

    pub async fn delete(&self, user_id: &String, note_id: &String) -> Result<(), CommonError> {
        let mut note = match self.note_repository.get(user_id, note_id).await {
            Ok(n) => n,
            Err(e) => return Err(e)
        };

        note.status = NoteStatus::DELETED;
        match self.note_repository.update(&note).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
