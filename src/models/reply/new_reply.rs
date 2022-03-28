use crate::schema::reply;


#[derive(Insertable, Clone, Debug)]
#[table_name="reply"]
pub struct NewReply {
    pub content: String,
    pub blogger_id: i32,
    pub comment_id: i32
}


impl NewReply {
    pub fn new(content: String, blogger_id: i32, comment_id: i32) -> NewReply {
        return NewReply { content, blogger_id, comment_id }
    }
}


#[cfg(test)]
mod reply_tests {
    use super::NewReply;

    #[test]
    fn new() {
        let content: String = String::from("reply");
        let blogger_id: i32 = 1;
        let comment_id: i32 = 2;

        let expected_content: String = String::from("reply");
        let expected_blogger: i32 = 1;
        let expected_comment: i32 = 2;
        
        let reply = NewReply::new(content, blogger_id, comment_id);
        assert_eq!(reply.content, expected_content);
        assert_eq!(reply.blogger_id, expected_blogger);
        assert_eq!(reply.comment_id, expected_comment);
    }
}