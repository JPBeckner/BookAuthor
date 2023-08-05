use crate::domain::author_service::AuthorService;
use crate::domain::entities::Author;

use axum::{response::Json, extract::State};
use axum::extract::Json as eJson;

use std::sync::Arc;
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub struct AuthorController {
    pub author_service: AuthorService,
}

pub async fn get_author(State(state): State<Arc<AuthorService>>, eJson(payload): eJson<serde_json::Value>) -> Json<Value> {
    let find_author = Author { 
        id_author: payload.get("id_author").unwrap_or(&json!(0)).as_i64().unwrap() as i32, 
        name: payload.get("name").unwrap_or(&json!("")).as_str().unwrap().to_string(), 
        last_name: payload.get("last_name").unwrap_or(&json!("")).as_str().unwrap().to_string() 
    };
    let author = state.get(find_author).await;
    match author {
        Ok(author) => {
            Json(json!(author))
        },
        Err(_) => Json(json!({"message": "Author Not Found"}))
    }
}

pub async fn post_author(State(state): State<Arc<AuthorService>>, eJson(payload): eJson<serde_json::Value>) -> Json<Value> {
    let author = Author { 
        id_author: payload.get("id_author").unwrap().as_i64().unwrap() as i32, 
        name: payload.get("name").unwrap().as_str().unwrap().to_string(), 
        last_name: payload.get("last_name").unwrap().as_str().unwrap().to_string() 
    };
    let result = state.create(author).await;
    match result {
        Ok(message) => Json(json!({"message": message})),
        Err(message) => Json(json!({"message": message}))
    }
}

// pub async fn put_author() {
    
// }

// pub async fn delete_author() {

// }

