use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommentJson {
    pub article_unique_id: String,
    pub content: String
}


