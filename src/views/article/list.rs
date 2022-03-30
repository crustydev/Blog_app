/// path: -> /my_articles
/// this page returns json format of all articles owned by a blogger as a
/// Responder to be passed into a view.
  

use actix_web::{Responder, HttpRequest};
use super::utils::return_articles;

use crate::auth::jwt::JwtToken;


pub async fn list_articles(req: HttpRequest) -> impl Responder {
    let token: JwtToken = 
        JwtToken::decode_from_request(req).unwrap();
    return return_articles(&token.blogger_id);
}
