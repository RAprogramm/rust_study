use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

/// Represents a model for a Note.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoteModel {
    #[serde(rename = "_id")]
    pub id: ObjectId, // Unique identifier for the note.
    pub title: String,            // Title of the note.
    pub content: String,          // Content of the note.
    pub category: Option<String>, // Optional category of the note.
    pub published: Option<Bool>,  // Optional publication status of the note.
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>, // Date and time when the note was created.
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>, // Date and time when the note was last updated.
}
