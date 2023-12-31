mod db;
mod errors;
mod handlers;
mod model;
mod response;
mod schema;

use db::DB;
use dotenv::dotenv;
use schema::FilterOptions;
use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection};

// Define custom Result and WebResult types for handling errors and rejections
type Result<T> = std::result::Result<T, errors::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

// Entry point of the application
#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging configuration if not provided
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    // Initialize logging based on configuration
    pretty_env_logger::init();
    // Load environment variables from a .env file if present
    dotenv().ok();
    // Initialize a connection to the database
    let db = DB::init().await?;

    // Configure Cross-Origin Resource Sharing (CORS) policies
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    // Define routes for different endpoints
    let note_router = warp::path!("api" / "notes");
    let note_router_id = warp::path!("api" / "notes" / String);
    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(handlers::health_checker_handler);

    // Define routes for handling note-related actions
    let note_routes = note_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone())) // Inject database instance into the handler
        .and_then(handlers::create_note_handler)
        .or(note_router
            .and(warp::get())
            .and(warp::query::<FilterOptions>())
            .and(with_db(db.clone()))
            .and_then(handlers::notes_list_handler));

    let note_routes_id = note_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handlers::edit_note_handler)
        .or(note_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handlers::get_note_handler))
        .or(note_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handlers::delete_note_handler));

    // Combine routes, logging, error recovery, and CORS policies
    let routes = note_routes
        .with(warp::log("api")) // Log API requests
        .or(note_routes_id)
        .or(health_checker)
        .with(cors) // Apply CORS policies to routes
        .recover(errors::handle_rejection); // Handle errors and rejections

    // Start the server on port 8080
    println!("Server started successfully on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

// Helper function to inject the database instance into route handlers
fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
