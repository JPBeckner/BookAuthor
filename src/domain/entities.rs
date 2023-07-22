use std::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Author {
    pub id_author: i32,
    pub name: String,
    pub last_name: String,
}

impl Display for Author {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // use write! macro just like println! macro, but output gets writen to
        // the formatter struct.
        write!(f, "{} {}<{}>", self.name, self.last_name, self.id_author)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id_book: i32,
    pub title: String,
    pub author: i32,
    pub genres: Vec<String>
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // use write! macro just like println! macro, but output gets writen to
        // the formatter struct.
        write!(f, "{} {}<{}>", self.title, self.author, self.id_book)
    }
}