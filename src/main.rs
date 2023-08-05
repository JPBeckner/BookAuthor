mod adapter;
mod domain;
mod repositories;

use std::sync::Arc;

use axum::{
    routing::get,
    Router, 
    // extract::State,
};

#[tokio::main]
async fn main() {
    let author_repo = repositories::author::AuthorRepository::new().await;

    let repo_state = Arc::new(domain::author_service::AuthorService{ repo: author_repo});


    let app = Router::new()
    .route("/", get(|| async { "system up!" }))
    .route("/healthcheck", get(|| async { "system up!" }))
    .route("/author", get(adapter::author_controller::get_author).post(adapter::author_controller::post_author))
    .with_state(repo_state);

    println!("\nServer up on http://0.0.0.0:3000");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}