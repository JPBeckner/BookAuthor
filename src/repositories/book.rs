use crate::domain::entities::Book;
use crate::repositories::mongo::MongoDB;
use mongodb::{Collection, bson::doc, options::FindOptions};

struct BookRepository {
    connection: Collection<Book>
}

impl BookRepository {
    pub async fn new() -> Self {
        BookRepository{ connection: MongoDB{}.book().await}
    } 

    pub async fn insert(&self, book: Book) {
        self.connection.insert_one(book, None);
    }

    pub async fn search(&self, id: i32) {
        let filter = doc! { "id_book": id };
        // let find_options = FindOptions::builder().sort(doc! { "title": 1 }).build();
        let mut cursor = self.connection.find_one(filter, None).await;
        match cursor {
            Ok(cursor) => {
                cursor.unwrap();
            },
            Err(_) => println!("Not Found")
        }
    }
}