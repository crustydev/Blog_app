use crate::models::article::article::Article;
use crate::models::comment::comment::Comment;

use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde::Serialize;

use futures::future::{ready, Ready};


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