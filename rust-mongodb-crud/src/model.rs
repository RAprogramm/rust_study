use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoteModel {
    /// Unique identifier for the note.
    #[serde(rename = "_id")]
    pub id: ObjectId,
    /// Title of the note.
    pub title: String,
    /// Content of the note.
    pub content: String,
    /// Optional category of the note.
    pub category: Option<String>,
    /// Optional publication status of the note.
    pub published: Option<Bool>,
    /// Date and time when the note was created.
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    /// Date and time when the note was last updated.
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}
