mod structs;

pub enum PostTypes {
    Comment(Comment),
    Article(Article)
}

pub fn comment_factory(user: &User, input: &String,
         tag: &Post) -> Result<PostTypes::Comment, &'static str> {
    let new_comment = Comment::new(user, input, tag);
    Ok(PostTypes::Comment(new_comment))
}
 

pub fn article_factory(user: &User, input: &String, title: &String, desc: &String,
         category: &Category) -> Result<PostTypes::Article, &'static str> {
    let new_article = Article::new(user, input, title, desc, category);
    Ok(PostTypes::Article(new_article))
}