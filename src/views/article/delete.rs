/// path: -> my_articles/{id}/delete
/// It retrieves the unique id of the article to be edited from the url

use crate::diesel;
use diesel::prelude::*;

use actix_web::{HttpResponse, HttpRequest};

use crate::database::establish_connection;
use crate::models::article::article::Article;
use crate::schema::article;

use crate::auth::jwt::JwtToken;

use super::utils::return_articles;


pub async fn delete(req: HttpRequest) -> HttpResponse {
    
    let article_id: String = req.match_info()
                                .get("unique_id").unwrap().to_string();

    let token: JwtToken = JwtToken::decode_from_request(req)
                                        .unwrap();
    
    let connection = establish_connection();

    let articles = article::table
        .filter(article::columns::unique_id.eq(&article_id))
        .filter(article::columns::blogger_id.eq(
            &token.blogger_id))
        .order(article::columns::id.asc())
        .load::<Article>(&connection)
        .unwrap();

        let _ = diesel::delete(&articles[0]).execute(&connection);

        return HttpResponse::Ok().json(return_articles(&token.blogger_id))
}
