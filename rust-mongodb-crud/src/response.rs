use chrono::{DateTime, Utc};
use serde::Serialize;

/// Represents a generic response structure.
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,  // Status of the response.
    pub message: String, // Message in the response.
}

/// Represents a response structure for a single note.
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct NoteResponse {
    pub id: String,               // Unique identifier for the note.
    pub title: String,            // Title of the note.
    pub content: String,          // Content of the note.
    pub category: String,         // Category of the note.
    pub published: bool,          // Indicates if the note is published or not.
    pub createdAt: DateTime<Utc>, // Date and time when the note was created.
    pub updatedAt: DateTime<Utc>, // Date and time when the note was last updated.
}

/// Represents the data part of a note response.
#[derive(Serialize, Debug)]
pub struct NoteData {
    pub note: NoteResponse, // Contains the note details in NoteResponse format.
}

/// Represents a response structure for a single note, including status and data.
#[derive(Debug, Serialize)]
pub struct SingleNoteResponse {
    pub status: String, // Status of the response.
    pub data: NoteData, // Data part of the response containing note details.
}

/// Represents a response structure for a list of notes.
#[derive(Debug, Serialize)]
pub struct NoteListResponse {
    pub status: String,           // Status of the response.
    pub results: usize,           // Number of notes in the response.
    pub notes: Vec<NoteResponse>, // List of NoteResponse objects.
}
