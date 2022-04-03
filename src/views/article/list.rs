/// path: -> /my_articles
/// this page returns json format of all articles owned by a blogger as a
/// Responder to be passed into a view.
  

use actix_web::{Responder, HttpRequest};
use super::utils::return_user_articles;
use super::utils::return_all_articles;

use crate::auth::jwt::JwtToken;


pub async fn get_user_articles(req: HttpRequest) -> impl Responder {
    let token: JwtToken = 
        JwtToken::decode_from_request(req).unwrap();
    return return_user_articles(&token.blogger_id);
}

pub async fn get_all_articles() -> impl Responder {
    return_all_articles()
}
