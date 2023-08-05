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
    // build our application with a single route
    let author_repo = repositories::author::AuthorRepository::new().await;
    let author_service = domain::author_service::AuthorService{ repo: author_repo.clone()};
    let author_controller = adapter::author_controller::AuthorController{ author_service: author_service };

    // let get_author = move || async move { 
    //     match author_controller.get_author().await {
    //         response => response,
    //         // _ => "error".to_owned(),
    //     }
    // };
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