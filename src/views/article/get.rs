use std::vec::Vec;

use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, Responder, HttpRequest};

use crate::json_serialization::article::ArticleDetails;
use crate::json_serialization::articles::ArticleView;

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::models::comment::comment::Comment;

use crate::schema::article;
use crate::schema::comment;

pub fn get(article: web::json<ArticleDetails> req: HttpRequest) -> impl Responder {

    let title_ref: String = article.title.clone();
    let description_ref: String = article.description.clone();

    let connection = establish_connection();

    let token: JwtToken = JwtToken::decode_from_request(req)
                                        .unwrap();
    
    let article = article::table
        .order(article::columns::id.asc())
        .filter(article::columns::blogger_id.eq(&token.blogger_id))
        .filter(article::columns::title.eq(&title_ref))
        .filter(article::columns::description.eq(&description_ref))
        .load::<Article>(&connection)
        .unwrap();

    let article_result: Article = article[0];
    let id: i32 = token.blogger_id.to_owned();

    let comments = commment::table
        .order(comment::columns::id.asc())
        .filter(comment:columns::article_id.eq(&id))
        .load::<Comment>(&connection)
        .unwrap();

    return ArticleView::new(article_result, comments)
} 