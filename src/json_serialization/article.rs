use serde::{Serialize, Deserialize};

use crate::models::article::article::Article;
use crate::models::comment::comment::Comment;

use actix_web::{Error, Responder, HttpRequest, HttpResponse};

use futures::future::{ready, Ready};

#[derive(Deserialize)]
pub struct ArticleJson {
    pub title: String,
    pub description: String,
    pub content: String
}


#[derive(Deserialize)]
pub struct LightArticle {
    pub unique_id: String,
    pub title: String, 
    pub description: String
}


#[derive(Serialize)]
pub struct ArticleView {
    pub article: Article,  
    pub comments: Vec<Comment>
}


impl ArticleView {
    pub fn new(article: Article, comments:
                        Vec<Comment>) -> ArticleView {
        return ArticleView{ article, comments }
    }
}


impl Responder for ArticleView {
    type Error = Error;

    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)))
    }
}

