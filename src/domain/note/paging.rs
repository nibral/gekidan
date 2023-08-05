use crate::domain::note::note::Note;

pub struct NotesPage {
    pub notes: Vec<Note>,
    pub total: u64,
}

const DEFAULT_OFFSET: u64 = 0;
const DEFAULT_LIMIT: u64 = 10;

pub struct NotesPagingParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl NotesPagingParams {
    pub fn offset(&self) -> u64 {
        self.offset.unwrap_or(DEFAULT_OFFSET)
    }

    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(DEFAULT_LIMIT)
    }
}
