use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewBloggerJson {
    pub username: String,
    pub email: String,
    pub password: String
}