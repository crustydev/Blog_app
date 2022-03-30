/// path:-> /comment/{comment_unique_id}/edit
/// 
use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpRequest, HttpResponse};
use crate::json_serialization::comment::CommentJson;

use crate::database::establish_connection;
use crate::schema::comment;

pub async fn edit(req: HttpRequest, new: web::Json<CommentJson>)
                         -> HttpResponse {
    let comment_id: String = req.match_info().get("comment_unique_id")
                            .unwrap().to_string();
    
    let new_content: String  = new.content.clone();

    let connection = establish_connection();

    let comments = comment::table
        .filter(comment::columns::unique_id.eq(&comment_id));

    let _ = diesel::update(comments)
        .set(comment::columns::content.eq(&new_content))
        .execute(&connection);
    
    return HttpResponse::Ok().await.unwrap();
}