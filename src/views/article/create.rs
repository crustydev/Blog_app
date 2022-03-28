use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpRequest;
use actix_web::HttpReponse;

use crate::json_serialization::article::ArticleJson;
use super::utils::return_article;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::models::article::new_article::NewArticle;

use crate::schema::article;



pub async fn create(req: HttpRequest, article:
                     web::Json<ArticleJson> ) -> impl Responder {

    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    let connection = establish_connection();

    let title: String = article.title;
    let description: String = article.description;
    let content: String = article.content;
    let blogger_id: String = token.blogger_id;

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

    return return_article(&token.blogger_id)
    
}