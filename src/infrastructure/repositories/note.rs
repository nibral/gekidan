use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbConn, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::*;
use crate::domain::error::{CommonError, CommonErrorCode};
use crate::domain::note::note::{Note, NoteStatus};
use crate::domain::note::note_repository::NoteRepository;
use crate::domain::note::paging::{NotesPage, NotesPagingParams};
use crate::infrastructure::databases::converters::note::restore;
use crate::infrastructure::databases::entities::note;

pub struct NoteSeaORMRepository {
    db_conn: DbConn,
}

impl NoteSeaORMRepository {
    pub fn new(db_conn: DbConn) -> Self {
        NoteSeaORMRepository {
            db_conn
        }
    }
}

#[async_trait]
impl NoteRepository for NoteSeaORMRepository {
    async fn add(&self, new_note: &Note) -> Result<(), CommonError> {
        match note::ActiveModel::from(new_note).insert(&self.db_conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to insert note: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }
    }

    async fn list(&self, user_id: &String, paging_params: &NotesPagingParams) -> Result<NotesPage, CommonError> {
        let published: i32 = NoteStatus::PUBLISHED.into();

        // total count
        let total = note::Entity::find()
            .filter(note::Column::Status.eq(published))
            .count(&self.db_conn)
            .await;
        let total = match total {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to get num of notes: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };

        // select published notes
        let result = note::Entity::find()
            .filter(
                Condition::all()
                    .add(note::Column::UserId.eq(user_id))
                    .add(note::Column::Status.eq(published))
            )
            .order_by_desc(note::Column::CreatedAt)
            .offset(paging_params.offset())
            .limit(paging_params.limit())
            .all(&self.db_conn)
            .await;
        let notes = match result {
            Ok(l) => l.iter()
                .map(|n| -> Note {
                    restore(n)
                })
                .collect(),
            Err(e) => {
                log::error!("Failed to list notes: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };
        Ok(NotesPage {
            total,
            notes,
        })
    }

    async fn get(&self, user_id: &String, note_id: &String) -> Result<Note, CommonError> {
        let published: i32 = NoteStatus::PUBLISHED.into();
        let note = note::Entity::find()
            .filter(
                Condition::all()
                    .add(note::Column::Id.eq(note_id))
                    .add(note::Column::UserId.eq(user_id))
                    .add(note::Column::Status.eq(published))
            )
            .one(&self.db_conn)
            .await;
        match note {
            Ok(r) => match r {
                Some(n) => Ok(restore(&n)),
                None => return Err(CommonError::new(CommonErrorCode::NoteDoesNotExists))
            }
            Err(e) => {
                log::error!("Failed to get note: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }
    }

    async fn update(&self, note: &Note) -> Result<(), CommonError> {
        let target = match note::Entity::find_by_id(&note.id).one(&self.db_conn).await {
            Ok(r) => match r {
                Some(t) => t,
                None => {
                    log::error!("Specified note does not exists");
                    return Err(CommonError::new(CommonErrorCode::UnexpectedError));
                }
            }
            Err(e) => {
                log::error!("Failed to get note: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        };
        let mut target: note::ActiveModel = target.into();

        target.status = Set(note.status.into());
        target.updated_at = Set((&note.updated_at).to_rfc3339());

        match target.update(&self.db_conn).await {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to update note: {}", e.to_string());
                return Err(CommonError::new(CommonErrorCode::DBError));
            }
        }
    }
}
