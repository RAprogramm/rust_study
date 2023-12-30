use chrono::{DateTime, Utc};
use serde::Serialize;

/// Represents a generic response structure.
#[derive(Serialize)]
pub struct GenericResponse {
    /// Status of the response.
    pub status: String,
    /// Message in the response.
    pub message: String,
}

/// Represents a response structure for a single note.
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct NoteResponse {
    /// Unique identifier for the note.
    pub id: String,
    /// Title of the note.
    pub title: String,
    /// Content of the note.
    pub content: String,
    /// Category of the note.
    pub category: String,
    /// Indicates if the note is published or not.
    pub published: bool,
    /// Date and time when the note was created.
    pub createdAt: DateTime<Utc>,
    /// Date and time when the note was last updated.
    pub updatedAt: DateTime<Utc>,
}

/// Represents the data part of a note response.
#[derive(Serialize, Debug)]
pub struct NoteData {
    /// Contains the note details in NoteResponse format.
    pub note: NoteResponse,
}

/// Represents a response structure for a single note, including status and data.
#[derive(Debug, Serialize)]
pub struct SingleNoteResponse {
    /// Status of the response.
    pub status: String,
    /// Data part of the response containing note details.
    pub data: NoteData,
}

/// Represents a response structure for a list of notes.
#[derive(Debug, Serialize)]
pub struct NoteListResponse {
    /// Status of the response.
    pub status: String,
    /// Number of notes in the response.
    pub results: usize,
    /// List of NoteResponse objects.
    pub notes: Vec<NoteResponse>,
}
