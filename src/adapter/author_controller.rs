use crate::{domain::author_service::AuthorService, repositories::author};
use crate::domain::entities::Author;

use axum::{response::Json, extract::State};
use axum::extract::{Path, Query, Json as eJson};

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
        Err(_) => Json(json!({"message": "Author Not Found"}))
    }
    // println!("{}", author);
    
    // return author.id_author;
}

pub async fn post_author(State(state): State<Arc<AuthorService>>, eJson(payload): eJson<serde_json::Value>) -> Json<Value> {
    let author = Author { id_author: payload.get("id_author").unwrap().as_i64().unwrap() as i32, name: payload.get("name").unwrap().as_str().unwrap().to_string(), last_name: payload.get("last_name").unwrap().as_str().unwrap().to_string() };
    let result = state.create(author).await;
    match result {
        Ok(message) => Json(json!({"message": message})),
        Err(message) => Json(json!({"message": message}))
    }
}


impl AuthorController{
    
    
    
    pub async fn put_author() {
    
    }
    
    pub async fn delete_author() {
    
    }
    
}
