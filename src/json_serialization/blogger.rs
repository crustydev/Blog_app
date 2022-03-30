
use crate::models::blogger::blogger::Blogger;

use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

use futures::future::ready;
use futures::future::Ready;


#[derive(Deserialize)]
pub struct NewBloggerJson {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct BloggerDetails {
    username: String,
    email: String
}


impl BloggerDetails {
    pub fn new(blogger: Blogger) -> BloggerDetails {
        let username: String = blogger.username;
        let email: String = blogger.email;

        return BloggerDetails {username, email}
    }
}


impl Responder for BloggerDetails {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(body)))
    }
}