use crate::domain::entities::Author;
use crate::repositories::mongo::MongoDB;
use mongodb::{Collection, bson::doc, options::FindOptions, error::Error};


#[derive(Debug, Clone)]
pub struct AuthorRepository {
    connection: Collection<Author>
}

impl AuthorRepository {
    pub async fn new() -> Self {
        AuthorRepository{ connection: MongoDB{}.author().await}
    } 

    pub async fn insert(&self, author: Author) {
        self.connection.insert_one(author, None);
    }

    pub async fn search(&self, id: i32) -> Result<Author, Error> {
        let filter = doc! { "id_author": id };
        // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let cursor = self.connection.find_one(filter, None).await;
        match cursor {
            Ok(cursor) => Ok(cursor.unwrap()),
            Err(err) => Err(err),
        }
    }
}