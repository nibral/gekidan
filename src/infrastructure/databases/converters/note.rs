use chrono::{DateTime, Utc};
use sea_orm::ActiveValue::Set;
use crate::domain::note::note::{Note, NoteStatus};
use crate::infrastructure::databases::entities::note;

impl From<&Note> for note::ActiveModel {
    fn from(note: &Note) -> Self {
        let status = match note.status {
            NoteStatus::PUBLISHED => 1,
            NoteStatus::DELETED => 2,
            NoteStatus::UNKNOWN => 0,
        };

        note::ActiveModel {
            id: Set(note.id.clone()),
            user_id: Set(note.user_id.clone()),
            content: Set(note.content.clone()),
            status: Set(status),
            created_at: Set(note.created_at.to_rfc3339()),
            updated_at: Set(note.updated_at.to_rfc3339()),
        }
    }
}

pub fn restore(note: &note::Model) -> Note {
    let status = match note.status {
        1 => NoteStatus::PUBLISHED,
        2 => NoteStatus::DELETED,
        _ => NoteStatus::UNKNOWN,
    };

    Note {
        id: note.id.clone(),
        user_id: note.user_id.clone(),
        content: note.content.clone(),
        status,
        created_at: DateTime::parse_from_rfc3339(&note.created_at).unwrap().with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&note.updated_at).unwrap().with_timezone(&Utc),
    }
}
