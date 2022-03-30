use crate::schema::reply;

use super::super::comment::comment::Comment;
use super::super::blogger::blogger::Blogger;

#[derive(Identifiable, Queryable, Associations, Clone, Debug)]
#[belongs_to(Blogger)]
#[belongs_to(Comment)]
#[table_name="reply"]
pub struct Reply {
    pub id: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub blogger_id: i32,
    pub comment_id: i32
}

