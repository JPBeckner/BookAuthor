
use crate::domain::entities::Author;
use crate::repositories::author::AuthorRepository;

#[derive(Debug, Clone)]
pub struct AuthorService{
    pub repo: AuthorRepository
}

impl AuthorService {

    pub async fn get(&self, find_author: Author) -> Result<Author, String> {
        match self.repo.search(find_author).await {
            Ok(author) => Ok(author),
            Err(_) => Err(String::from("Not Found"))
        }
    }

    pub async fn create(&self, author: Author) -> Result<String, String> {
        let result = self.repo.insert(author).await;
        match result {
            Ok(_) => Ok(String::from("Sucess")),
            Err(_) => Err(String::from("Fail")),
        }
    }
}


