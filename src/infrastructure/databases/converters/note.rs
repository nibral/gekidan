use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use crate::domain::note::note::{Note, NoteStatus};
use crate::infrastructure::databases::entities::note;

impl From<&Note> for note::ActiveModel {
    fn from(note: &Note) -> Self {
        note::ActiveModel {
            id: Set(note.id.clone()),
            user_id: Set(note.user_id.clone()),
            content: Set(note.content.clone()),
            status: Set(note.status.into()),
            created_at: Set(note.created_at.to_rfc3339()),
            updated_at: Set(note.updated_at.to_rfc3339()),
        }
    }
}

pub fn restore(note: &note::Model) -> Note {
    Note {
        id: note.id.clone(),
        user_id: note.user_id.clone(),
        content: note.content.clone(),
        status: note.status.into(),
        created_at: DateTime::parse_from_rfc3339(&note.created_at).unwrap().with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&note.updated_at).unwrap().with_timezone(&Utc),
    }
}

impl From<NoteStatus> for i32 {
    fn from(value: NoteStatus) -> Self {
        match value {
            NoteStatus::PUBLISHED => 1,
            NoteStatus::DELETED => 2,
            NoteStatus::UNKNOWN => 0,
        }
    }
}

impl From<i32> for NoteStatus {
    fn from(value: i32) -> Self {
        match value {
            1 => NoteStatus::PUBLISHED,
            2 => NoteStatus::DELETED,
            _ => NoteStatus::UNKNOWN,
        }
    }
}
