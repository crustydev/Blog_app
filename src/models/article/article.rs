use chrono::NaiveDateTime;

use actix_web::{Responder, HttpRequest, HttpResponse, Error};
use serde::Serialize;

use crate::schema::article;
use crate::models::blogger::blogger::Blogger;

use futures::future::{ready, Ready};

#[derive(Serialize)]
#[derive(Queryable, Identifiable, Associations, Clone, Debug)]
#[belongs_to(Blogger)]
#[table_name="article"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub blogger_id: i32
}

impl Article {

}

impl Responder for Article {
    type Error = Error;

    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)))
    }
}