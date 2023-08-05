use chrono::{DateTime, Utc};
use crate::domain::id_generator::IDGenerator;

#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub status: NoteStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoteStatus {
    UNKNOWN,
    PUBLISHED,
    DELETED,
}

impl Note {
    pub fn new(user_id: &String, content: &String) -> Note {
        let id = IDGenerator::generate(12);
        let now = Utc::now();

        Note {
            id,
            user_id: user_id.clone(),
            content: content.clone(),
            status: NoteStatus::PUBLISHED,
            created_at: now.clone(),
            updated_at: now.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::domain::note::note::{Note, NoteStatus};

    #[test]
    fn test_new_note() {
        let note = Note::new(&"abcd1234".to_string(), &"Hello, world!".to_string());

        assert_ne!(note.id, "");
        assert_eq!(note.user_id, "abcd1234");
        assert_eq!(note.status, NoteStatus::PUBLISHED);
    }
}
