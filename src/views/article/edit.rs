use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_article;

use crate::database::establish_connection;
use crate::json_serialization::article::ArticleJson;
use crate::schema::article;

use crate::auth::jwt::JwtToken;


pub async fn edit(article: web::Json<ArticleJson>, new_article:
     web::Json<ArticleJson>, req: HttpRequest) -> HttpResponse {

    let title_ref: String = article.title.clone();
    let description_ref: String = article.description.clone();
    let content_ref: String = article.content.clone();

    let new_title: String = new_article.title.clone();
    let new_description: String = new_article.description.clone();
    let new_content: String = new_article.content.clone();

    let token: JwtToken = JwtToken::
        decode_from_request(req).unwrap(); 

    let connection = establish_connection();

    let results = article::table
        .filter(article::columns::title.eq(title_ref))
        .filter(article::columns::description.eq(description_ref))
        .filter(article::columns::blogger_id.eq(&token.blogger_id));

    let _ = diesel::update(results)
        .set(article::columns::title.eq(new_title))
        .set(article::columns::description.eq(new_description))
        .set(article::columns::content.eq(new_content))
        .execute(&connection);
    
    return HttpResponse::Ok().json(return_article(&token.blogger_id))
    
}