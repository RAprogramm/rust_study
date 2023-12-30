mod errors;
mod response;

use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};

/// Type alias for handling web-related results.
type WebResult<T> = std::result::Result<T, Rejection>;

/// Represents a generic response structure for the API.
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,  // Status of the response.
    pub message: String, // Message in the response.
}

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

#[tokio::main]
async fn main() {
    // Initialization and server setup code here.
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    // Define the health checker route.
    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);

    // Attach logging and serve the defined routes.
    let routes = health_checker.with(warp::log("api"));

    println!("Server started successfully!");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
