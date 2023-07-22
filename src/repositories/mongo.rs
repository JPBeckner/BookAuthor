use mongodb::{Client, options::ClientOptions, Collection, bson::Document, Database};
use crate::domain::entities::{Author, Book};

pub struct MongoDB {}

impl MongoDB {
    pub async fn get_connection(&self) -> Database {
        // Parse a connection string into an options struct.
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await;
        // Manually set an option.
        // client_options.app_name = Some("bookauthor".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options.unwrap());

        let db = client.unwrap().database("bookauthor");
        return db;
    }

    pub async fn book(&self) -> Collection<Book> {
        let db = self.get_connection().await;
        let collection: Collection<Book> = db.collection::<Book>("books");
        return collection;
    }

    pub async fn author(&self) -> Collection<Author> {
        let db = self.get_connection().await;
        let collection: Collection<Author> = db.collection::<Author>("author");
        return collection;
    }
}



