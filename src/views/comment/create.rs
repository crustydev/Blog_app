/// Path:-> /articles/{article_unique_id}/create_comment
/// content, blogger_id, article_id
/// 
use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpRequest, HttpResponse};
use crate::json_serialization::comment::CommentJson;

use crate::database::establish_connection;
use crate::schema::comment;
use crate::schema::article;

use crate::models::comment::new_comment::NewComment;
use crate::models::article::article::Article;

use crate::auth::jwt::JwtToken;



pub async fn create(req: HttpRequest, comment: 
                        web::Json<CommentJson>) -> HttpResponse {

    let token: JwtToken = JwtToken::decode_from_request(req.clone()).unwrap();
    let content: String = comment.content.clone();
    let article_unique_id: String = comment.article_unique_id.clone();

    let connection = establish_connection();

    let article = article::table
        .filter(article::columns::unique_id.eq(&article_unique_id))
        .load::<Article>(&connection)
        .unwrap();

    if article.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    }

    let article_id: i32 = article[0].id;
    let new_comment = NewComment::new(content, token.blogger_id,
                         article_id);
                         
    let _ = diesel::insert_into(comment::table)
            .values(&new_comment)
            .execute(&connection);

    return HttpResponse::Ok().await.unwrap();
}