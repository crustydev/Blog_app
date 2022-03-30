/// 
/// path:-> /my_articles/create
/// 
/// This allows the active blogger to create an article that belongs to him.
/// It uses the JwtToken to retrieve the blogger_id to make sure that a blogger
/// cannot create an article for another account that is not currently signed in.
/// 
/// It takes a serialized struct sent as json containing the details filled 
/// in by the writer from the frontend. After this, it redirects back to 
/// /articles with return_state() which contains all the writer's articles
/// 



use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpRequest, Responder};

use crate::json_serialization::article::ArticleJson;
use super::utils::return_articles;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::models::article::new_article::NewArticle;

use crate::schema::article;
use crate::auth::jwt::JwtToken;


pub async fn create(req: HttpRequest, article:
                     web::Json<ArticleJson> ) -> impl Responder {

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    let connection = establish_connection();

    let title: String = article.title.clone();
    let description: String = article.description.clone();
    let content: String = article.content.clone();
    let blogger_id: i32 = token.blogger_id.clone();

    let articles = article::table
        .filter(article::columns::blogger_id.eq(&token.blogger_id))
        .filter(article::columns::title.eq(&title))
        .filter(article::columns::description.eq(&description))
        .order(article::columns::id.asc())
        .load::<Article>(&connection)
        .unwrap();

    if articles.len() == 0 {
        let new_article = NewArticle::new(title, description, content, blogger_id);
        let _ = diesel::insert_into(article::table)
            .values(&new_article)
            .execute(&connection);
    }

    return return_articles(&token.blogger_id);
}