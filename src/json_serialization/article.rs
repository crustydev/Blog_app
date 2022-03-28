use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleJson {
    pub title: String,
    pub description: String,
    pub content: String
}


#[derive(Deserialize)]
pub struct ArticleDetails {
    pub title: String,
    pub description: String
}

