mod error;
mod notes;

use axum::routing::get;
use axum::Router;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("DATABASE_URL")?;
    let db = PgPoolOptions::new().max_connections(5).connect(&url).await?;
    sqlx::migrate!().run(&db).await?;
    println!("migrations applied");

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/notes", get(notes::list).post(notes::create))
        .route("/notes/{id}", get(notes::get).put(notes::update).delete(notes::delete))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
