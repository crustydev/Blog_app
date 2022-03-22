const ARTICLE_MAX_CHAR: i32 = 2000

use super::post::Post;
use super::category::Category;

pub struct Article {
    pub super_struct: Post, 
    pub title: String,
    pub description: String,
    pub category: Category,
    pub max_char: i32
}


impl Article {
    pub fn new(user: &User, input: &String, title: &String,
         desc: &String, category: &Category) -> Article {
         let post: Post = Post::new(user, input);

         return Article {
             super_struct: post,
             title: title,
             description: desc,
             category: category,
             max_char: ARTICLE_MAX_CHAR
         }   
    }
}