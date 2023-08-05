use crate::domain::author_service::AuthorService;
use crate::domain::entities::Author;

use axum::{response::Json, extract::State};
use std::sync::Arc;
use serde_json::{Value, json};

// use crate::domain::entities::Author;
// use axum::extract::Json;
// use std::error::Error;
// use closure::Clone;
#[derive(Debug, Clone)]
pub struct AuthorController {
    pub author_service: AuthorService,
}

// impl Clone for AuthorController{
//     fn clone(&self) -> Self {
//         return &self
//     }
// }
pub async fn get_author(State(state): State<Arc<AuthorService>>,) -> Json<Value> {
    let author = state.get(1).await;
    match author {
        Ok(author) => {
            // Json(Author { id_author: 1, name: String::from("asd"), last_name: String::from("asd") })
            // TODO: CONVERT AUTHOR INTO JSON
            Json(json!(author))
            // serde_json::to_string(&author).unwrap();
        },
        Err(e) => Json(json!({"message": "Author Not Found"}))
    }
    // println!("{}", author);
    
    // return author.id_author;
}


impl AuthorController{
    
    pub async fn post_author() {
    
    }
    
    pub async fn put_author() {
    
    }
    
    pub async fn delete_author() {
    
    }
    
}
