use serde::{Deserialize, Serialize};

/// Structure defining options for filtering notes.
#[derive(Debug, Deserialize)]
pub struct FilterOptions {
    /// The page number for pagination.
    pub page: Option<usize>,
    /// The maximum number of notes per page.
    pub limit: Option<usize>,
}

/// Structure defining parameters for note operations.
#[derive(Debug, Deserialize)]
pub struct ParamOptions {
    /// The ID of the note.
    pub id: String,
}

/// Schema for creating a new note.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNoteSchema {
    /// The title of the note.
    pub title: String,
    /// The content of the note.
    pub content: String,
    /// The category of the note, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Whether the note is published or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

/// Schema for updating an existing note.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNoteSchema {
    /// The updated title of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The updated content of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The updated category of the note, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Whether the note should be marked as published or unpublished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}
