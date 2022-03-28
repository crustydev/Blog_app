use crate::schema::article;


#[derive(Insertable, Clone, Debug)]
#[table_name="article"]
pub struct NewArticle {
    pub title: String,
    pub description: String,
    pub content: String,
    pub blogger_id: i32
}


impl NewArticle {
    pub fn new(title: String, description: String, content: String, blogger_id: i32) -> NewArticle {

        return NewArticle { title, description, content, blogger_id }
    }
}


#[cfg(test)]
mod article_test {
    use super::NewArticle;

    #[test]
    fn new() {
        let title = String::from("bitcoin");
        let description = String::from("btc");
        let content = String::from("bitcoin is the future");
        let id: i32 = 1;
        
        let expected_title: String = String::from("bitcoin");
        let expected_description: String = String::from("btc");
        let expected_content: String = String::from("bitcoin is the future");
        let expected_id: i32 = 1;
        

        let article: NewArticle = NewArticle::new(title, description, content, id);

        assert_eq!(article.content, expected_content);
        assert_eq!(article.title, expected_title);
        assert_eq!(article.description, expected_description);
        assert_eq!(article.blogger_id, expected_id);
    }
}