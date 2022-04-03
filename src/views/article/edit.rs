/// path: -> /myarticles/{unique_id}/edit
/// it receives a serialized json struct containing the new article's details
/// and retrieves the unique id of the article to be edited from the request url.



use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_user_articles;

use crate::database::establish_connection;
use crate::json_serialization::article::ArticleJson;
use crate::schema::article;

use crate::auth::jwt::JwtToken;


pub async fn edit(new_article: web::Json<ArticleJson>, req: HttpRequest) -> HttpResponse {

    let article_id: String = req.match_info().get("unique_id").unwrap().to_string();

    let new_title: String = new_article.title.clone();
    let new_description: String = new_article.description.clone();
    let new_content: String = new_article.content.clone();

    let token: JwtToken = JwtToken::
        decode_from_request(req).unwrap(); 

    let connection = establish_connection();

    let results = article::table
        .filter(article::columns::unique_id.eq(&article_id))
        .filter(article::columns::blogger_id.eq(&token.blogger_id));

    let _ = diesel::update(results)
        .set((article::columns::title.eq(new_title),
                (article::columns::description.eq(new_description)),
                    article::columns::content.eq(new_content)))
        .execute(&connection);
    

    return HttpResponse::Ok().json(return_user_articles(&token.blogger_id))
    
}