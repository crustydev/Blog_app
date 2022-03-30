use crate::schema::comment;
use uuid::Uuid;


#[derive(Insertable, Clone, Debug)]
#[table_name="comment"]
pub struct NewComment {
    pub content: String,
    pub unique_id: String,
    pub blogger_id: i32,
    pub article_id: i32
}


impl NewComment {
    pub fn new(content: String, blogger_id: i32, article_id: i32)
                         -> NewComment {
        let unique_id: String = Uuid::new_v4().to_string();
        return NewComment { content, unique_id, blogger_id, article_id }
    }
}


#[cfg(test)]
mod comment_test {
    use super::NewComment;

    #[test]
    fn new() {
        let content: String = String::from("rashford and maguire");
        let blogger_id: i32 = 1;
        let article_id: i32 = 2;

        let expected_content: String = String::from("rashford and maguire");
        let expected_blogger: i32 = 1;
        let expected_article: i32 = 2;

        let comment: NewComment = NewComment::new(content, blogger_id, article_id);

        assert_eq!(comment.content, expected_content);
        assert_eq!(comment.blogger_id, expected_blogger);
        assert_eq!(comment.article_id, expected_article);
    }
}
