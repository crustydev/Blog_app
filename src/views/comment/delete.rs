/// path:-> /comment/{comment_unique_id}/delete

use crate::diesel;
use diesel::prelude::*;


use actix_web::{HttpRequest, HttpResponse};

use crate::database::establish_connection;
use crate::models::comment::comment::Comment;

use crate::schema::comment;


pub async fn delete(req: HttpRequest) -> HttpResponse {
    let comment_id: String = req.match_info().get("comment_unique_id")
            .unwrap().to_string();
    
    let connection = establish_connection();

    let comments = comment::table
        .filter(comment::columns::unique_id.eq(&comment_id))
        .load::<Comment>(&connection)
        .unwrap();
    
    let _ = diesel::delete(&comments[0]).execute(&connection);
    return HttpResponse::Ok().await.unwrap()
}