use crate::response::{NoteData, NoteListResponse, NoteResponse, SingleNoteResponse};
use crate::{
    error::Error::*, model::NoteModel, schema::CreateNoteSchema, schema::UpdateNoteSchema, Result,
};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

/// Represents a structure to manage different MongoDB collections.
#[derive(Clone, Debug)]
pub struct DB {
    /// Collection for handling NoteModel data.
    pub note_collection: Collection<NoteModel>,

    /// Collection for handling generic BSON documents.
    pub collection: Collection<Document>,
}

impl DB {
    /// Initializes a connection to the database using the provided environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or if establishing a connection fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::DB;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = DB::init().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn init() -> Result<Self> {
        // Retrieve environment variable values for configuring the connection
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let database_name: String =
            std::env::var("MONGO_INITDB_DATABASE").expect("MONGO_INITDB_DATABASE must be set.");
        let mongodb_note_collection: String =
            std::env::var("MONGODB_NOTE_COLLECTION").expect("MONGODB_NOTE_COLLECTION must be set.");

        // Parse MongoDB client options from the URL and set the database name
        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        // Create a MongoDB client and access the database
        let client = Client::with_options(client_options).unwrap();
        let database = client.database(database_name.as_str());

        // Access note collections and a shared collection (if any)
        let note_collection = database.collection(mongodb_note_collection.as_str());
        let collection = database.collection::<Document>(mongodb_note_collection.as_str());

        println!("Database connected successfully");

        // Return an instance of the DB structure with the obtained collections
        Ok(Self {
            note_collection,
            collection,
        })
    }

    /// Fetches a list of notes based on provided pagination parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of notes to retrieve.
    /// * `page` - The specific page of notes to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails or encounters issues during retrieval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::DB;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = DB::init().await?;
    /// let notes = db.fetch_notes(10, 1).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_notes(&self, limit: i64, page: i64) -> Result<NoteListResponse> {
        // Define find options based on provided limit and page values
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        // Query the note collection and retrieve a cursor
        let mut cursor = self
            .note_collection
            .find(None, find_options)
            .await
            .map_err(MongoQueryError)?;

        // Initialize an empty vector to store note responses
        let mut json_result: Vec<NoteResponse> = Vec::new();

        // Iterate through the cursor and collect note responses
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_note(&doc.unwrap())?);
        }

        // Create a NoteListResponse based on the collected note responses
        let json_note_list = NoteListResponse {
            status: "success".to_string(),
            results: json_result.len(),
            notes: json_result,
        };

        Ok(json_note_list)
    }

    /// Creates a new note based on the provided data.
    ///
    /// # Arguments
    ///
    /// * `body` - The data representing the note to be created.
    ///
    /// # Errors
    ///
    /// Returns an error if the note creation fails due to various reasons like duplicate keys or other errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::{DB, CreateNoteSchema};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = DB::init().await?;
    /// let new_note = CreateNoteSchema {
    ///     title: "New Note".to_string(),
    ///     content: "This is a new note!".to_string(),
    ///     category: Some("General".to_string()),
    ///     published: None,
    /// };
    ///
    /// let created_note = db.create_note(&new_note).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_note(&self, body: &CreateNoteSchema) -> Result<Option<SingleNoteResponse>> {
        // Retrieve or set default values for 'published' and 'category'
        let published = body.published.to_owned().unwrap_or(false);
        let category = body.category.to_owned().unwrap_or("".to_string());

        // Serialize 'body' into BSON document
        let serialized_data = bson::to_bson(&body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();

        // Define options and create an index for 'title' field
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"title": 1})
            .options(options)
            .build();

        // Create index for 'title' field in the note collection
        self.note_collection
            .create_index(index, None)
            .await
            .expect("error creating index!");

        // Prepare document with dates and merge 'document'
        let datetime = Utc::now();
        let mut doc_with_dates = doc! {"createdAt": datetime, "updatedAt": datetime, "published": published, "category": category};
        doc_with_dates.extend(document.clone());

        // Insert the new document into the collection
        let insert_result = self
            .collection
            .insert_one(&doc_with_dates, None)
            .await
            .map_err(|e| {
                if e.to_string()
                    .contains("E11000 duplicate key error collection")
                {
                    return MongoDuplicateError(e);
                }
                return MongoQueryError(e);
            })?;

        // Retrieve the ID of the inserted document
        let new_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("issue with new _id");

        // Find the newly created note by ID
        let note_doc = self
            .note_collection
            .find_one(doc! {"_id":new_id }, None)
            .await
            .map_err(MongoQueryError)?;

        // Return None if the note document is not found
        if note_doc.is_none() {
            return Ok(None);
        }

        // Prepare and return the response containing the newly created note
        let note_response = SingleNoteResponse {
            status: "success".to_string(),
            data: NoteData {
                note: self.doc_to_note(&note_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(note_response))
    }

    /// Retrieves a note based on the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice representing the ID of the note to be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the retrieval of the note fails due to an invalid ID or a query error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::{DB, CreateNoteSchema};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = DB::init().await?;
    /// let note_id = "6021e59541a3ae69b39ecb46"; // ID of the note to retrieve
    ///
    /// let retrieved_note = db.get_note(note_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_note(&self, id: &str) -> Result<Option<SingleNoteResponse>> {
        // Parse the string ID into an `ObjectId`
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        // Find the note document by its ID
        let note_doc = self
            .note_collection
            .find_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        // Return None if the note document is not found
        if note_doc.is_none() {
            return Ok(None);
        }

        // Prepare and return the response containing the retrieved note
        let note_response = SingleNoteResponse {
            status: "success".to_string(),
            data: NoteData {
                note: self.doc_to_note(&note_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(note_response))
    }

    /// Edits a note based on the provided ID and update schema.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice representing the ID of the note to be edited.
    /// * `body` - An `UpdateNoteSchema` object containing the updated note information.
    ///
    /// # Errors
    ///
    /// Returns an error if editing the note fails due to an invalid ID, serialization error,
    /// or a query error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::{DB, UpdateNoteSchema};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = DB::init().await?;
    /// let note_id = "6021e59541a3ae69b39ecb46"; // ID of the note to edit
    /// let updated_info = UpdateNoteSchema {
    ///     title: Some("Updated Title".to_string()),
    ///     content: Some("Updated Content".to_string()),
    ///     category: None,
    ///     published: None,
    /// };
    ///
    /// let edited_note = db.edit_note(note_id, &updated_info).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_note(
        &self,
        id: &str,
        body: &UpdateNoteSchema,
    ) -> Result<Option<SingleNoteResponse>> {
        // Parse the string ID into an `ObjectId`
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };

        // Prepare options for the find and update operation
        let find_one_and_update_options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        // Serialize the update body to BSON document
        let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
        let document = serialized_data.as_document().unwrap();
        let update = doc! {"$set": document};

        // Find and update the note based on the provided ID and update information
        let note_doc = self
            .note_collection
            .find_one_and_update(query, update, find_one_and_update_options)
            .await
            .map_err(MongoQueryError)?;

        // Return None if the note document is not found
        if note_doc.is_none() {
            return Ok(None);
        }

        // Prepare and return the response containing the updated note
        let note_response = SingleNoteResponse {
            status: "success".to_string(),
            data: NoteData {
                note: self.doc_to_note(&note_doc.unwrap()).unwrap(),
            },
        };

        Ok(Some(note_response))
    }

    /// Deletes a note based on the provided ID.
    ///
    /// # Arguments
    ///
    /// * `id` - A string slice representing the ID of the note to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if deleting the note fails due to an invalid ID or a query error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::DB;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = DB::init().await?;
    /// let note_id = "6021e59541a3ae69b39ecb46"; // ID of the note to delete
    ///
    /// // Delete the note by ID
    /// let deletion_result = db.delete_note(note_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_note(&self, id: &str) -> Result<Option<()>> {
        // Parse the string ID into an `ObjectId`
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        // Delete the note based on the provided ID
        let result = self
            .collection
            .delete_one(doc! {"_id":oid }, None)
            .await
            .map_err(MongoQueryError)?;

        // Return None if the note document is not found
        if result.deleted_count == 0 {
            return Ok(None);
        }

        Ok(Some(()))
    }

    /// Converts a `NoteModel` into a `NoteResponse`.
    ///
    /// # Arguments
    ///
    /// * `note` - A reference to a `NoteModel` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an issue extracting or transforming data from the `NoteModel`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use my_db_handler::{DB, NoteModel};
    /// # async fn example(db: &DB) -> Result<(), Box<dyn std::error::Error>> {
    /// # let note = NoteModel::default(); // Placeholder for a NoteModel instance
    /// let note_response = db.doc_to_note(&note)?;
    /// # Ok(())
    /// # }
    /// ```
    fn doc_to_note(&self, note: &NoteModel) -> Result<NoteResponse> {
        let note_response = NoteResponse {
            id: note.id.to_hex(),
            title: note.title.to_owned(),
            content: note.content.to_owned(),
            category: note.category.to_owned().unwrap_or_default(),
            published: note.published.unwrap_or_default(),
            createdAt: note.createdAt,
            updatedAt: note.updatedAt,
        };

        Ok(note_response)
    }
}
