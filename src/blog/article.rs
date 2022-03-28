extern crate uuid;
use uuid::Uuid;


pub struct BlogArticle {
    pub title: String,
    pub description: String,
    pub content: String,
    pub unique_id: String
}


impl BlogArticle {
    pub fn new(title:&str, describe: &str, input: &str) -> BlogArticle {

        let id: String = Uuid::new_v4().to_string();

        return BlogArticle {
            title: title.to_string(),
            description: describe.to_string(),
            content: input.to_string(),
            unique_id: id
        }
    }
}


#[cfg(test)]
mod article_test {
    use super::BlogArticle;

    #[test]
    fn new() {
        let content: String = String::from("bitcoin is the future");
        let title: String = String::from("bitcoin");
        let description: String = String::from("btc");
        
        let expected_content: String = String::from("bitcoin is the future");
        let expected_title: String = String::from("bitcoin");
        let expected_description: String = String::from("btc");

        let article: BlogArticle = BlogArticle::new(&title, &description, &content);

        assert_eq!(article.content, expected_content);
        assert_eq!(article.title, expected_title);
        assert_eq!(article.description, expected_description);
    }
}