use crate::domain::entities::Author;
use crate::repositories::mongo::MongoDB;
use mongodb::{Collection, bson::{doc, Document}, error::Error};


#[derive(Debug, Clone)]
pub struct AuthorRepository {
    connection: Collection<Author>
}

impl AuthorRepository {
    pub async fn new() -> Self {
        AuthorRepository{ connection: MongoDB{}.author().await}
    } 

    pub async fn insert(&self, author: Author) -> Result<i32, bool> {
        let result = self.connection.insert_one(&author, None).await;
        match result {
            Ok(_) => Ok(author.id_author),
            Err(_) => Err(false),
        }
    }

    pub async fn search(&self, find_author: Author) -> Result<Author, Error> {

        let mut filter = Document::from(doc! {});
        if find_author.id_author > 0 {
            filter.insert("id_author", find_author.id_author);
        }
        if !find_author.name.is_empty() {
            filter.insert("name", find_author.name);
        }
        if !find_author.last_name.is_empty() {
            filter.insert("last_name", find_author.last_name);
        }

        let cursor = self.connection.find_one(filter, None).await;
        
        match cursor {
            Ok(cursor) => Ok(cursor.unwrap()),
            Err(err) => Err(err),
        }
    }
}