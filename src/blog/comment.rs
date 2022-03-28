extern crate uuid;
use uuid::Uuid;


pub struct BlogComment {
    pub content: String,
    pub unique_id: String
}


impl BlogComment {
    pub fn new(input: &str) -> BlogComment {
        let id: String = Uuid::new_v4().to_string();

        return BlogComment {
            content: input.to_string(),
            unique_id: id
        }
    }
}


#[cfg(test)]
mod comment_test {
    use super::BlogComment;

    #[test]
    fn new() {
        let content: String = String::from("rashford and maguire");
        let expected_content: String = String::from("rashford and maguire");

        let comment: BlogComment = BlogComment::new(&content);

        assert_eq!(comment.content, expected_content);
    }
}
