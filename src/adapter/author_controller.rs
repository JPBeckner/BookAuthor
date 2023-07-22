use crate::domain::author_service::AuthorService;
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

impl AuthorController{
    pub async fn get_author(&self) -> String {
        let author = self.author_service.get(1).await;
        match author {
            Ok(author) => format!("{} {}", author.name, author.last_name),
            Err(e) => e
        }
        // println!("{}", author);
        
        // return author.id_author;
    }
    
    pub async fn post_author() {
    
    }
    
    pub async fn put_author() {
    
    }
    
    pub async fn delete_author() {
    
    }
    
}
