use std::sync::Arc;
use sea_orm::Database;
use crate::domain::activity_pub::activity_pub_service::ActivityPubService;
use crate::domain::app_config::AppConfig;
use crate::domain::note::note_repository::NoteRepository;
use crate::domain::user::user_repository::UserRepository;
use crate::domain::user::user_service::UserService;
use crate::infrastructure::config::env_file::load_app_config;
use crate::infrastructure::repositories::note::NoteSeaORMRepository;
use crate::infrastructure::repositories::user::UserSeaORMRepository;
use crate::usecase::activity_pub::ActivityPubUseCase;
use crate::usecase::user_management::UserManagementUseCase;
use crate::usecase::user_note::UserNoteUseCase;

pub struct Container {
    pub app_config: Arc<AppConfig>,
    pub activity_pub_usecase: Arc<ActivityPubUseCase>,
    pub user_management_usecase: Arc<UserManagementUseCase>,
    pub user_note_usecase: Arc<UserNoteUseCase>,
}

impl Container {
    pub async fn new() -> Self {
        let app_config = Arc::new(load_app_config().await);
        let db_conn = Database::connect(&app_config.database_url)
            .await
            .expect("Failed to connect database");

        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserSeaORMRepository::new(db_conn.clone())
        );
        let note_repository: Arc<dyn NoteRepository> = Arc::new(
            NoteSeaORMRepository::new(db_conn.clone())
        );

        let activity_pub_service = Arc::new(
            ActivityPubService::new(user_repository.clone()),
        );
        let activity_pub_usecase = Arc::new(
            ActivityPubUseCase::new(
                app_config.clone(),
                activity_pub_service,
                user_repository.clone(),
            ),
        );

        let user_service: Arc<UserService> = Arc::new(
            UserService::new(user_repository.clone())
        );
        let user_management_usecase = Arc::new(
            UserManagementUseCase::new(user_repository, user_service)
        );

        let user_note_usecase = Arc::new(
            UserNoteUseCase::new(note_repository)
        );

        Container {
            app_config,
            activity_pub_usecase,
            user_management_usecase,
            user_note_usecase,
        }
    }
}
