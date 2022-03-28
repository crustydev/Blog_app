use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use crate::database::establish_connection;
use crate::json_serialization::article::ArticleJson;
use crate::models::article::article::Article;
use crate::schema::article;

use super::utils::return_article;
use crate::auth::jwt::JwtToken;


pub async fn delete(article: web::Json<ArticleJson>,
                     req: HttpRequest) -> HttpResponse {
    
    let title_ref: String = article.title.clone();
    let description: String = article.description.clone();

    let token: JwtToken = JwtToken::decode_from_request(req)
                                        .unwrap();
    
    let connection = establish_connection();

    let articles = article::table
        .filter(article::columns::title.eq(
            title_ref.as_str()))
        .filter(article::columns::description.eq(
            description.as_str()))
        .filter(article::columns::blogger_id.eq(
            &token.blogger_id))
        .order(article::columns::id.asc())
        .load::<Article>(&connection)
        .unwrap();

        let _ = diesel::delete(&articles[0]).execute(&connection);
        return HttpResponse::Ok().json(return_state(&token.blogger_id))
}
