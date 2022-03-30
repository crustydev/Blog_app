/// path: -> my_articles/{unique_id} 
/// we retrieve the unique_id from the url to retrieve the article from 
/// our database and return its corresponding ArticleView as a responder
/// which returns our article and its comments in json format.


use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpResponse, HttpRequest};

use crate::json_serialization::article::ArticleView;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::models::comment::comment::Comment;

use crate::schema::article;
use crate::schema::comment;

use crate::auth::jwt::JwtToken;

pub async fn writer_article_view(req: HttpRequest) -> HttpResponse {

    let article_id: String = req.match_info()
            .get("unique_id").unwrap().to_string();

    let connection = establish_connection();

    let token: JwtToken = JwtToken::decode_from_request(req)
                                        .unwrap();
    
    let article = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::blogger_id.eq(&token.blogger_id))
        .filter(article::columns::unique_id.eq(&article_id))
        .load::<Article>(&connection)
        .unwrap();

    
    let article_result: Article = article[0].clone();
    let comments_article_id: i32 = article_result.id;
    

    let comments = comment::table
        .order(comment::columns::id.asc())
        .filter(comment::columns::article_id.eq(&comments_article_id))
        .load::<Comment>(&connection)
        .unwrap();

    let result = ArticleView::new(article_result, comments);

    return HttpResponse::Ok().json(result);
} 

/// path:-> /articles/{unique_id}
pub async fn reader_article_view(req: HttpRequest) -> HttpResponse {
    let article_id: String = req.match_info()
            .get("unique_id").unwrap().to_string();

    let connection = establish_connection();
    
    let article = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::unique_id.eq(&article_id))
        .load::<Article>(&connection)
        .unwrap();

    let article_result: Article = article[0].clone();
    let comments_article_id: i32 = article_result.id;

    let comments = comment::table
        .order(comment::columns::id.asc())
        .filter(comment::columns::article_id.eq(&comments_article_id))
        .load::<Comment>(&connection)
        .unwrap();

    let result = ArticleView::new(article_result, comments.clone());

    return HttpResponse::Ok().json(result);
}
