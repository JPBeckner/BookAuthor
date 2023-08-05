use std::process::id;

use crate::domain::entities::Author;
use crate::repositories::author::{AuthorRepository, self};

#[derive(Debug, Clone)]
pub struct AuthorService{
    pub repo: AuthorRepository
}

impl AuthorService {
    // pub fn new(id_author: String, Name: String, last_name: String) -> Author {
    //     Author {
    //         id_author, 
    //         Name, 
    //         last_name
    //     }
    // }

    pub async fn get(&self, id_author: i32) -> Result<Author, String> {
        match self.repo.search(id_author).await {
            Ok(author) => Ok(author),
            Err(_) => Err(String::from("Not Found"))
        }
    }

    pub async fn create(&self, author: Author) -> Result<String, String> {
        let result = self.repo.insert(author).await;
        match result {
            Ok(_) => Ok(String::from("Sucess")),
            Err(_) => Err(String::from("Fail")),
            // _ => todo!("Fail"),
        }
    }
}


