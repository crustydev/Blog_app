pub mod article;
pub mod comment;

use article::BlogArticle;
use comment::BlogComment;


pub fn comment_factory(input: &str) -> 
    Result<BlogComment, &'static str> {
        let new_comment = BlogComment::new(input);
        Ok(new_comment)
}

pub fn article_factory(title: &str,  description: &str, input: &str) 
    -> Result<BlogArticle, &'static str> {
        let new_article = BlogArticle::new(title, description, input);
        Ok(new_article)
}
