use serde::Serialize;

use crate::schema::comment;

use super::super::blogger::blogger::Blogger;
use super::super::article::article::Article;


#[derive(Serialize, Identifiable, Queryable, Associations, Clone, Debug)]
#[belongs_to(Article)]
#[belongs_to(Blogger)]
#[table_name="comment"]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub unique_id: String,
    pub blogger_id: i32,
    pub article_id: i32
}


