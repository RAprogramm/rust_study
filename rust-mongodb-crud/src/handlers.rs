use crate::{
    db::DB,
    response::GenericResponse,
    schema::UpdateNoteSchema,
    schema::{CreateNoteSchema, FilterOptions},
    WebResult,
};
use warp::{http::StatusCode, reject, reply::json, reply::with_status, Reply};

/// Handles the health check endpoint.
///
/// This function responds with a JSON indicating the status of the API.
/// Returns a `Reply` which is a trait used for generating HTTP responses.
pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    // Serialize the response to JSON and return it as a Reply.
    Ok(json(response_json))
}

/// Handles the retrieval of a list of notes based on the provided options.
///
/// # Arguments
///
/// * `opts` - FilterOptions containing parameters like page number and limit.
/// * `db` - An instance of the DB struct, representing the database connection.
///
/// # Returns
///
/// Returns a Warp Result containing the JSON representation of the NoteListResponse or a rejection if an error occurs.
pub async fn notes_list_handler(opts: FilterOptions, db: DB) -> WebResult<impl Reply> {
    // Extract limit and page from FilterOptions or use default values if not provided
    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    // Fetch notes from the database based on provided options
    let result_json = db
        .fetch_notes(limit, page)
        .await
        .map_err(|e| reject::custom(e))?; // Map errors to a custom rejection

    // Return the JSON representation of the fetched notes
    Ok(json(&result_json))
}

/// Handles the creation of a new note based on the provided schema.
///
/// # Arguments
///
/// * `body` - CreateNoteSchema containing details of the note to be created.
/// * `db` - An instance of the DB struct, representing the database connection.
///
/// # Returns
///
/// Returns a Warp Result containing the JSON representation of the created note or a rejection if an error occurs.
pub async fn create_note_handler(body: CreateNoteSchema, db: DB) -> WebResult<impl Reply> {
    // Create a new note based on the provided schema
    let note = db.create_note(&body).await.map_err(|e| reject::custom(e))?;

    // Return the JSON representation of the created note with a status code indicating successful creation
    Ok(with_status(json(&note), StatusCode::CREATED))
}

/// Handles retrieval of a specific note based on the provided ID.
///
/// # Arguments
///
/// * `id` - String representing the ID of the note to retrieve.
/// * `db` - An instance of the DB struct, representing the database connection.
///
/// # Returns
///
/// Returns a Warp Result containing the JSON representation of the requested note if found,
/// or a JSON response with a 'not found' status if the note does not exist, or a rejection if an error occurs.
pub async fn get_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
    // Retrieve the note based on the provided ID
    let note = db.get_note(&id).await.map_err(|e| reject::custom(e))?;

    // Construct an error response if the note is not found
    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    // Check if the note exists and return the appropriate response
    if note.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    // Return the JSON representation of the retrieved note with a success status
    Ok(with_status(json(&note), StatusCode::OK))
}

/// Handles editing an existing note based on the provided ID and request body.
///
/// # Arguments
///
/// * `id` - String representing the ID of the note to edit.
/// * `body` - An instance of UpdateNoteSchema containing the updated note details.
/// * `db` - An instance of the DB struct, representing the database connection.
///
/// # Returns
///
/// Returns a Warp Result containing the JSON representation of the edited note if successful,
/// or a JSON response with a 'not found' status if the note does not exist, or a rejection if an error occurs.
pub async fn edit_note_handler(
    id: String,
    body: UpdateNoteSchema,
    db: DB,
) -> WebResult<impl Reply> {
    // Edit the note based on the provided ID and request body
    let note = db
        .edit_note(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;

    // Construct an error response if the note is not found
    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    // Check if the note exists and return the appropriate response
    if note.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    // Return the JSON representation of the edited note with a success status
    Ok(with_status(json(&note), StatusCode::OK))
}

/// Handles the deletion of a note based on the provided ID.
///
/// # Arguments
///
/// * `id` - String representing the ID of the note to delete.
/// * `db` - An instance of the DB struct, representing the database connection.
///
/// # Returns
///
/// Returns a Warp Result containing a 'no content' response if the note is successfully deleted,
/// or a JSON response with a 'not found' status if the note does not exist, or a rejection if an error occurs.
pub async fn delete_note_handler(id: String, db: DB) -> WebResult<impl Reply> {
    // Delete the note based on the provided ID
    let result = db.delete_note(&id).await.map_err(|e| reject::custom(e))?;

    // Construct an error response if the note is not found
    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    // Check if the note exists and return the appropriate response
    if result.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    // Return a 'no content' response indicating successful deletion
    Ok(with_status(json(&""), StatusCode::NO_CONTENT))
}
