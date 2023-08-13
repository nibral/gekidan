use std::sync::Arc;
use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
use crate::domain::app_config::AppConfig;
use crate::domain::error::CommonError;
use crate::domain::follower::follower_repository::FollowerRepository;
use crate::domain::note::note::{Note, NoteStatus};
use crate::domain::note::note_repository::NoteRepository;
use crate::domain::note::paging::{NotesPage, NotesPagingParams};
use crate::domain::user::user_repository::UserRepository;

pub struct UserNoteUseCase {
    app_config: Arc<AppConfig>,
    note_repository: Arc<dyn NoteRepository>,
    user_repository: Arc<dyn UserRepository>,
    follower_repository: Arc<dyn FollowerRepository>,
    activity_pub_service: Arc<ActivityPubService>,
}

impl UserNoteUseCase {
    pub fn new(
        app_config: Arc<AppConfig>,
        note_repository: Arc<dyn NoteRepository>,
        user_repository: Arc<dyn UserRepository>,
        follower_repository: Arc<dyn FollowerRepository>,
        activity_pub_service: Arc<ActivityPubService>,
    ) -> Self {
        UserNoteUseCase {
            app_config,
            note_repository,
            user_repository,
            follower_repository,
            activity_pub_service,
        }
    }

    pub async fn create(&self, user_id: &String, content: &String) -> Result<Note, CommonError> {
        let user = self.user_repository.get(user_id).await?;

        let new_note = Note::new(&user.id, content);
        self.note_repository.add(&new_note).await?;

        let recipients = self.follower_repository.list(&user.id).await?;
        self.activity_pub_service.send_note(&user, &new_note, recipients, &self.app_config.app_url).await?;

        Ok(new_note)
    }

    pub async fn list(&self, user_id: &String, paging_params: &NotesPagingParams) -> Result<NotesPage, CommonError> {
        self.note_repository
            .list(user_id, paging_params)
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
